use crate::{driver::*, utils::*, DevController, DevView, IdleLoop, DEVICE_CONST};
use corelib::{
    basic_config::{MAX_RX_FRAMES, MAX_TX_FRAMES, VDA},
    CanDispatch, CoreModel, QIdleEvents, QRxFrames, QTxFrames, QTxIrqFrames,
};
use cortex_m::peripheral::Peripherals as CorePeripherals;
use defmt::*;
use heapless::{mpmc::MpMcQueue, spsc::Queue};
use stm32h7xx_hal::{
    adc,
    dma::dma::StreamsTuple,
    independent_watchdog::IndependentWatchdog,
    pac::Peripherals as DevicePeripherals,
    prelude::*,
    rcc::{rec, rec::AdcClkSel, ResetEnable},
};

pub type DevCanDispatch = CanDispatch<VDA, 8, MAX_TX_FRAMES, MAX_RX_FRAMES, DevRng>;

pub const H7_HCLK: u32 = 200_000_000;

pub fn hw_init(
    dp: DevicePeripherals,
    mut cp: CorePeripherals,
) -> (
    DevCanDispatch,
    CanRx,
    CanTx<MAX_TX_FRAMES>,
    CoreModel,
    DevController,
    DevView,
    IdleLoop,
    Keyboard,
    MonoTimer,
    NmeaRx,
    NmeaTx,
    Sound,
    Statistics,
) {
    // Setup ----------> the queues

    // This queue transports the can bus frames from the can dispatcher to the irq routine.
    let (p_tx_irq_frames, c_tx_irq_frames) = {
        static mut Q_TX_IRQ_FRAMES: QTxIrqFrames<MAX_TX_FRAMES> = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_TX_IRQ_FRAMES.split() }
    };

    // This queue transports the can bus frames from the can dispatcher to the controller.
    let (p_rx_frames, c_rx_frames) = {
        static mut Q_RX_FRAMES: QRxFrames<MAX_RX_FRAMES> = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_RX_FRAMES.split() }
    };

    // This queue transports the can bus frames from the controller to the can dispatcher.
    let (p_tx_frames, c_tx_frames) = {
        static mut Q_TX_FRAMES: QTxFrames<MAX_TX_FRAMES> = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_TX_FRAMES.split() }
    };

    // This queue routes the StorageItems from the controller to the idle loop.
    let (p_idle_events, c_idle_events) = {
        static mut Q_IDLE_EVENTS: QIdleEvents = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_IDLE_EVENTS.split() }
    };

    // This queue routes the events to the controller.
    static Q_EVENTS: QEvents = MpMcQueue::new();

    // Unfortantly adc does not work on vos3
    // Constrain and freeze power, save a little bit power, optimum is at vos3 / 200 MHz
    // let pwrcfg = dp.PWR.constrain().vos3().freeze();
    // Constrain and Freeze power
    let pwrcfg = dp.PWR.constrain().freeze();

    let mut ccdr = dp
        .RCC
        .constrain()
        .use_hse(16.MHz())
        .sys_ck(400.MHz())
        .hclk(H7_HCLK.Hz())
        .pll1_q_ck(50.MHz()) // CAN
        .pll2_p_ck(100.MHz()) // ?
        .pll2_r_ck(50.MHz()) // LCD
        .pll3_p_ck(150.MHz())
        .pll3_q_ck(150.MHz())
        .pll3_r_ck(9.MHz()) // LTDC pixel frequency
        .freeze(pwrcfg, &dp.SYSCFG);

    // Enable cortex m7 cache and cyclecounter
    cp.SCB.enable_icache();
    cp.DWT.enable_cycle_counter();

    ccdr.peripheral.kernel_adc_clk_mux(AdcClkSel::Per);

    // Setup ----------> system timer
    let mono = MonoTimer::new(dp.TIM2, ccdr.peripheral.TIM2, &ccdr.clocks);
    let mut delay = Delay {};

    // Take ownership of GPIO ports
    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);
    let gpiog = dp.GPIOG.split(ccdr.peripheral.GPIOG);
    let gpioh = dp.GPIOH.split(ccdr.peripheral.GPIOH);
    let gpioi = dp.GPIOI.split(ccdr.peripheral.GPIOI);
    let gpioj = dp.GPIOJ.split(ccdr.peripheral.GPIOJ);
    let gpiok = dp.GPIOK.split(ccdr.peripheral.GPIOK);

    // Switch LCD Backlight off
    let mut backlight_control = gpioc.pc6.into_push_pull_output();
    backlight_control.set_high();

    // Setup ----------> The front key interface
    let keyboard = {
        let keyboard_pins = KeyboardPins::new(gpioa.pa3);
        let enc1_res = Enc1Res::new(ccdr.peripheral.TIM5, dp.TIM5, gpioa.pa0, gpioa.pa1);
        let enc2_res = Enc2Res::new(ccdr.peripheral.TIM3, dp.TIM3, gpiob.pb4.into(), gpioc.pc7);
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
        init_can(fdcan_prec, fdcan_1, gpiob.pb8, gpiob.pb9, c_tx_irq_frames)
    };

    let rng = dp.RNG.constrain(ccdr.peripheral.RNG, &ccdr.clocks);
    let rnd = DevRng::new(rng);

    let mut can_dispatch = CanDispatch::new(rnd, p_tx_irq_frames, p_rx_frames, c_tx_frames);
    can_dispatch.set_legacy_filter(0x100, 0x11f).unwrap();
    can_dispatch.set_legacy_filter(0x282, 0x282).unwrap(); // Vario display master device avg_climb_rates
    let _ = can_dispatch.set_object_id_filter(2); // Sensorbox
    let _ = can_dispatch.set_object_id_filter(3); // Gps

    // Setup ----------> CoreModel
    let mut core_model = CoreModel::new(&&DEVICE_CONST, uuid());

    // Setup ----------> Frame buffer, Display
    let dev_view = {
        let lcd_pins = LcdPins(gpiob.pb5, gpiog.pg9, gpiog.pg11, gpioh.ph4, gpioh.ph3);
        St7701s::init(
            dp.SPI1,
            ccdr.peripheral.SPI1,
            lcd_pins,
            &ccdr.clocks,
            &mut delay,
        );

        let ltdc_pins = LtdcPins(
            gpioa.pa5, gpioa.pa8, gpioc.pc0, gpiod.pd3, gpiog.pg6, gpiog.pg7, gpiog.pg10,
            gpioi.pi11, gpioi.pi12, gpioi.pi13, gpioj.pj1, gpioj.pj2, gpioj.pj9, gpioj.pj11,
            gpioj.pj12, gpioj.pj15, gpiok.pk0, gpiok.pk3, gpiok.pk4, gpiok.pk5, gpiok.pk6,
            gpiok.pk7,
        );
        let ltdc = Ltdc::init(dp.LTDC, ltdc_pins, ccdr.peripheral.LTDC, &ccdr.clocks);

        let frame_buffer = FrameBuffer::new(ltdc);
        let display = Display::new(frame_buffer);

        DevView::new(display, &core_model)
    };

    // Setup ----------> controller
    let mut dev_controller = {
        let mut adc1 = adc::Adc::adc1(
            dp.ADC1,
            4.MHz(),
            &mut delay,
            ccdr.peripheral.ADC12,
            &ccdr.clocks,
        );
        adc1.calibrate();

        DevController::new(
            &mut core_model,
            &Q_EVENTS,
            p_idle_events,
            p_tx_frames,
            c_rx_frames,
            adc1.enable(),
            gpioa.pa6,
            gpiob.pb1,
        )
    };

    // Setup ----------> Idleloop
    let idle_loop = {
        let mut wp = gpioc.pc5.into_push_pull_output();
        wp.set_low(); // Always enable writing to the eeprom

        let scl = gpiob.pb6.into_alternate_open_drain();
        let sda = gpiob.pb7.into_alternate_open_drain();
        let i2c = dp
            .I2C1
            .i2c((scl, sda), 400.kHz(), ccdr.peripheral.I2C1, &ccdr.clocks);

        let pins = SdcardPins(
            gpioc.pc12,
            gpiod.pd2,
            gpioc.pc8,
            gpioc.pc9,
            gpioc.pc10,
            gpioc.pc11,
            gpioa.pa15.into(),
        );

        // Init filesystem if sdcard available
        let _ = FileSys::new(pins, dp.SDMMC1, ccdr.peripheral.SDMMC1, &ccdr.clocks).ok();

        // Init reset watch and create entry in PANIC.LOG if watchdog reset
        ResetWatch::new();

        let watchdog = IndependentWatchdog::new(dp.IWDG);

        let idle_loop = IdleLoop::new(
            i2c,
            watchdog,
            c_idle_events,
            &Q_EVENTS,
            &mut core_model,
            &mut dev_controller,
        );

        // switch LCD backlight on, after eventually firmware update (avoids flickering)
        backlight_control.set_low();

        idle_loop
    };

    let streams = StreamsTuple::new(dp.DMA1, ccdr.peripheral.DMA1);

    // Setup ----------> Sound
    let sound = {
        let dac = dp.DAC.dac(gpioa.pa4, ccdr.peripheral.DAC12);
        let dac = dac.calibrate_buffer(&mut delay);
        let _ = ccdr.peripheral.TIM6.enable();
        Sound::new(dac, dp.TIM6, streams.0)
    };

    // Setup ----------> Nmea
    let (nmea_tx, nmea_rx) = NmeaTxRx::new(
        streams.1,
        streams.2,
        gpioa.pa9,
        gpiob.pb15,
        dp.USART1,
        ccdr.peripheral.USART1,
        &ccdr.clocks,
    );

    // set time of controller to current time
    dev_controller.set_ms(timestamp_ms());

    info!("Larus init finished");

    (
        can_dispatch,
        rx_can,
        tx_can,
        core_model,
        dev_controller,
        dev_view,
        idle_loop,
        keyboard,
        mono,
        nmea_rx,
        nmea_tx,
        sound,
        statistics,
    )
}
