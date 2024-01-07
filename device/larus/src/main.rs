#![no_main]
#![no_std]

mod dev_controller;
mod dev_view;
mod driver;
mod idle_loop;
mod macros;
mod utils;

use defmt_rtt as _;
use panic_rtt_target as _;

use rtic::app;

use corelib::*;
use dev_controller::*;
use dev_view::*;
use driver::*;
use idle_loop::*;
use utils::*;

#[app(device = stm32h7xx_hal::pac, peripherals = true, dispatchers = [SPI1, SPI2, ETH])]
mod app {
    use super::*;

    #[monotonic(binds = TIM2, default = true)]
    type MyMono = MonoTimer;

    #[shared]
    struct Shared {
        core_model: CoreModel,
        frame_buffer: FrameBuffer,
        statistics: Statistics,
    }

    #[local]
    struct Local {
        can_rx: CanRx,
        can_tx: CanTx,
        controller: DevController,
        dev_view: DevView,
        idle_loop: IdleLoop,
        keyboard: Keyboard,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let (
            can_rx,
            can_tx,
            core_model,
            controller,
            mut dev_view,
            frame_buffer,
            idle_loop, 
            keyboard,
            mono,
            statistics,
        ) = hw_init(cx.device, cx.core);

        dev_view.setup_timer(DevInstant::from_ticks(timestamp() as u64));
        task_view::spawn().unwrap();
        task_controller::spawn().unwrap();
        task_keyboard::spawn().unwrap();

        (
            Shared {
                core_model,
                frame_buffer,
                statistics,
            },
            Local {
                can_rx,
                can_tx,
                controller,
                dev_view,
                idle_loop,
                keyboard,
            },
            init::Monotonics(mono), // Give the monotonic to RTIC
        )
    }

    /// Receive can frames
    #[task(binds = FDCAN1_IT0, local = [can_rx], shared = [statistics], priority=9)]
    fn isr_can_rx(mut cx: isr_can_rx::Context) {
        task_start!(cx, Task::CanRx);
        cx.local.can_rx.on_interrupt();
        task_end!(cx, Task::CanRx);
    }

    /// Send can frames
    #[task(binds = FDCAN1_IT1, local = [can_tx], shared = [statistics], priority=8)]
    fn isr_can_tx(mut cx: isr_can_tx::Context) {
        task_start!(cx, Task::CanTx);
        cx.local.can_tx.on_interrupt();
        task_end!(cx, Task::CanTx);
    }

    /// Support of M-DMA (isr to copy the data from the frame buffer to the LCD)
    #[task(binds = MDMA, shared = [frame_buffer, statistics], priority=8)]
    fn isr_mdma(mut cx: isr_mdma::Context) {
        task_start!(cx, Task::Mdma);
        cx.shared.frame_buffer.lock(|frame_buffer| {
            frame_buffer.on_interrupt();
        });
        task_end!(cx, Task::Mdma);
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
    #[task(local = [dev_view], shared = [core_model, frame_buffer, statistics], priority=5)]
    fn task_view(mut cx: task_view::Context) {
        task_start!(cx, Task::LcdView);

        let view = cx.local.dev_view;
        cx.shared.core_model.lock(|core_model| {
            let _ = view.tick(core_model);
        });
        let wake_up_at = view.wake_up_at();
        task_view::spawn_at(wake_up_at).unwrap();
        //rtic::pend(stm32f4xx_hal::interrupt::CAN1_TX);
        cx.shared.frame_buffer.lock(|frame_buffer| {
            frame_buffer.flush();
        });
        task_end!(cx, Task::LcdView);
    }

    #[idle(local = [idle_loop])]
    fn idle(cx: idle::Context) -> ! {
        cx.local.idle_loop.idle_loop();
    }
}
