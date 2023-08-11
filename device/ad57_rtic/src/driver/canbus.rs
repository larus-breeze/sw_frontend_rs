use bxcan::{filter::Mask32, Fifo, Frame, Interrupt};
use defmt::*;
use heapless::spsc::{Consumer, Producer, Queue};
use stm32f4xx_hal::{
    can::{Can, CanExt},
    gpio::Pin,
    pac::CAN1,
};

// This queue transports the can bus frames from the view component to the can tx driver.
const MAX_TX_FRAMES: usize = 10;
pub type QTxFrames = Queue<Frame, MAX_TX_FRAMES>;
pub type PTxFrames = Producer<'static, Frame, MAX_TX_FRAMES>;
pub type CTxFrames = Consumer<'static, Frame, MAX_TX_FRAMES>;

// This queue transports the can bus frames from the can rx driver to the controller.
const MAX_RX_FRAMES: usize = 20;
pub type QRxFrames = Queue<Frame, MAX_RX_FRAMES>;
pub type PRxFrames = Producer<'static, Frame, MAX_RX_FRAMES>;
pub type CRxFrames = Consumer<'static, Frame, MAX_RX_FRAMES>;



/// Initialize peripheral bxcan and generate instances to send and receive can bus frames
pub fn init_can(
    can_1: CAN1,
    tx: Pin<'A', 12>,
    rx: Pin<'A', 11>,
    c_tx_frames: CTxFrames,
    p_rx_frames: PRxFrames,
) -> (CanTx, CanRx) {
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
    filters.enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
    drop(filters); // Drop filters to leave filter configuraiton mode.

    can.enable_interrupt(Interrupt::Fifo0MessagePending);
    can.enable_interrupt(Interrupt::TransmitMailboxEmpty);

    let (tx, rx0, _rx1) = can.split();
    let tx_can = CanTx::new(c_tx_frames, tx);
    let rx_can = CanRx::new(p_rx_frames, rx0);
    (tx_can, rx_can)
}

/// Interrupt service for sending can bus frames
pub struct CanTx {
    tx: bxcan::Tx<Can<CAN1>>,
    c_tx_frames: CTxFrames,
    extra_frame: Option<Frame>,
}

impl CanTx {
    /// Generate the service
    fn new(c_tx_frames: CTxFrames, tx: bxcan::Tx<Can<CAN1>>) -> Self {
        CanTx {
            c_tx_frames,
            tx,
            extra_frame: None,
        }
    }

    /// Method to call during an active interrupt
    pub fn on_interrupt(&mut self) {
        // trace!("Can tx irq");

        self.tx.clear_interrupt_flags(); // we want receive next irqs

        // we first check if there is anything left in the extra frame buffer
        if let Some(frame) = &self.extra_frame {
            if let Ok(transmit_status) = self.tx.transmit(frame) {
                if let Some(frame) = transmit_status.dequeued_frame() {
                    // Dropping into a mailbox did not work. We need to save the unstored
                    // frame and wait until something is free again
                    self.extra_frame = Some(frame.clone());
                    return; // All mailboxes are full
                } else {
                    self.extra_frame = None;
                    trace!("Extra frame put into can tx mailbox");
                }
            }
        }

        // now we work off the queue
        while let Some(frame) = self.c_tx_frames.dequeue() {
            if let Ok(transmit_status) = self.tx.transmit(&frame) {
                if let Some(frame) = transmit_status.dequeued_frame() {
                    // Dropping into a mailbox did not work. We need to save the unstored
                    // frame and wait until something is free again
                    self.extra_frame = Some(frame.clone());
                    return; // All mailboxes are full
                }
            }
        }
    }
}

/// Interrupt service for receiving can bus frames
pub struct CanRx {
    p_rx_frames: PRxFrames,
    rx0: bxcan::Rx0<Can<CAN1>>,
}

impl CanRx {
    /// Create the service
    fn new(p_rx_frames: PRxFrames, rx0: bxcan::Rx0<Can<CAN1>>) -> Self {
        CanRx { p_rx_frames, rx0 }
    }

    /// Call this, when irq is active
    pub fn on_interrupt(&mut self) {
        // trace!("Can rx irq");
        while self.p_rx_frames.capacity() > self.p_rx_frames.len() {
            match self.rx0.receive() { // silently ignore errors
                Ok(frame) => {
                    let _ = self.p_rx_frames.enqueue(frame); 
                },
                Err(_) => return,
            }
        }
    }
}
