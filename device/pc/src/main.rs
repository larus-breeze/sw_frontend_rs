use vario_display::*;

use embedded_graphics::{
    pixelcolor::Rgb565, 
    prelude::*, 
    geometry::{Angle, AngleUnit}
};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};
use rand::Rng;
use ringbuffer::{AllocRingBuffer, RingBufferExt, RingBuffer, RingBufferWrite};

fn simulate(
    blackboard: &mut Blackboard,
    climb: &mut f32, 
    average_buffer: &mut AllocRingBuffer<f32>,
    opt_climb: &mut f32,
    climb_end: f32,
    wind_speed: &mut f32,
    wind_angle: &mut Angle,
    speed_to_fly_dif: &mut f32, 
    time: f32,
    rng: &mut rand::rngs::ThreadRng,
    vario: &mut VarioDisplay,
    display: &mut SimulatorDisplay<Rgb565>,
    window: &mut Window,
) 
{
    println!("climb_end: {:.1}, time: {:.1}", climb_end, time);

    let mut wind_incr = 0.1;
    let mut wind_angle_incr = 0.02.rad();
    let mut speed_to_fly_incr: f32 = 0.5;
    let count = (time * 14.0) as u32;
    let climb_inc = (climb_end - *climb) / (count as f32);
    for _ in 0..count {
        let mut sum = 0.0f32;
        for e in average_buffer.iter() {
            sum += e;
        }
        let average_climb = sum / (average_buffer.len() as f32);
        let rnd = rng.gen_range(-0.1..0.1);
        *climb += climb_inc + rnd as f32;
        *opt_climb = *opt_climb + (*climb - *opt_climb) / 20.0;
        average_buffer.push(*opt_climb);

        *wind_angle = (wind_angle.to_radians() + wind_angle_incr.to_radians()).rad();
        if *wind_angle > 360.0.deg() {
            wind_angle_incr = -0.02.rad();
        }
        if *wind_angle < 0.0.deg() {
            wind_angle_incr = 0.02.rad();
        };
        *wind_speed += wind_incr;
        if *wind_speed > 30.0 {
            wind_incr = -1.0*wind_incr;
        }

        *speed_to_fly_dif += speed_to_fly_incr;
        if *speed_to_fly_dif > 30.0 {
            speed_to_fly_incr = -0.5;
        }
        if *speed_to_fly_dif < -30.0 {
            speed_to_fly_incr = 0.5;
        }

        // set values and draw display
        blackboard.average_climb_rate =average_climb;
        blackboard.climb_rate = *opt_climb;
        blackboard.wind_angle =*wind_angle;
        blackboard.wind_speed = *wind_speed;
        blackboard.speed_to_fly_dif = *speed_to_fly_dif;

        vario.draw(display, blackboard).unwrap();
        window.update(display);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT));
    let mut window = Window::new("Vario - Mock", &OutputSettings::default());

    let mut climb: f32 = 0.0;
    let mut average_buffer = AllocRingBuffer::with_capacity(256);
    average_buffer.fill_with(|| 0.0f32);
    let mut opt_climb: f32 = 0.0;

    let mut wind_angle = 150.0.deg();
    let mut wind_speed: f32 = 15.0;

    let mut speed_to_fly_dif: f32 = 0.0;

    let mut rng = rand::thread_rng();
    let mut vario = VarioDisplay::new();
    let mut blackboard = Blackboard::new();
    blackboard.mc_cready = 1.7;
    blackboard.average_wind_speed = 25.0;
    blackboard.average_wind_angle = 330.0.deg();

    for _ in 1..20 {
        let climb_end = rng.gen_range(-1.5..4.5) as f32;
        let time = rng.gen_range(1.0..5.0) as f32;
        simulate(&mut blackboard, &mut climb, &mut average_buffer, &mut opt_climb, climb_end, &mut wind_speed, &mut wind_angle, 
            &mut speed_to_fly_dif, time, &mut rng, &mut vario, &mut display, &mut window);
    
    

        let time = rng.gen_range(2.0..30.0) as f32;
        simulate(&mut blackboard, &mut climb, &mut average_buffer, &mut opt_climb, climb_end, &mut wind_speed, &mut wind_angle, 
            &mut speed_to_fly_dif, time, &mut rng, &mut vario, &mut display, &mut window);
        }

    Ok(())
}
