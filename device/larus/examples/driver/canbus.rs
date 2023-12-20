use core::{
    num::{NonZeroU16, NonZeroU8},
    ops::Deref,
};
use fdcan::{
    config::NominalBitTiming,
    filter::{StandardFilter, StandardFilterSlot},
    frame::{FrameFormat, TxFrameHeader},
    id::{Id, StandardId},
    interrupt::{Interrupt, InterruptLine, Interrupts},
    InternalLoopbackMode, ConfigMode, FdCan,
    Tx, Rx, Fifo0, ReceiveOverrun, FdCanControl,
};
use corelib::CTxFrames;
use defmt::*;
//use embedded_hal::can::{Frame, Id};
use heapless::spsc::{Consumer, Producer, Queue};
use stm32h7xx_hal::{
    gpio::Pin,
    can,
    gpio::Speed,
    pac::{interrupt, CorePeripherals, Peripherals as DevicePeripherals, FDCAN1, NVIC},
    prelude::*,
    rcc::{rec, PllConfigStrategy, rec::Fdcan},
};

use corelib::{frontend, sensor, CanFrame};

// This queue transports the can bus frames from the can rx driver to the controller.
const MAX_RX_FRAMES: usize = 20;
pub type QRxFrames = Queue< CanFrame, MAX_RX_FRAMES>;
pub type PRxFrames = Producer<'static, CanFrame, MAX_RX_FRAMES>;
pub type CRxFrames = Consumer<'static, CanFrame, MAX_RX_FRAMES>;

/// Initialize peripheral bxcan and generate instances to send and receive can bus frames
pub fn init_can(
    fdcan_prec: Fdcan,
    fdcan_1: FDCAN1,
    rx: Pin<'H', 14>,
    tx: Pin<'H', 13>,
    c_tx_frames: CTxFrames,
    p_rx_frames: PRxFrames,
) -> (CanTx, CanRx) {
    let mut can = {
        let rx = rx.into_alternate::<9>().speed(Speed::VeryHigh);
        let tx = tx.into_alternate::<9>().speed(Speed::VeryHigh);
        fdcan_1.fdcan(tx, rx, fdcan_prec)
    };
    // Kernel Clock 32MHz, Bit rate: 1MBit/s, Sample Point 87.5%
    // Value was calculated with http://www.bittiming.can-wiki.info/
    // TODO: use the can_bit_timings crate
    let data_bit_timing = NominalBitTiming {
        prescaler: NonZeroU16::new(2).unwrap(),
        seg1: NonZeroU8::new(13).unwrap(),
        seg2: NonZeroU8::new(2).unwrap(),
        sync_jump_width: NonZeroU8::new(1).unwrap(),
    };
    can.set_nominal_bit_timing(data_bit_timing);

    can.set_standard_filter(
        StandardFilterSlot::_0,
        StandardFilter::accept_all_into_fifo0(),
    );
    can.set_protocol_exception_handling(false);
    can.select_interrupt_line_1(Interrupts::TX_COMPLETE);

    // Unsafe during init of peripheral is ok
    unsafe {
        // FDCAN_TXBTIE Tx buffer transmission interrupt enable register
        core::ptr::write_volatile(0x4000a0e0 as *mut u32, 0xffff_ffff);
    }
    let mut can = can.into_internal_loopback();
    //let mut can = can.into_normal();

    // can.enable_interrupt(Interrupt::RxFifo0NewMsg);
    can.enable_interrupt_line(InterruptLine::_0, true);
    can.enable_interrupt_line(InterruptLine::_1, true);
    can.enable_interrupts(Interrupts::RX_FIFO0_NEW_MSG | Interrupts::TX_COMPLETE);

    let (
        ctrl, 
        tx, 
        rx, 
        _
    ) = can.split();
    let tx_can = CanTx::new(c_tx_frames, tx);
    let rx_can = CanRx::new(p_rx_frames, rx, ctrl);
    (tx_can, rx_can)
}

/// Interrupt service for sending can bus frames
pub struct CanTx {
    tx: Tx<can::Can<FDCAN1>, InternalLoopbackMode>,
    c_tx_frames: CTxFrames,
    extra_frame: Option<CanFrame>,
}

impl CanTx {
    /// Generate the service
    fn new(c_tx_frames: CTxFrames, tx: Tx<can::Can<FDCAN1>, InternalLoopbackMode>) -> Self {
        CanTx {
            c_tx_frames,
            tx,
            extra_frame: None,
        }
    }

    /// Method to call during an active interrupt
    pub fn on_interrupt(&mut self) {
        self.tx.clear_transmission_completed_flag(); // we want receive next irqs

        // now we work off the queue
        while !self.tx.tx_queue_is_full() && self.c_tx_frames.len() > 0 {
            match self.c_tx_frames.dequeue() {
                Some(frame) => {
                    let header = TxFrameHeader {
                        len: frame.dlc(),
                        id: StandardId::new(frame.id()).unwrap().into(),
                        frame_format: FrameFormat::Standard,
                        bit_rate_switching: false,
                        marker: None,
                    };
                    let buffer = frame.data();
                    // tx queue is not full, so the result of transmit can be ignored
                    let _r = self.tx.transmit(header, buffer);
                },
                None => return,
            }
        }
    }
}

/// Interrupt service for receiving can bus frames
pub struct CanRx {
    p_rx_frames: PRxFrames,
    rx: Rx<can::Can<FDCAN1>, InternalLoopbackMode, Fifo0>,
    ctrl: FdCanControl<can::Can<FDCAN1>, InternalLoopbackMode>,
}

impl CanRx {
    /// Create the service
    fn new(
        p_rx_frames: PRxFrames, rx: Rx<can::Can<FDCAN1>, 
        InternalLoopbackMode, Fifo0>,
        ctrl: FdCanControl<can::Can<FDCAN1>, InternalLoopbackMode>) -> Self {
        CanRx { p_rx_frames, rx, ctrl }
    }

    /// Call this, when irq is active
    pub fn on_interrupt(&mut self) {
        self.ctrl.clear_interrupt(Interrupt::RxFifo0NewMsg);

        while self.p_rx_frames.capacity() > self.p_rx_frames.len() {
            let mut buffer = [0u8; 8];
            match self.rx.receive(&mut buffer) {
                // silently ignore errors
                Ok(over_run) => {
                    match over_run {
                        ReceiveOverrun::NoOverrun(rx_info) => {
                            if let Id::Standard(standard_id) = rx_info.id {
                                let id = standard_id.as_raw();
                                let len = rx_info.len;
                                let can_frame = if rx_info.rtr {
                                    CanFrame::remote_trans_rq(id, len)
                                } else {
                                    CanFrame::from_slice(id, &buffer[..len as usize])
                                };
                                let _ = self.p_rx_frames.enqueue(can_frame);
                            }
                        },
                        ReceiveOverrun::Overrun(_) => (),
                    }
                }
                Err(_) => return,
            }
        }
    }
}
