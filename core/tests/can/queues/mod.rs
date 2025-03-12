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
    let (p_tx_irq_frames, c_tx_irq_frames) = spsc_queue!(QTxIrqFrames<10>);
    let (p_tx_frames, c_tx_frames) = spsc_queue!(QTxFrames<10>);
    let (p_rx_frames, mut c_rx_frames) = spsc_queue!(QRxFrames<30>);
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
