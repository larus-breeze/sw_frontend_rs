/// The can_dispatch module guides you through the startup process and filters CAN bus datagrams
///
/// The startup process checks whether the desired virtual device address is available and assigns
/// it. If it is not available, a replacement address is searched for. The bus is monitored and
/// all participants are identified by their heartbeats. If necessary, the information is updated
/// during operation, i.e. devices are deleted or added.
///
/// A filter function ensures that the application only receives the telegrams it needs. Legacy
/// telegrams are taken into account in the same way as telegrams according to specification. The
/// module is not dependent on the hardware or the application and can therefore be used in many
/// areas.
///
/// Another task of this module is to carry out the address conversion. CAN bus telegrams
/// according to specification are either generic or specific. Generic telegrams are assigned a
/// generic ID independently of the sender and forwarded to the application. Specific telegrams,
/// if selected by object id, are provided with the object id and forwarded.
///
/// This component runs in the context of the device driver.
///
/// [CAN Bus Specification](https://github.com/larus-breeze/doc_larus/tree/master/documentation)
use crate::{CanFrame, Frame, GenericFrame, SpecificFrame};

use heapless::{
    spsc::{Consumer, Producer, Queue},
    FnvIndexSet, Vec,
};

// This queue transports the can bus frames from the view component to the can dispatcher.
pub type QTxIrqFrames<const MAX_TX_FRAMES: usize> = Queue<CanFrame, MAX_TX_FRAMES>;
pub type PTxIrqFrames<const MAX_TX_FRAMES: usize> = Producer<'static, CanFrame, MAX_TX_FRAMES>;
pub type CTxIrqFrames<const MAX_TX_FRAMES: usize> = Consumer<'static, CanFrame, MAX_TX_FRAMES>;

// This queue transports the can bus frames from the view component to the can dispatcher.
pub type QTxFrames<const MAX_TX_FRAMES: usize> = Queue<Frame, MAX_TX_FRAMES>;
pub type PTxFrames<const MAX_TX_FRAMES: usize> = Producer<'static, Frame, MAX_TX_FRAMES>;
pub type CTxFrames<const MAX_TX_FRAMES: usize> = Consumer<'static, Frame, MAX_TX_FRAMES>;

// This queue transprot the can bus frames from the can dispatcher to the view component
pub type QRxFrames<const MAX_RX_FRAMES: usize> = Queue<Frame, MAX_RX_FRAMES>;
pub type PRxFrames<const MAX_RX_FRAMES: usize> = Producer<'static, Frame, MAX_RX_FRAMES>;
pub type CRxFrames<const MAX_RX_FRAMES: usize> = Consumer<'static, Frame, MAX_RX_FRAMES>;

pub trait CanRng {
    fn random(&mut self, min: u32, max: u32) -> u32;
}

#[derive(PartialEq)]
enum OpMode {
    Startup,
    Normal,
}

pub struct CanDispatch<
    const VDA: u16,
    const FILTER_ELEMENTS: usize,
    const MAX_TX: usize,
    const MAX_RX: usize,
    RNG: CanRng,
> {
    op_mode: OpMode,
    sec_tick: u8,

    // For the startup phase
    startup_stage: u8,
    next_startup_instant: Option<u64>,
    received_adgs: [bool; 16],

    // During normal operation
    vda: u16,
    can_devices: [CanDevice; 64],
    rng: RNG,
    legacy_filter: Vec<(u16, u16), FILTER_ELEMENTS>,
    object_id_filter: FnvIndexSet<u16, FILTER_ELEMENTS>,

    // Queues
    p_tx_irq_frames: PTxIrqFrames<MAX_TX>,
    p_rx_frames: PRxFrames<MAX_RX>,
    c_tx_frames: CTxFrames<MAX_TX>,
}

impl<
        const VDA: u16,
        const FILTER_ELEMENTS: usize,
        const MAX_TX: usize,
        const MAX_RX: usize,
        RNG: CanRng,
    > CanDispatch<VDA, FILTER_ELEMENTS, MAX_TX, MAX_RX, RNG>
{
    /// Initialization of the CAN dispatcher
    ///
    /// The rand_func() function provides 32-bit random values which are required for the
    /// startup process.
    ///
    /// p_rx_frames is the input of a queue that is
    /// used to transmit CAN bus telegrams to the application. c_view_ c_tx_frames is the
    /// output of a queue that is used to transport the CAN bus telegrams from the application.
    pub fn new(
        rng: RNG,
        p_tx_irq_frames: PTxIrqFrames<MAX_TX>,
        p_rx_frames: PRxFrames<MAX_RX>,
        c_tx_frames: CTxFrames<MAX_TX>,
    ) -> Self {
        CanDispatch {
            op_mode: OpMode::Startup,
            sec_tick: 0,

            startup_stage: 15,
            next_startup_instant: None,
            received_adgs: [false; 16],

            vda: 0,
            can_devices: [CanDevice::default(); 64],
            rng,
            legacy_filter: Vec::new(),
            object_id_filter: FnvIndexSet::new(),

            p_tx_irq_frames,
            p_rx_frames,
            c_tx_frames,
        }
    }

    /// Sets the filter for CAN bus ID ranges that are used by old devices that do not fulfill the
    /// specification.
    pub fn set_legacy_filter(&mut self, min: u16, max: u16) -> Result<(), (u16, u16)> {
        self.legacy_filter.push((min, max))
    }

    /// Sets teh filter for object ids according to the specification.
    pub fn set_object_id_filter(&mut self, object_id: u16) -> Result<bool, u16> {
        self.object_id_filter.insert(object_id)
    }

    /// The tick() function is called every 100ms with the tick stamp in µs resolution. It
    /// manages time dependencies in the CAN bus dispatcher. During startups, this function
    /// is also expected to be activated in the meantime at the exact time of the next datagram.
    ///
    /// This routine returns next wakeup time, when extra wakeup is needed
    pub fn tick(&mut self, ticks: u64) -> Option<u64> {
        let sec_tick = (ticks / 1_000_000) as u8;
        if sec_tick != self.sec_tick {
            self.sec_tick = sec_tick;

            let mut set_obj_ids: FnvIndexSet<u16, 128> = FnvIndexSet::new();
            for can_device in &mut self.can_devices {
                can_device.sec_tick();
                if can_device.is_alive() {
                    if set_obj_ids.contains(&can_device.object_id) {
                        can_device.set_is_first(false);
                    } else {
                        set_obj_ids.insert(can_device.object_id()).unwrap();
                        can_device.set_is_first(true);
                    }
                }
            }
        }
        match self.op_mode {
            OpMode::Startup => self.startup_tick(ticks),
            OpMode::Normal => self.norm_tick(ticks),
        }
    }

    /// rx_data() takes CAN bus frames from the hardware driver, analyzes them and passes them on
    /// to the application if necessary.
    pub fn rx_data(&mut self, can_frame: CanFrame) {
        if can_frame.is_heartbeat() {
            self.can_devices[can_frame.vda() as usize].set_object_id(can_frame);
            if can_frame.vda() == self.vda {
                self.op_mode = OpMode::Startup;
                self.startup_stage = 15;
                self.received_adgs = [false; 16];
            }
        }
        match self.op_mode {
            OpMode::Startup => {
                // Save startup stages of other devices
                if can_frame.id() < 16 {
                    self.received_adgs[can_frame.id() as usize] = true;
                }
            }
            OpMode::Normal => {
                // First check whether it is a legacy frame
                for (min, max) in &self.legacy_filter {
                    if can_frame.id() >= *min && can_frame.id() <= *max {
                        let legacy_frame = Frame::Legacy(can_frame);
                        let _ = self.p_rx_frames.enqueue(legacy_frame);
                        return;
                    }
                }
                // Then check whether it is a generic frame
                if let Some(generic_id) = can_frame.generic_id() {
                    // We don't want to see all the heartbeats
                    if generic_id > 0 {
                        let generic_frame = Frame::Generic(GenericFrame {
                            generic_id,
                            can_frame,
                        });
                        let _ = self.p_rx_frames.enqueue(generic_frame);
                    }
                    return;
                }
                // Finally: test whether the application is interested in the object_id
                let vda = can_frame.vda();
                let object_id = self.can_devices[vda as usize].object_id;
                let is_first = self.can_devices[vda as usize].is_first();
                if self.object_id_filter.contains(&object_id) && is_first {
                    if let Some(specific_id) = can_frame.specific_id() {
                        let specific_frame = Frame::Specific(SpecificFrame {
                            specific_id,
                            object_id,
                            can_frame,
                        });
                        let _ = self.p_rx_frames.enqueue(specific_frame);
                    }
                }
                // Ignore other can frames
            }
        }
    }

    fn startup_tick(&mut self, ticks: u64) -> Option<u64> {
        // send no information frames during startup
        while self.c_tx_frames.len() > 0 {
            let _ = self.c_tx_frames.dequeue();
        }

        let mut next = None;
        match self.next_startup_instant {
            Option::None => {
                next = Some(ticks + self.rng.random(500_000, 600_000) as u64);
                self.startup_stage = 15;
                let can_frame = CanFrame::remote_trans_rq(self.startup_stage as u16, 0);
                let _ = self.p_tx_irq_frames.enqueue(can_frame);
                self.received_adgs = [false; 16];
            }
            Some(current) => {
                if current <= ticks {
                    let can_frame = if !(self.received_adgs[(self.startup_stage - 1) as usize]
                        || self.received_adgs[(self.startup_stage - 2) as usize])
                    {
                        self.startup_stage -= 1;
                        if self.startup_stage <= 1 {
                            // Set operation mode to normal
                            self.op_mode = OpMode::Normal;
                            // Search for the next free virtual device address
                            self.vda = VDA;
                            while self.can_devices[self.vda as usize].is_alive() {
                                self.vda += 1;
                            }
                            // Send remote transmission request on heartbeat id
                            CanFrame::remote_trans_rq(self.heartbeat_id(), 0)
                        } else {
                            // Send remote transmission request on arbritation area
                            next = Some(current + self.rng.random(34_000, 67_000) as u64);
                            CanFrame::remote_trans_rq(self.startup_stage as u16, 0)
                        }
                    } else {
                        // Send remote transmission request on arbritation area, same stage
                        next = Some(current + self.rng.random(34_000, 67_000) as u64);
                        CanFrame::remote_trans_rq(self.startup_stage as u16, 0)
                    };
                    let _ = self.p_tx_irq_frames.enqueue(can_frame);
                    self.received_adgs = [false; 16];
                }
            }
        };
        self.next_startup_instant = next;
        next
    }

    pub fn norm_tick(&mut self, _ticks: u64) -> Option<u64> {
        while self.c_tx_frames.len() > 0 {
            let frame = self.c_tx_frames.dequeue().unwrap();
            let can_frame = match frame {
                Frame::Legacy(can_frame) => can_frame,
                Frame::Generic(generic_frame) => {
                    let mut can_frame = generic_frame.can_frame;
                    can_frame.set_id(self.heartbeat_id() + generic_frame.generic_id);
                    can_frame
                }
                Frame::Specific(specific_frame) => {
                    let mut can_frame = specific_frame.can_frame;
                    can_frame.set_id(self.base_id() + specific_frame.specific_id);
                    can_frame
                }
            };
            let _ = self.p_tx_irq_frames.enqueue(can_frame);
        }
        None
    }

    fn heartbeat_id(&self) -> u16 {
        (self.vda << 4) + 0x400
    }

    fn base_id(&self) -> u16 {
        self.vda << 4
    }
}

// Auxiliary structure for managing the bus devices
#[derive(Clone, Copy, Default)]
struct CanDevice {
    time_to_death: u8,
    is_first: bool,
    object_id: u16,
}

#[allow(unused)]
impl CanDevice {
    fn is_death(&mut self) -> bool {
        self.time_to_death == 0
    }

    fn is_alive(&mut self) -> bool {
        self.time_to_death != 0
    }

    fn sec_tick(&mut self) {
        if self.time_to_death > 0 {
            self.time_to_death -= 1;
            if self.time_to_death == 0 {
                self.object_id = 0;
            }
        }
    }

    /// Set object id and reset time-to-death counter
    fn set_object_id(&mut self, frame: CanFrame) {
        if frame.is_heartbeat() {
            self.object_id = frame.read_u16(0);
            self.time_to_death = 3;
        }
    }

    fn object_id(&self) -> u16 {
        self.object_id
    }

    fn set_is_first(&mut self, is_first: bool) {
        self.is_first = is_first;
    }

    fn is_first(&self) -> bool {
        self.is_first
    }
}

#[cfg(test)]
mod tests {
}