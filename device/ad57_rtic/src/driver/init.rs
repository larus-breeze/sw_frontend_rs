use crate::driver::{frame_buffer::*, init_can, keyboard::*, CanRx, CanTx, MonoTimer, Storage};
use crate::utils::{HW_VERSION, SW_VERSION};
use crate::{
    dev_controller::DevController, dev_view::DevView, driver::*, idle_loop::IdleLoop, Statistics,
};
use corelib::{
    basic_config::{MAX_RX_FRAMES, MAX_TX_FRAMES, VDA},
    CanDispatch, CoreModel, Event, QIdleEvents, QRxFrames, QTxFrames, QTxIrqFrames,
};
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
use defmt_rtt as _;
use heapless::{mpmc::MpMcQueue, spsc::Queue};
use stm32f4xx_hal::{
    fsmc_lcd::{DataPins16, LcdPins},
    gpio::alt::fsmc,
    pac,
    pac::interrupt,
    pac::NVIC,
    prelude::*,
    watchdog::IndependentWatchdog,
};

pub const TICKS_PER_SECOND: u32 = 1_000_000;
pub type DevDuration = fugit::Duration<u64, 1, TICKS_PER_SECOND>;
pub type DevInstant = fugit::Instant<u64, 1, TICKS_PER_SECOND>;
pub type DevMonoTimer = MonoTimer;

pub type QEvents = MpMcQueue<Event, 8>;

// Currently not in use
// pub type KeyBacklight = Pin<Output<PushPull>, 'A', 3>;
pub type DevDisplay = Display;

pub type DevCanDispatch = CanDispatch<VDA, 8, MAX_TX_FRAMES, MAX_RX_FRAMES, DevRng>;

pub fn hw_init(
    device: pac::Peripherals,
    mut core: cortex_m::peripheral::Peripherals,
) -> (
    DevCanDispatch,
    CanRx,
    CanTx<MAX_TX_FRAMES>,
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
        .require_pll48clk()
        .freeze();
    // trace!("AHB1 freq {}", clocks.hclk().0);    // 168 Mhz
    // trace!("APB1 freq {}", clocks.pclk1().0);   // 42 Mhz
    // trace!("APB2 freq {}", clocks.pclk2().0);   // 84 Mhz
    // trace!("Sysclock freq {}", clocks.sysclk().0); // 168 Mhz

    // Setup ----------> timer
    let dev_mono_timer = MonoTimer::new(device.TIM2, &clocks);

    // Take ownership of GPIO ports
    let gpioa = device.GPIOA.split();
    let gpiob = device.GPIOB.split();
    let gpioc = device.GPIOC.split();
    let gpiod = device.GPIOD.split();
    let gpioe = device.GPIOE.split();

    // Setup ----------> the queues
    // This queue transports the can bus frames from the view component to the can tx driver.
    let (p_tx_irq_frames, c_tx_irq_frames) = {
        static mut Q_TX_IRQ_FRAMES: QTxIrqFrames<MAX_TX_FRAMES> = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_TX_IRQ_FRAMES.split() }
    };

    // This queue transports the can bus frames from the view component to the can tx driver.
    let (p_tx_frames, c_tx_frames) = {
        static mut Q_TX_FRAMES: QTxFrames<MAX_TX_FRAMES> = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_TX_FRAMES.split() }
    };

    // This queue transports the can bus frames from the can rx driver to the controller.
    let (p_rx_frames, c_rx_frames) = {
        static mut Q_RX_FRAMES: QRxFrames<MAX_RX_FRAMES> = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_RX_FRAMES.split() }
    };

    // This queue routes the StorageItems from the controller to the idle loop.
    let (p_idle_events, c_idle_events) = {
        static mut Q_IDLE_EVENTS: QIdleEvents = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_IDLE_EVENTS.split() }
    };

    // This queue routes the events to the controller.
    static Q_EVENTS: QEvents = MpMcQueue::new();

    // Setup ----------> can bus interface
    let (can_tx, can_rx) = init_can(device.CAN1, gpioa.pa12, gpioa.pa11, c_tx_irq_frames);

    let rng = device.RNG.constrain(&clocks);
    let rnd = DevRng::new(rng);

    let mut can_dispatch: DevCanDispatch =
        CanDispatch::new(rnd, p_tx_irq_frames, p_rx_frames, c_tx_frames);
    can_dispatch.set_legacy_filter(0x100, 0x120).unwrap();

    // Setup ----------> statistics
    let statistics = Statistics::new();

    // Setup ----------> The front key interface
    let keyboard = {
        let keyboard_pins =
            KeyboardPins::new(gpioa.pa7, gpioc.pc5, gpiob.pb0, gpiob.pb1, gpioa.pa6);
        let enc1_res = Enc1Res::new(device.TIM4, gpiod.pd12, gpiod.pd13);
        let enc2_res = Enc2Res::new(device.TIM5, gpioa.pa0, gpioa.pa1);
        Keyboard::new(keyboard_pins, enc1_res, enc2_res, &Q_EVENTS)
    };

    // Setup ----------> Eeprom driver for idle loop
    let scl = gpiob.pb6.internal_pull_up(true);
    let sda = gpiob.pb7.internal_pull_up(true);
    let i2c = device.I2C1.i2c((scl, sda), 400.kHz(), &clocks);
    let mut eeprom = Storage::new(i2c).unwrap();

    // Setup ----------> CoreModel
    let mut core_model = CoreModel::new(p_idle_events, p_tx_frames, uuid(), HW_VERSION, SW_VERSION);
    for item in eeprom.iter_over(corelib::EepromTopic::ConfigValues) {
        core_model.restore_persistent_item(item);
    }

    // Setup ----------> controller
    let dev_controller = DevController::new(&mut core_model, &Q_EVENTS, c_rx_frames);

    // Setup ----------> LCD driver peripheral of STM32F407 and view component
    let (dev_view, frame_buffer) = {
        //use stm32f4xx_hal::gpio::alt::fsmc as alt;
        let lcd_pins = LcdPins::new(
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

        let (frame_buffer, display) = FrameBuffer::new(device.FSMC, lcd_pins, lcd_reset);

        unsafe {
            core.NVIC.set_priority(interrupt::DMA2_STREAM0, 3);
            NVIC::unmask(interrupt::DMA2_STREAM0);
        }

        // Initialize the display and clear the screen
        (DevView::new(display), frame_buffer)
    };

    // Setup ----------> Idleloop (last, because of the dog)
    let watchdog = IndependentWatchdog::new(device.IWDG);
    let idle_loop = {
        // Setup ----------> FileSys driver for idle loop
        let sdio_pins: SdioPins = (
            gpioc.pc12,
            gpiod.pd2.internal_pull_up(true),
            gpioc.pc8.internal_pull_up(true),
            gpioc.pc9.internal_pull_up(true),
            gpioc.pc10.internal_pull_up(true),
            gpioc.pc11.internal_pull_up(true),
        );

        //let sd_detect = gpioc.pc0.internal_pull_up(true).into_input();
        // Init filesystem if sdcard available
        let _ = FileSys::new(device.SDIO, &clocks, sdio_pins);

        // Init reset watch and create entry in PANIC.LOG if watchdog reset
        ResetWatch::new();

        IdleLoop::new(eeprom, c_idle_events, &Q_EVENTS, watchdog)
    };
    trace!("AD57 initialized");

    // Setup ----------> Backlight Port an switch on the lcd as a last action
    // Should be activated at the very end, otherwise the LCD will show a strange display during 
    // the firmware update.
    let mut backlight = gpiob.pb4.into_push_pull_output();
    backlight.set_high(); // Is fixed at the moment, perhaps PWM in the future
    (
        can_dispatch,
        can_rx,
        can_tx,
        dev_controller,
        core_model,
        dev_mono_timer,
        dev_view,
        idle_loop,
        frame_buffer,
        keyboard,
        statistics,
    )
}
