#![no_main]
#![no_std]

mod driver;

use nb::block;
use core::{cell::RefCell, num::{NonZeroU16, NonZeroU8}};
use defmt::*;
use defmt_rtt as _;
use panic_rtt_target as _;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use stm32h7xx_hal::{
    pac::{
        CorePeripherals, Peripherals as DevicePeripherals, interrupt, NVIC, FDCAN1
    },
    prelude::*,
    can,
    gpio::Speed,
    rcc::{PllConfigStrategy, rec}, //device::fdcan1,
};
use fdcan::{
    InternalLoopbackMode, //FdCan, ConfigMode, 
    interrupt::Interrupt,
    config::NominalBitTiming,
    filter::{StandardFilter, StandardFilterSlot},
    frame::{FrameFormat, TxFrameHeader},
    id::StandardId,
};

#[allow(dead_code)]
#[derive(Debug)]
enum Error {
    EepromOrI2c1,
    NoItemAvailable,
}

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

    let fdcan_prec = ccdr
        .peripheral
        .FDCAN
        .kernel_clk_mux(rec::FdcanClkSel::Pll1Q);

    let mut can = {
        info!("Init CAN 1");
        let gpioh = dp.GPIOH.split(ccdr.peripheral.GPIOH);
        let rx = gpioh.ph14.into_alternate().speed(Speed::VeryHigh);
        let tx = gpioh.ph13.into_alternate().speed(Speed::VeryHigh);

        info!("-- Create CAN 1 instance");
        dp.FDCAN1.fdcan(tx, rx, fdcan_prec)
    };
    
    can.set_protocol_exception_handling(false);
    can.enable_interrupt(Interrupt::TxEmpty);

    info!("-- Configure nominal timing");
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

    info!("-- Configure Filters");
    can.set_standard_filter(
        StandardFilterSlot::_0,
        StandardFilter::accept_all_into_fifo0(),
    );

    info!("-- Set CAN mode");
    let can = can.into_internal_loopback();
    //let mut can = can.into_normal();

    cortex_m::interrupt::free(|cs| {
        FDCAN1.borrow(cs).replace(Some(can));
    });

    unsafe {
        cp.NVIC.set_priority(interrupt::FDCAN1_IT0, 1);
        NVIC::unmask(interrupt::FDCAN1_IT0);
    }


    info!("Create Message Data");
    let mut buffer = [
        0xAA, 0xAA, 0xAA, 0xAA, 0xFF, 0xFF, 0xFF, 0xFF, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ];
    info!("Create Message Header");
    let header = TxFrameHeader {
        len: 2 * 4,
        id: StandardId::new(0x1).unwrap().into(),
        frame_format: FrameFormat::Standard,
        bit_rate_switching: false,
        marker: None,
    };
//    info!("Initial Header: {:?}", &header);

    info!("Transmit initial message");
    cortex_m::interrupt::free(|cs| {
        let mut rc = FDCAN1.borrow(cs).borrow_mut();
        let can = rc.as_mut().unwrap();
        block!(can.transmit(header, &buffer)).unwrap();
    });

    loop {
        let res_rxheader = cortex_m::interrupt::free(|cs| {
            let mut rc = FDCAN1.borrow(cs).borrow_mut();
            let can = rc.as_mut().unwrap();
            block!(can.receive0(&mut buffer))
        });
        if let Ok(rx_header) = res_rxheader {
            //info!("Received Header Id: {:?}", rxheader);
            info!("received data: {:?}", &buffer);
            delay.delay_ms(1_u16);

            let tx_header = rx_header.unwrap().to_tx_header(None);
            cortex_m::interrupt::free(|cs| {
                let mut rc = FDCAN1.borrow(cs).borrow_mut();
                let can = rc.as_mut().unwrap();
                block!(can.transmit(tx_header, &buffer)).unwrap();
            });
            info!("Transmit: {:?}", buffer);
        }
    }

}

static FDCAN1: Mutex<RefCell<Option<fdcan::FdCan<can::Can<FDCAN1>, InternalLoopbackMode>>>> =
    Mutex::new(RefCell::new(None));

/*#[interrupt]
fn FDCAN1_IT0() {
    cortex_m::interrupt::free(|cs| {
        let mut rc = FDCAN1.borrow(cs).borrow_mut();
        let can = rc.as_mut().unwrap();
        can.clear_interrupt(Interrupt::TxEmpty);
    })
}*/
