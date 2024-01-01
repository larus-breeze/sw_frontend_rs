#![no_std]
#![no_main]

/// Below are a few general words about the structure of the application. RTIC version
/// 1.xx (https://rtic.rs/1/book/en/) is used here as the basis for the real-time system:
/// The app module defines the application. In RTIC tasks are equivalent to interrupt
/// service routines, so that all tasks/ISRs including their priorities can be found in
/// modul app. Thus one can recognize well the whole structure of the software.
///
/// However, there are some essential components missing. How do the tasks communicate with
/// each other?
///
/// On the one hand, thread-safe queus are used to communicate between tasks with different
/// priorities. Furthermore, there is a large core_model structure that holds all the data
/// required for the application. ISRs, Tasks, queues and the data model: that's all! There
/// are no other essential structural elements. The initialization of the hardware and software
/// components is done in the init.rs component. There one can also look at the connections
/// by means of queues.
///
/// The crate ad57_rtic contains the real-time system and the runtime environment for the target
/// hardware. The majority (core) of the application is designed to be portable and has no
/// dependencies on the hardware or the real-time system.
use defmt::trace;

use rtic::app;

mod dev_controller;
mod dev_view;
mod driver;
mod idle_loop;
mod macros;
mod utils;

use defmt_rtt as _;

use corelib::*;
use dev_controller::*;
use dev_view::*;
use driver::*;
use idle_loop::*;
use utils::*;

#[app(device = stm32f4xx_hal::pac, peripherals = true, dispatchers = [SPI1, SPI2, DMA2_STREAM0, DMA2_STREAM1])]
mod app {
    use super::*;

    /// Data used by more than one Task. The tasks are protected by RTIC against unauthorized
    /// access by setting priorities. Note: If only tasks with identical priority access,
    /// then nothing needs to be protected and the overhead does not apply. This is the case
    /// with the core_model.
    #[shared]
    struct Shared {
        core_model: CoreModel,  // holds the application data
        statistics: Statistics, // track the task runtimes
    }

    /// Data required by single tasks
    #[local]
    struct Local {
        can_rx: CanRx,             // receive can pakets
        can_tx: CanTx,             // transmit can pakets
        controller: DevController, // control the application
        idle_loop: IdleLoop,       // Idle loop and persistence layer
        view: DevView,             // bring application data to the user
        frame_buffer: FrameBuffer, // between view component and the LCD
        keyboard: Keyboard,        // capture the user input
    }

    /// Time base for the real-time system
    #[monotonic(binds = TIM2, default = true)]
    type Mono = DevMonoTimer;

    /// Initialization of the hardware and software
    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let (
            can_rx,
            can_tx,
            controller,
            core_model,
            mono_timer,
            mut view,
            idle_loop,
            frame_buffer,
            keyboard,
            statistics,
        ) = hw_init(cx.device, cx.core);

        view.setup_timer(DevInstant::from_ticks(timestamp() as u64));
        task_view::spawn().unwrap();
        task_controller::spawn().unwrap();
        task_keyboard::spawn().unwrap();

        // return the initialized components to RTIC
        (
            Shared {
                core_model,
                statistics,
            },
            Local {
                can_rx,
                can_tx,
                controller,
                idle_loop,
                view,
                frame_buffer,
                keyboard,
            },
            init::Monotonics(mono_timer),
        )
    }

    // In the following interrupt service routines and tasks are listed in descending order of
    // priority. In RTIC the difference between a task and an interrupt service routine is only
    // that the interrupt service routine is bound to the interrupt vector of a used peripheral,
    // while a task uses an interrupt vector not needed by the circuitry.

    /// Receive can frames
    #[task(binds = CAN1_RX0, local = [can_rx], shared = [statistics], priority=9)]
    fn isr_can_rx(mut cx: isr_can_rx::Context) {
        task_start!(cx, Task::CanRx);
        cx.local.can_rx.on_interrupt();
        task_end!(cx, Task::CanRx);
    }

    /// Send can frames
    #[task(binds = CAN1_TX, local = [can_tx], shared = [statistics], priority=8)]
    fn isr_can_tx(mut cx: isr_can_tx::Context) {
        task_start!(cx, Task::CanTx);
        cx.local.can_tx.on_interrupt();
        task_end!(cx, Task::CanTx);
    }

    /// Scan the keyboard
    #[task(local = [keyboard], shared = [statistics], priority=7)]
    fn task_keyboard(mut cx: task_keyboard::Context) {
        task_start!(cx, Task::Keys);

        task_keyboard::spawn_after(DevDuration::millis(20)).unwrap();
        cx.local.keyboard.tick();

        task_end!(cx, Task::Keys);
    }

    /// The controller contains the complete logic for processing the data and events
    #[task(local = [controller], shared = [core_model, statistics], priority=5)]
    fn task_controller(mut cx: task_controller::Context) {
        task_start!(cx, Task::Controller);

        task_controller::spawn_after(DevDuration::millis(100)).unwrap();
        let controller = cx.local.controller;
        let all_alive = cx
            .shared
            .statistics
            .lock(|statistics| statistics.all_alive());
        cx.shared.core_model.lock(|core_model| {
            if all_alive {
                core_model.send_idle_event(IdleEvent::FeedTheDog);
            }
            controller.tick(core_model)
        });

        task_end!(cx, Task::Controller);
    }

    /// Prepares the display and passes the data to the appropriate output routines.
    /// This mainly concerns the LCD but also the sound output.
    #[task(local = [view], shared = [core_model, statistics], priority=5)]
    fn task_view(mut cx: task_view::Context) {
        task_start!(cx, Task::LcdView);

        let view = cx.local.view;
        cx.shared.core_model.lock(|core_model| {
            let _ = view.tick(core_model);
        });
        let wake_up_at = view.wake_up_at();
        task_view::spawn_at(wake_up_at).unwrap();
        task_lcd_copy::spawn().unwrap();
        rtic::pend(stm32f4xx_hal::interrupt::CAN1_TX);

        task_end!(cx, Task::LcdView);
    }

    /// Copies the data from the frame buffer to the LCD
    #[task(local = [frame_buffer], shared = [statistics], priority=4)]
    fn task_lcd_copy(mut cx: task_lcd_copy::Context) {
        task_start!(cx, Task::LcdCopy);
        cx.local.frame_buffer.flush();
        task_end!(cx, Task::LcdCopy);
    }

    #[idle(local = [idle_loop])]
    fn idle(cx: idle::Context) -> ! {
        // Locals in idle have lifetime 'static
        trace!("idle");

        cx.local.idle_loop.idle_loop();
    }
}
