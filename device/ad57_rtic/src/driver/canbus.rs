use stm32f4xx_hal::{
    gpio::Pin,
    pac::CAN1,
    can::{Can, CanExt},
};
use bxcan::{
    filter::Mask32, Fifo,
    Frame, //StandardId},
};
use heapless::spsc::{Consumer, Producer, Queue};

const MAX_TX_FRAMES: usize = 10;
pub type QTxFrames = Queue<Frame, MAX_TX_FRAMES>;
pub type PTxFrames = Producer<'static, Frame, MAX_TX_FRAMES>;
pub type CTxFrames = Consumer<'static, Frame, MAX_TX_FRAMES>;

const MAX_RX_FRAMES: usize = 20;
pub type QRxFrames = Queue<Frame, MAX_RX_FRAMES>;
pub type PRxFrames = Producer<'static, Frame, MAX_RX_FRAMES>;
pub type CRxFrames = Consumer<'static, Frame, MAX_RX_FRAMES>;


pub fn init_can (
    can_1: CAN1,
    tx: Pin<'A', 12>,
    rx: Pin<'A', 11>,
    c_tx_frames: CTxFrames,
    p_rx_frames: PRxFrames,
)  -> (CanTx, CanRx) {

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

    let (tx, rx0, _rx1) = can.split();
    let tx_can = CanTx::new(c_tx_frames, tx);
    let rx_can = CanRx::new(p_rx_frames, rx0);
    (tx_can, rx_can)
}

pub struct CanTx {
    _tx: bxcan::Tx<Can<CAN1>>,
    _c_tx_frames: CTxFrames,
}

impl CanTx {
    fn new(c_tx_frames: CTxFrames, tx: bxcan::Tx::<Can<CAN1>>) -> Self {
        CanTx { _c_tx_frames: c_tx_frames, _tx: tx }
    }

    pub fn tick(&self) {}
}

pub struct CanRx {
    _p_rx_frames: PRxFrames,
    _rx0: bxcan::Rx0<Can<CAN1>>,
}

impl CanRx {
    fn new(p_rx_frames: PRxFrames, rx0:  bxcan::Rx0<Can<CAN1>>) -> Self {
        CanRx { _p_rx_frames: p_rx_frames, _rx0: rx0 }
    }

    pub fn tick(&self) {}
}