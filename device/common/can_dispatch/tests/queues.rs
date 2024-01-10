use can_dispatch::*;
use heapless::spsc::Queue;

#[allow(unused)]
pub fn get_the_queues() -> (
    PViewTxFrames<10>,
    CViewTxFrames<10>,
    PViewRxFrames<30>,
    CViewRxFrames<30>,
) {
    let (p_view_tx_frames, c_view_tx_frames) = {
        static mut Q_VIEW_TX_FRAMES: QViewTxFrames<10> = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_VIEW_TX_FRAMES.split() }
    };
    let (p_view_rx_frames, mut c_view_rx_frames) = {
        static mut Q_VIEW_RX_FRAMES: QViewRxFrames<30> = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_VIEW_RX_FRAMES.split() }
    };
    (
        p_view_tx_frames,
        c_view_tx_frames,
        p_view_rx_frames,
        c_view_rx_frames,
    )
}
