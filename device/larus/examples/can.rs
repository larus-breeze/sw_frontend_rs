#![no_main]
#![no_std]

mod driver;

use driver::*;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use panic_rtt_target as _;
use stm32h7xx_hal::{
    pac::{interrupt, CorePeripherals, Peripherals as DevicePeripherals, NVIC},
    prelude::*,
    rcc::{rec, PllConfigStrategy},
};
use corelib::{CanFrame, QTxFrames};
use heapless::spsc::Queue;


#[entry]
fn main() -> ! {
    // Setup clocks
    let mut cp = CorePeripherals::take().unwrap();
    let dp = DevicePeripherals::take().unwrap();

    info!("init");

    // Constrain and freeze power
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Initialize clock system
    let rcc = dp.RCC.constrain();
    let ccdr = rcc
        .use_hse(25.MHz())
        .sys_ck(192.MHz())
        .hclk(192.MHz()) // FMC clock from HCLK by default
        .pll1_strategy(PllConfigStrategy::Iterative)
        .pll1_q_ck(32.MHz())
        .pll2_p_ck(96.MHz())
        .pll2_r_ck(96.MHz())
        .freeze(pwrcfg, &dp.SYSCFG);

    // Initialize system...
    cp.SCB.enable_icache();
    cp.DWT.enable_cycle_counter();

    let mut delay = cp.SYST.delay(ccdr.clocks);

    let (mut p_tx_frames, c_tx_frames) = {
        static mut Q_TX_FRAMES: QTxFrames = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_TX_FRAMES.split() }
    };
    let (p_rx_frames, mut c_rx_frames) = {
        static mut Q_RX_FRAMES: QRxFrames = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_RX_FRAMES.split() }
    };

    let gpioh = dp.GPIOH.split(ccdr.peripheral.GPIOH);
    //let rx = gpioh.ph14.into_alternate().speed(Speed::VeryHigh);
    //let tx = gpioh.ph13.into_alternate().speed(Speed::VeryHigh);
    let fdcan_prec = ccdr
        .peripheral
        .FDCAN
        .kernel_clk_mux(rec::FdcanClkSel::Pll1Q);
    let fdcan_1 = dp.FDCAN1;

    let (tx_can, rx_can) = init_can(
        fdcan_prec, 
        fdcan_1, 
        gpioh.ph14, 
        gpioh.ph13, 
        c_tx_frames, 
        p_rx_frames
    );
    cortex_m::interrupt::free(|cs| {
        TX_CAN.borrow(cs).replace(Some(tx_can));
        RX_CAN.borrow(cs).replace(Some(rx_can));
    });

    unsafe {
        cp.NVIC.set_priority(interrupt::FDCAN1_IT0, 1);
        cp.NVIC.set_priority(interrupt::FDCAN1_IT1, 1);
        NVIC::unmask(interrupt::FDCAN1_IT0);
        NVIC::unmask(interrupt::FDCAN1_IT1);
    }

    let mut buffer: [u8; 8] = [1,2,3,4,5,6,7,8];

    loop {
        let tx_frame = CanFrame::from_slice(0x100, &buffer);
        let _ = p_tx_frames.enqueue(tx_frame);
        NVIC::pend(interrupt::FDCAN1_IT1);

        delay.delay_ms(100_u16);
        let o_rx_frame = c_rx_frames.dequeue();
        if let Some(rx_frame) = o_rx_frame {
            trace!("rx {:?}", rx_frame.data());
            (buffer[0], _) = buffer[0].overflowing_add(1);
        }
    }
}

static TX_CAN: Mutex<RefCell<Option<CanTx>>> = Mutex::new(RefCell::new(None));
static RX_CAN: Mutex<RefCell<Option<CanRx>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn FDCAN1_IT0() {  // rx
    // info!("FDCAN1_IT0 interrupt");
    cortex_m::interrupt::free(|cs| {
        let mut rc = RX_CAN.borrow(cs).borrow_mut();
        let rx = rc.as_mut().unwrap();
        rx.on_interrupt();
    })
}

#[interrupt]
fn FDCAN1_IT1() {  // tx
    // info!("FDCAN1_IT1 interrupt");
    cortex_m::interrupt::free(|cs| {
        let mut rc = TX_CAN.borrow(cs).borrow_mut();
        let tx = rc.as_mut().unwrap();
        tx.on_interrupt();
    })
}
