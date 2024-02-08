use corelib::*;
use heapless::spsc::Queue;

#[allow(unused)]
pub fn get_the_queues() -> (
    PTxIrqFrames<10>,
    CTxIrqFrames<10>,
    PTxFrames<10>,
    CTxFrames<10>,
    PRxFrames<30>,
    CRxFrames<30>,
) {
    let (p_tx_irq_frames, c_tx_irq_frames) = {
        static mut Q_TX_IRQ_FRAMES: QTxIrqFrames<10> = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_TX_IRQ_FRAMES.split() }
    };
    let (p_tx_frames, c_tx_frames) = {
        static mut Q_TX_FRAMES: QTxFrames<10> = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_TX_FRAMES.split() }
    };
    let (p_rx_frames, mut c_rx_frames) = {
        static mut Q_RX_FRAMES: QRxFrames<30> = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_RX_FRAMES.split() }
    };
    (
        p_tx_irq_frames,
        c_tx_irq_frames,
        p_tx_frames,
        c_tx_frames,
        p_rx_frames,
        c_rx_frames,
    )
}

pub struct Rng {}

impl CanRng for Rng {
    fn random(&mut self, min: u32, max: u32) -> u32 {
        min + (max - min) / 2
    }
}
