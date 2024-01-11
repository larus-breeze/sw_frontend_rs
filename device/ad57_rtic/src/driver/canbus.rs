use bxcan::{filter::Mask32, Data, Fifo, Frame, Id, Interrupt, StandardId};
use can_dispatch::CanFrame;
use corelib::basic_config::MAX_TX_FRAMES;
use heapless::Vec;
use stm32f4xx_hal::{
    can::{Can, CanExt},
    gpio::Pin,
    pac::CAN1,
};

/// Initialize peripheral bxcan and generate instances to send and receive can bus frames
pub fn init_can(can_1: CAN1, tx: Pin<'A', 12>, rx: Pin<'A', 11>) -> (CanTx, CanRx) {
    let mut can = {
        let rx = rx.into_alternate::<9>();
        let tx = tx.into_alternate::<9>();
        let can = can_1.can((tx, rx));
        bxcan::Can::builder(can)
            // APB1 (PCLK1): 42MHz, Bit rate: 1 MBit/s, Sample Point 87.5%
            // Value was calculated with http://www.bittiming.can-wiki.info/
            .set_bit_timing(0x001a0002)
            .enable()
    };

    let mut filters = can.modify_filters();
    filters
        .clear()
        .enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
    drop(filters); // Drop filters to leave filter configuraiton mode.

    can.enable_interrupt(Interrupt::Fifo0MessagePending);
    can.enable_interrupt(Interrupt::TransmitMailboxEmpty);

    let (tx, rx0, _rx1) = can.split();
    let tx_can = CanTx::new(tx);
    let rx_can = CanRx::new(rx0);
    (tx_can, rx_can)
}

/// Interrupt service for sending can bus frames
#[allow(unused)]
pub struct CanTx {
    tx: bxcan::Tx<Can<CAN1>>,
    pub wakeup_at: u64, // just memory for isr
    buf: Vec<CanFrame, MAX_TX_FRAMES>,
}

impl CanTx {
    /// Generate the service
    fn new(tx: bxcan::Tx<Can<CAN1>>) -> Self {
        CanTx {
            tx,
            wakeup_at: 0,
            buf: Vec::new(),
        }
    }

    /// Method to call during an active interrupt
    pub fn on_interrupt(&mut self) {
        self.tx.clear_interrupt_flags(); // we want receive next irqs
        if let Some(can_frame) = self.buf.pop() {
            let id = StandardId::new(can_frame.id()).unwrap();
            let bx_frame = if can_frame.is_rtr() {
                Frame::new_remote(id, can_frame.dlc())
            } else {
                let data = Data::new(can_frame.data()).unwrap();
                Frame::new_data(id, data)
            };
            let _ = self.tx.transmit(&bx_frame); // Silently ignore errors
        }
    }

    pub fn send_frame(&mut self, can_frame: CanFrame) {
        let _ = self.buf.push(can_frame);
    }
}

/// Interrupt service for receiving can bus frames
pub struct CanRx {
    rx0: bxcan::Rx0<Can<CAN1>>,
}

impl CanRx {
    /// Create the service
    fn new(rx0: bxcan::Rx0<Can<CAN1>>) -> Self {
        CanRx { rx0 }
    }

    /// Call this, when irq is active
    pub fn on_interrupt(&mut self) -> Option<CanFrame> {
        // trace!("Can rx irq");
        match self.rx0.receive() {
            // silently ignore errors
            Ok(bx_frame) => {
                let id = if let Id::Standard(standard_id) = bx_frame.id() {
                    standard_id.as_raw()
                } else {
                    return None;
                };
                let can_frame = if bx_frame.is_remote_frame() {
                    CanFrame::remote_trans_rq(id, bx_frame.dlc())
                } else {
                    CanFrame::from_slice(id, bx_frame.data().unwrap())
                };
                Some(can_frame)
            }
            Err(_) => None,
        }
    }
}
