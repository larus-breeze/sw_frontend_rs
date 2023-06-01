#![no_std]
#![no_main]

use core::convert::Infallible;

use rtic::app;
use systick_monotonic::fugit::ExtU32;

use defmt::*;
use {defmt_rtt as _, panic_probe as _};

use driver::{
    hw_init, MonoTimer,
    FrameBuffer, Display,
    r61580::{
        Instruction,
        PORTRAIT_AVAIL_WIDTH, PORTRAIT_AVAIL_HEIGHT,
        PORTRAIT_ORIGIN_X, PORTRAIT_ORIGIN_Y,
    }
};

use vario_display::{Colors, RGB565_COLORS};

use stm32f4xx_hal::{
        gpio::{gpiob::PB4, Pin, Output, PushPull},
        pac,
};

mod driver;
mod utils;

use vario_display::*;

defmt::timestamp!("{=u32:us}", {
    // NOTE(interrupt-safe) single instruction volatile read operation
    unsafe { core::ptr::read_volatile(0x4000_0c24 as *const u32) }
});


#[app(device = stm32f4xx_hal::pac, peripherals = true, dispatchers = [SPI1, DMA2_STREAM0])]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        backlight: PB4<Output<PushPull>>,
        state: bool,
        display: Display<Pin<Output<PushPull>, 'D', 3>, Infallible>,
        frame_buffer: FrameBuffer,
    }

    #[monotonic(binds = TIM5, default = true)]
    type MyMono = MonoTimer<pac::TIM5, 1_000_000>;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let device = cx.device;
        let (
            backlight,
            display,
            mono,
            frame_buffer
        ) = hw_init(device, cx.core);

        // Schedule the blinking task
        blink::spawn_after(1000.millis()).unwrap();

        (
            Shared {},
            Local {frame_buffer, display, backlight, state: false },
            init::Monotonics(mono),
        )
    }

    #[task(local = [display, backlight, state], priority=5)]
    fn blink(cx: blink::Context) {
        trace!("blink");
        if *cx.local.state {
            let model = Blackboard::new();
            let mut vario = VarioDisplay::new();
            let _ = vario.draw(cx.local.display, &model);
            //let _ = cx.local.display.flush();
            trace!("Spawn lcd_copy");
            let _ = lcd_copy::spawn();
            cx.local.backlight.set_high();
            *cx.local.state = false;
        } else {
            //cx.local.backlight.set_low();
            *cx.local.state = true;
        }
        blink::spawn_after(1000_000.micros()).unwrap();
        trace!("blink_ready")
    }

    #[task(local = [frame_buffer, pixel_storage: [u16; PORTRAIT_AVAIL_WIDTH as usize] = [0; PORTRAIT_AVAIL_WIDTH as usize]], priority=6)]
    fn lcd_copy(cx: lcd_copy::Context) {
        trace!("lcd_copy");

        #[inline]
        fn write_command(cmd: u8) {
            unsafe {core::ptr::write_volatile(0x60000000 as *mut u8, cmd)}
        }
    
        #[inline]
        fn write_data(data: u16) {
            unsafe {core::ptr::write_volatile(0x60020000 as *mut u16, data)};
        }
    
        fn write_command_and_data(cmd: u8, data: u16) {
            write_command(cmd);
            write_data(data)
        }
    
        for y in 0..PORTRAIT_AVAIL_HEIGHT {
            write_command_and_data(Instruction::PosX as u8, PORTRAIT_ORIGIN_X);
            write_command_and_data(Instruction::PosY as u8, y + PORTRAIT_ORIGIN_Y);
            write_command(Instruction::Gram as u8);
            for x in 0..PORTRAIT_AVAIL_WIDTH {
                let idx = x as usize + y as usize * PORTRAIT_AVAIL_WIDTH as usize;
                let color = cx.local.frame_buffer.buf[idx] as usize;
                write_data(RGB565_COLORS[color]);
            }
        }
        trace!("lcd_copy_ready");
    }
}
