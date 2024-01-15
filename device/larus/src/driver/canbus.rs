use can_dispatch::{CanFrame, CTxIrqFrames};
use core::num::{NonZeroU16, NonZeroU8};
use fdcan::{
    config::NominalBitTiming,
    filter::{StandardFilter, StandardFilterSlot},
    frame::{FrameFormat, TxFrameHeader},
    id::{Id, StandardId},
    interrupt::{Interrupt, InterruptLine, Interrupts},
    FdCanControl, Fifo0, NormalOperationMode, Rx, Tx,
};
//use embedded_hal::can::{Frame, Id};
use stm32h7xx_hal::{can, gpio::Pin, gpio::Speed, pac::FDCAN1, prelude::*, rcc::rec::Fdcan};

/// Initialize peripheral bxcan and generate instances to send and receive can bus frames
pub fn init_can<const MAX_TX: usize>(
    fdcan_prec: Fdcan,
    fdcan_1: FDCAN1,
    rx: Pin<'B', 8>,
    tx: Pin<'B', 9>,
    c_tx_irq_frames: CTxIrqFrames<MAX_TX>,
) -> (CanTx<MAX_TX>, CanRx) {
    let mut can = {
        let rx = rx.into_alternate::<9>().speed(Speed::VeryHigh);
        let tx = tx.into_alternate::<9>().speed(Speed::VeryHigh);
        fdcan_1.fdcan(tx, rx, fdcan_prec)
    };
    // Kernel Clock 32MHz, Bit rate: 1MBit/s, Sample Point 87.5%
    // Value was calculated with http://www.bittiming.can-wiki.info/
    // TODO: use the can_bit_timings crate
    let data_bit_timing = NominalBitTiming {
        prescaler: NonZeroU16::new(5).unwrap(),
        seg1: NonZeroU8::new(8).unwrap(),
        seg2: NonZeroU8::new(1).unwrap(),
        sync_jump_width: NonZeroU8::new(1).unwrap(),
    };
    can.set_nominal_bit_timing(data_bit_timing);
    can.set_automatic_retransmit(false);

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
    let mut can = can.into_normal();

    // can.enable_interrupt(Interrupt::RxFifo0NewMsg);
    can.enable_interrupt_line(InterruptLine::_0, true);
    can.enable_interrupt_line(InterruptLine::_1, true);
    can.enable_interrupts(Interrupts::RX_FIFO0_NEW_MSG | Interrupts::TX_COMPLETE);

    let (ctrl, tx, rx, _) = can.split();
    let tx_can = CanTx::new(tx, c_tx_irq_frames);
    let rx_can = CanRx::new(rx, ctrl);
    (tx_can, rx_can)
}

/// Interrupt service for sending can bus frames
#[allow(unused)]
pub struct CanTx<const MAX_TX: usize> {
    tx: Tx<can::Can<FDCAN1>, NormalOperationMode>,
    c_tx_irq_frames: CTxIrqFrames<MAX_TX>,
    pub wakeup_at: u64, // just memory for isr
}

impl <const MAX_TX: usize> CanTx <MAX_TX> {
    /// Generate the service
    fn new(
        tx: Tx<can::Can<FDCAN1>, NormalOperationMode>,
        c_tx_irq_frames: CTxIrqFrames<MAX_TX>,
    ) -> Self {
        CanTx { tx, wakeup_at: 0, c_tx_irq_frames }
    }

    /// Method to call during an active interrupt
    pub fn on_interrupt(&mut self) {
        self.tx.clear_transmission_completed_flag(); // we want receive next irqs

        if self.c_tx_irq_frames.len() > 0 {
            let can_frame = self.c_tx_irq_frames.dequeue().unwrap();
            let header = TxFrameHeader {
                len: can_frame.dlc(),
                id: StandardId::new(can_frame.id()).unwrap().into(),
                frame_format: FrameFormat::Standard,
                bit_rate_switching: false,
                marker: None,
            };
            let buffer = can_frame.data();
            // so the result of transmit is ignored
            let _r = self.tx.transmit(header, buffer);
    
        }
    }
}

/// Interrupt service for receiving can bus frames
pub struct CanRx {
    rx: Rx<can::Can<FDCAN1>, NormalOperationMode, Fifo0>,
    ctrl: FdCanControl<can::Can<FDCAN1>, NormalOperationMode>,
}

impl CanRx {
    /// Create the service
    fn new(
        rx: Rx<can::Can<FDCAN1>, NormalOperationMode, Fifo0>,
        ctrl: FdCanControl<can::Can<FDCAN1>, NormalOperationMode>,
    ) -> Self {
        CanRx { rx, ctrl }
    }

    /// Call this, when irq is active
    pub fn on_interrupt(&mut self) -> Option<CanFrame> {
        self.ctrl.clear_interrupt(Interrupt::RxFifo0NewMsg);

        let mut buffer = [0u8; 8];
        match self.rx.receive(&mut buffer) {
            // silently ignore errors
            Ok(over_run) => {
                // Let's ignore overrun error, unwrap() is always ok here
                let rx_info = over_run.unwrap();
                if let Id::Standard(standard_id) = rx_info.id {
                    let id = standard_id.as_raw();
                    let len = rx_info.len;
                    if rx_info.rtr {
                        Some(CanFrame::remote_trans_rq(id, len))
                    } else {
                        Some(CanFrame::from_slice(id, &buffer[..len as usize]))
                    }
                } else {
                    None
                }
            }
            Err(_) => None, // Fifo is empty -> no more datagrams
        }
    }
}
