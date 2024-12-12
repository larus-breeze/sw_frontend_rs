#![no_main]
#![no_std]

mod dev_const;
mod dev_controller;
mod dev_view;
mod driver;
mod idle_loop;
mod macros;
mod utils;

use corelib::basic_config::MAX_TX_FRAMES;
#[allow(unused)]
use defmt::trace;
use defmt_rtt as _;
use stm32h7xx_hal::interrupt;

use corelib::*;
use dev_const::*;
use dev_controller::*;
use dev_view::*;
use driver::*;
use idle_loop::*;
use rtic::app;
use utils::*;

#[app(device = stm32h7xx_hal::pac, peripherals = true, dispatchers = [SPI1, SPI2, ETH])]
mod app {
    use super::*;

    #[monotonic(binds = TIM2, default = true)]
    type MyMono = MonoTimer;

    #[shared]
    struct Shared {
        can_dispatch: DevCanDispatch,
        can_tx: CanTx<MAX_TX_FRAMES>,
        controller: DevController,
        core_model: CoreModel,
        sound: Sound,
        statistics: Statistics,
    }

    #[local]
    struct Local {
        can_rx: CanRx,
        dev_view: DevView,
        idle_loop: IdleLoop,
        keyboard: Keyboard,
        nmea_rx: NmeaRx,
        nmea_tx: NmeaTx,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let (
            can_dispatch,
            can_rx,
            can_tx,
            core_model,
            controller,
            mut dev_view,
            idle_loop,
            keyboard,
            mono,
            nmea_rx,
            nmea_tx,
            sound,
            statistics,
        ) = hw_init(cx.device, cx.core);

        dev_view.setup_timer(DevInstant::from_ticks(timestamp_us() as u64));
        task_view::spawn().unwrap();
        task_controller::spawn().unwrap();
        task_keyboard::spawn().unwrap();
        task_can_timer::spawn().unwrap();

        (
            Shared {
                can_dispatch,
                can_tx,
                controller,
                core_model,
                sound,
                statistics,
            },
            Local {
                can_rx,
                dev_view,
                idle_loop,
                keyboard,
                nmea_rx,
                nmea_tx,
            },
            init::Monotonics(mono), // Give the monotonic to RTIC
        )
    }

    #[task(binds = DMA1_STR0, shared = [statistics, sound], priority=6)]
    fn isr_sound(mut cx: isr_sound::Context) {
        task_start!(cx, Task::Sound);
        cx.shared.sound.lock(|sound| sound.on_interrupt());
        task_end!(cx, Task::Sound);
    }

    /// Receive can frames
    #[task(binds = FDCAN1_IT0, local = [can_rx], shared = [statistics, can_dispatch], priority=5)]
    fn isr_can_rx(mut cx: isr_can_rx::Context) {
        task_start!(cx, Task::CanRx);
        loop {
            let can_frame = cx.local.can_rx.on_interrupt();
            match can_frame {
                Option::None => break,
                Option::Some(can_frame) => {
                    cx.shared.can_dispatch.lock(|can_dispatch| {
                        can_dispatch.rx_data(can_frame);
                    });
                }
            }
        }
        task_end!(cx, Task::CanRx);
    }

    /// Transmit can frames
    #[task(binds = FDCAN1_IT1, shared = [can_tx, statistics], priority=5)]
    fn isr_can_tx(mut cx: isr_can_tx::Context) {
        task_start!(cx, Task::CanTx);
        cx.shared.can_tx.lock(|can_tx| can_tx.on_interrupt());
        task_end!(cx, Task::CanTx);
    }

    /// Task to support can dispatcher with timing functions
    #[task(shared = [statistics, can_dispatch, can_tx], priority=5)]
    fn task_can_timer(mut cx: task_can_timer::Context) {
        task_start!(cx, Task::CanTimer);

        let ticks = app::monotonics::now().ticks();
        let next_wakeup = cx
            .shared
            .can_dispatch
            .lock(|can_dispatch| can_dispatch.tick(ticks));
        let wakeup_at = cx.shared.can_tx.lock(|can_tx| {
            let wakeup_at = next_wakeup.unwrap_or(can_tx.wakeup_at + 100_000);
            can_tx.wakeup_at = wakeup_at;
            wakeup_at
        });
        let instant = DevInstant::from_ticks(wakeup_at);
        task_can_timer::spawn_at(instant).unwrap();
        rtic::pend(interrupt::FDCAN1_IT1);
        task_end!(cx, Task::CanTimer);
    }

    /// Handle the reception of NMEA data
    #[task(binds = USART1, local = [nmea_rx], shared = [controller, core_model, statistics], priority=5)]
    fn isr_nmea_rx(mut cx: isr_nmea_rx::Context) {
        task_start!(cx, Task::NmeaRx);
        let nmea_rx = cx.local.nmea_rx;
        nmea_rx.on_interrupt();
        cx.shared.core_model.lock(|core_model| {
            cx.shared.controller.lock(|controller| {
                let cc = controller.core();
                while let Some(chunk) = nmea_rx.read() {
                    cc.nmea_recv_slice(core_model, chunk);
                }
            })
        });
        task_end!(cx, Task::NmeaRx);
    }

    /// Handle the transmission of NMEA data
    #[task(binds = DMA1_STR1, local = [nmea_tx], shared = [controller, core_model, statistics], priority=5)]
    fn isr_nmea_tx(mut cx: isr_nmea_tx::Context) {
        task_start!(cx, Task::NmeaTx);
        let nmea_tx = cx.local.nmea_tx;
        if nmea_tx.ready() {
            cx.shared.core_model.lock(|core_model| {
                cx.shared.controller.lock(|controller| {
                    let cc = controller.core();
                    if let Some(chunk) = cc.nmea_next(core_model) {
                        nmea_tx.send(chunk);
                    }
                })
            });
        }
        task_end!(cx, Task::NmeaTx);
    }

    /// The controller contains the complete logic for processing the data and events
    #[task(shared = [controller, core_model, sound, statistics], priority=5)]
    fn task_controller(mut cx: task_controller::Context) {
        task_start!(cx, Task::Controller);

        let all_alive = cx
            .shared
            .statistics
            .lock(|statistics| statistics.all_alive());

        let recalc = cx.shared.controller.lock(|controller| {
            if all_alive {
                // feed the watchdog, if all tasks are alive
                controller.core().send_idle_event(IdleEvent::FeedTheDog);
            }
            cx.shared.core_model.lock(|core_model| {
                // do controller calculations
                if controller.tick_1ms(core_model) {
                    rtic::pend(interrupt::DMA1_STR1); // check if there is something to send
                    Some((
                        core_model.calculated.frequency,
                        core_model.calculated.continuous,
                        core_model.calculated.gain,
                    ))
                } else {
                    None
                }
            })
        });

        // set sound params
        if let Some((frequecy, continuous, gain)) = recalc {
            cx.shared.sound.lock(|sound| {
                sound.set_params(frequecy, continuous, gain);
            });
        }

        task_controller::spawn_after(DevDuration::millis(1)).unwrap();
        task_end!(cx, Task::Controller);
    }

    /// Scan the keyboard
    #[task(local = [keyboard], shared = [statistics], priority=4)]
    fn task_keyboard(mut cx: task_keyboard::Context) {
        task_start!(cx, Task::Keyboard);

        task_keyboard::spawn_after(DevDuration::millis(20)).unwrap();
        cx.local.keyboard.tick();

        task_end!(cx, Task::Keyboard);
    }

    /// Prepares the display and passes the data to the appropriate output routines.
    /// This mainly concerns the LCD but also the sound output.
    #[task(local = [dev_view], shared = [core_model, statistics], priority=3)]
    fn task_view(mut cx: task_view::Context) {
        task_start!(cx, Task::View);

        let view = cx.local.dev_view;
        cx.shared.core_model.lock(|core_model| {
            view.core().prepare(core_model);
        });
        let _ = view.core().draw();
        view.core().display.show();

        let wake_up_at = view.wake_up_at();
        task_view::spawn_at(wake_up_at).unwrap();

        task_end!(cx, Task::View);
    }

    #[idle(local = [idle_loop])]
    fn idle(cx: idle::Context) -> ! {
        cx.local.idle_loop.idle_loop();
    }
}
