use crate::{driver::*, utils::*, DevController, DevView, IdleLoop};
use corelib::{CoreModel, QIdleEvents, basic_config::{MAX_TX_FRAMES, MAX_RX_FRAMES}};
use can_dispatch::{QViewTxFrames, QViewRxFrames, CanDispatch};
use cortex_m::peripheral::Peripherals as CorePeripherals;
use defmt::*;
use heapless::{mpmc::MpMcQueue, spsc::Queue};
use st7789::ST7789;
use stm32h7xx_hal::{
    pac::Peripherals as DevicePeripherals,
    prelude::*,
    rcc::{rec, rec::FmcClkSel},
};

pub type DevCanDispatch = CanDispatch<32, 8, 10, 30, DevRng>;

pub fn hw_init(
    dp: DevicePeripherals,
    mut cp: CorePeripherals,
) -> (
    DevCanDispatch,
    CanRx,
    CanTx,
    CoreModel,
    DevController,
    DevView,
    FrameBuffer,
    IdleLoop,
    Keyboard,
    MonoTimer,
    Statistics,
) {
    // Setup ----------> the queues

    // This queue transports the can bus frames from the can dispatcher to the controller.
    let (p_view_rx_frames, c_view_rx_frames) = {
        static mut Q_VIEW_RX_FRAMES: QViewRxFrames<MAX_RX_FRAMES> = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_VIEW_RX_FRAMES.split() }
    };

    // This queue transports the can bus frames from the controller to the can dispatcher.
    let (p_view_tx_frames, c_view_tx_frames) = {
        static mut Q_VIEW_TX_FRAMES: QViewTxFrames<MAX_TX_FRAMES> = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_VIEW_TX_FRAMES.split() }
    };

    // This queue routes the StorageItems from the controller to the idle loop.
    let (p_idle_events, c_idle_events) = {
        static mut Q_IDLE_EVENTS: QIdleEvents = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_IDLE_EVENTS.split() }
    };

    // This queue routes the events to the controller.
    static Q_EVENTS: QEvents = MpMcQueue::new();

    // Constrain and freeze power, save a little bit power, optimum is at vos3 / 200 MHz
    let pwrcfg = dp.PWR.constrain().vos3().freeze();
    let ccdr = dp
        .RCC
        .constrain()
        .use_hse(16.MHz())
        .sys_ck(200.MHz())
        .hclk(100.MHz())
        .pll1_q_ck(50.MHz()) // CAN
        .pll2_p_ck(100.MHz()) // ?
        .pll2_r_ck(50.MHz()) // LCD
        .freeze(pwrcfg, &dp.SYSCFG);

    // Enable cortex m7 cache and cyclecounter
    cp.SCB.enable_icache();
    cp.DWT.enable_cycle_counter();

    // Setup ----------> system timer
    let mono = MonoTimer::new(dp.TIM2, ccdr.peripheral.TIM2, &ccdr.clocks);
    let mut delay = Delay {};

    // Take ownership of GPIO ports
    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);
    let gpiof = dp.GPIOF.split(ccdr.peripheral.GPIOF);

    // Setup ----------> The front key interface
    let keyboard = {
        let keyboard_pins = KeyboardPins::new(gpioe.pe5, gpioe.pe6, gpioe.pe4);
        let enc1_res = Enc1Res::new(ccdr.peripheral.TIM5, dp.TIM5, gpioa.pa0, gpioa.pa1);
        let enc2_res = Enc2Res::new(ccdr.peripheral.TIM3, dp.TIM3, gpiob.pb4.into(), gpioa.pa7);
        Keyboard::new(keyboard_pins, enc1_res, enc2_res, &Q_EVENTS)
    };

    // Setup ----------> Statistics
    let statistics = Statistics::new();

    // Setup ----------> Canbus
    let (tx_can, rx_can) = {
        let fdcan_prec = ccdr
            .peripheral
            .FDCAN
            .kernel_clk_mux(rec::FdcanClkSel::Pll1Q);
        let fdcan_1 = dp.FDCAN1;
        init_can(
            fdcan_prec,
            fdcan_1,
            gpiob.pb8,
            gpiob.pb9,
        )
    };

    let rng = dp.RNG.constrain(ccdr.peripheral.RNG, &ccdr.clocks);
    let rnd = DevRng::new(rng);

    let mut can_dispatch = CanDispatch::new(rnd, p_view_rx_frames, c_view_tx_frames);
    can_dispatch.set_legacy_filter(0x100, 0x120).unwrap();

    // Setup ----------> Frame buffer, Display
    let (frame_buffer, dev_view) = {
        let lcd_pins = LcdPins::new(
            DataPins16::new(
                gpiod.pd14, gpiod.pd15, gpiod.pd0, gpiod.pd1, gpioe.pe7, gpioe.pe8, gpioe.pe9,
                gpioe.pe10, gpioe.pe11, gpioe.pe12, gpioe.pe13, gpioe.pe14, gpioe.pe15, gpiod.pd8,
                gpiod.pd9, gpiod.pd10,
            ),
            gpiod.pd11,
            gpiod.pd4,
            gpiod.pd5,
            gpiod.pd7,
        );
        let pfmc = ccdr.peripheral.FMC;
        let pfmc = pfmc.kernel_clk_mux(FmcClkSel::Pll2R);
        let interface = LcdInterface::new(pfmc, dp.FMC, lcd_pins);

        let lcd_reset = gpioc.pc0.into_push_pull_output();
        let backlight_control = gpiof.pf5.into_push_pull_output();

        // Add LCD controller driver
        let mut lcd = ST7789::new(
            interface,
            Some(lcd_reset),
            Some(backlight_control),
            320,
            240,
        );
        // Initialise the display and clear the screen
        lcd.init(&mut delay).unwrap();
        lcd.set_orientation(st7789::Orientation::PortraitSwapped)
            .unwrap();

        let stream0 = stm32h7xx_hal::dma::mdma::StreamsTuple::new(dp.MDMA, ccdr.peripheral.MDMA).0;

        let (frame_buffer, display) = FrameBuffer::new(lcd, stream0);
        (frame_buffer, DevView::new(display))
    };

    // Setup ----------> CoreModel
    let mut core_model = CoreModel::new(p_idle_events, p_view_tx_frames, uuid());

    // Setup ----------> controller
    let dev_controller = DevController::new(&mut core_model, &Q_EVENTS, c_view_rx_frames);

    // Setup ----------> Idleloop (last, because of the dog)
    let idle_loop = {
        let scl = gpiob.pb6.into_alternate_open_drain();
        let sda = gpiob.pb7.into_alternate_open_drain();
        let i2c = dp
            .I2C1
            .i2c((scl, sda), 400.kHz(), ccdr.peripheral.I2C1, &ccdr.clocks);
        let mut eeprom = Storage::new(i2c).unwrap();
        for item in eeprom.iter_over(corelib::EepromTopic::ConfigValues) {
            core_model.restore_persistent_item(item);
        }
   
        IdleLoop::new(eeprom, c_idle_events, &Q_EVENTS)
    };

    info!("Larus Ad57 finished");

    (
        can_dispatch,
        rx_can,
        tx_can,
        core_model,
        dev_controller,
        dev_view,
        frame_buffer,
        idle_loop,
        keyboard,
        mono,
        statistics,
    )
}
