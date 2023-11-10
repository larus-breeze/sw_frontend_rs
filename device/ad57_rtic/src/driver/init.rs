use crate::driver::{
    frame_buffer::FrameBuffer, init_can, keyboard::*, CanRx, CanTx, DevLcdPins, Display, Eeprom,
    QRxFrames, QTxFrames,
};
use crate::{dev_controller::DevController, dev_view::DevView, idle_loop::IdleLoop, Statistics};
/// In the embedded rust ecosystem, hardware resources can only be used in one place. For this
/// reason, a careful distribution of the required hardware resources to corresponding software
/// components is necessary. This allocation is done here in the init component.
///
/// This makes it easy to see precisely which software component has which hardware. For example,
/// it can be seen below which pins and timers are used by the keyboard. Here in the init
/// routine the hardware and other resources are allocated - the actual initialization of the
/// hardware takes place however in the respective software component, which has the hardware.
///
/// In addition, the queues are set up here, which connect individual interrupt service rotines
/// with tasks communicatively. For example, a queue (Q_RX_FRAMES) is used for Can packets,
/// which forwards the frames from the interrupt service routine CanRx to the task DevController.
use defmt::*;
use heapless::spsc::Queue;
use stm32f4xx_hal::{
    fsmc_lcd::{DataPins16, LcdPins},
    gpio::alt::fsmc,
    pac,
    prelude::*,
    timer::monotonic::SysMonoTimerExt,
};
use systick_monotonic::*;
use vario_display::{CoreModel, QStorageItems};
use {defmt_rtt as _, panic_probe as _};

// Todo: use Timer as Timebase also for busy waiting
pub fn delay_ms(millis: u32) {
    let cycles = millis * 168_000;
    cortex_m::asm::delay(cycles)
}

pub const TICKS_PER_SECOND: u32 = 1_000;
pub type DevDuration = fugit::Duration<u64, 1, TICKS_PER_SECOND>;
pub type DevInstant = fugit::Instant<u64, 1, TICKS_PER_SECOND>;
pub type DevMonoTimer = Systick<TICKS_PER_SECOND>;

// Currently not in use
// pub type KeyBacklight = Pin<Output<PushPull>, 'A', 3>;
pub type DevDisplay = Display;

pub fn hw_init(
    device: pac::Peripherals,
    core: cortex_m::peripheral::Peripherals,
) -> (
    CanRx,
    CanTx,
    DevController,
    CoreModel,
    DevMonoTimer,
    DevView,
    IdleLoop,
    FrameBuffer,
    Keyboard,
    Statistics,
) {
    // Setup clocks
    let rcc = device.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(16.MHz())
        .sysclk(168.MHz())
        .hclk(168.MHz())
        .freeze();
    // trace!("AHB1 freq {}", clocks.hclk().0);    // 168 Mhz
    // trace!("APB1 freq {}", clocks.pclk1().0);   // 42 Mhz
    // trace!("APB2 freq {}", clocks.pclk2().0);   // 84 Mhz
    // trace!("Sysclock freq {}", clocks.sysclk().0); // 168 Mhz

    // Take ownership of GPIO ports
    let gpioa = device.GPIOA.split();
    let gpiob = device.GPIOB.split();
    let gpioc = device.GPIOC.split();
    let gpiod = device.GPIOD.split();
    let gpioe = device.GPIOE.split();

    // Setup ----------> the queues
    // This queue transports the can bus frames from the view component to the can tx driver.
    let (p_tx_frames, c_tx_frames) = {
        static mut Q_TX_FRAMES: QTxFrames = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_TX_FRAMES.split() }
    };

    // This queue transports the can bus frames from the can rx driver to the controller.
    let (p_rx_frames, c_rx_frames) = {
        static mut Q_RX_FRAMES: QRxFrames = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_RX_FRAMES.split() }
    };

    // This queue routes the key events from the keyboard crate to the controller.
    let (p_key_events, c_key_events) = {
        static mut Q_KEY_EVENTS: QKeyEvents = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_KEY_EVENTS.split() }
    };

    // This queue routes the StorageItems from the controller to the idle loop.
    let (p_sto_items, c_sto_items) = {
        static mut Q_STO_ITEMS: QStorageItems = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_STO_ITEMS.split() }
    };

    // Setup ----------> can bus interface
    let (can_tx, can_rx) = init_can(
        device.CAN1,
        gpioa.pa12,
        gpioa.pa11,
        c_tx_frames,
        p_rx_frames,
    );

    // Setup ----------> timer
    let dev_mono_timer = pac::SYST::monotonic(core.SYST, &clocks);

    // Setup ----------> statistics
    let statistics = Statistics::new(device.TIM2, &clocks);

    // Setup ----------> The front key interface
    let keyboard = {
        let keyboard_pins =
            KeyboardPins::new(gpioa.pa7, gpioc.pc5, gpiob.pb0, gpiob.pb1, gpioa.pa6);
        let enc1_res = Enc1Res::new(device.TIM4, gpiod.pd12, gpiod.pd13);
        let enc2_res = Enc2Res::new(device.TIM5, gpioa.pa0, gpioa.pa1);
        Keyboard::new(keyboard_pins, enc1_res, enc2_res, p_key_events)
    };

    // Setup ----------> Eeprom driver and idle loop
    let scl = gpiob.pb6.internal_pull_up(true);
    let sda = gpiob.pb7.internal_pull_up(true);
    let i2c = device.I2C1.i2c((scl, sda), 400.kHz(), &clocks);
    let mut eeprom = Eeprom::new(i2c).unwrap();

    // Setup ----------> CoreModel
    let mut core_model = CoreModel::new(p_sto_items);
    for item in eeprom.iter_over(vario_display::EepromTopic::ConfigValues) {
        core_model.restore_persistent_item(item);
    }
    let idle = IdleLoop::new(eeprom, c_sto_items);

    // Setup ----------> controller
    let dev_controller = DevController::new(&mut core_model, c_key_events, c_rx_frames);

    // Setup ----------> frame buffer
    let frame_buffer = FrameBuffer::new();

    // Setup ----------> LCD driver peripheral of STM32F407 and view component
    let dev_view = {
        //use stm32f4xx_hal::gpio::alt::fsmc as alt;
        let lcd_pins: DevLcdPins = LcdPins::new(
            DataPins16::new(
                gpiod.pd14, gpiod.pd15, gpiod.pd0, gpiod.pd1, gpioe.pe7, gpioe.pe8, gpioe.pe9,
                gpioe.pe10, gpioe.pe11, gpioe.pe12, gpioe.pe13, gpioe.pe14, gpioe.pe15, gpiod.pd8,
                gpiod.pd9, gpiod.pd10,
            ),
            fsmc::Address::from(gpiod.pd11),
            gpiod.pd4,
            gpiod.pd5,
            fsmc::ChipSelect1::from(gpiod.pd7),
        );
        let lcd_reset = gpiod.pd3.into_push_pull_output();

        // Initialize the display and clear the screen
        let display = Display::new(device.FSMC, lcd_pins, lcd_reset, frame_buffer.split());
        DevView::new(display, p_tx_frames)
    };

    // Setup ----------> Backlight Port an switch on the lcd
    let mut backlight = gpiob.pb4.into_push_pull_output();
    backlight.set_high(); // Is fixed at the moment, perhaps PWM in the future

    trace!("AD57 initialized");

    (
        can_rx,
        can_tx,
        dev_controller,
        core_model,
        dev_mono_timer,
        dev_view,
        idle,
        frame_buffer,
        keyboard,
        statistics,
    )
}
