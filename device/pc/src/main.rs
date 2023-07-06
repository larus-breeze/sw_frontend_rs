use embedded_graphics::{
    geometry::{Angle, AngleUnit},
    prelude::*,
};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};
use rand::Rng;
use ringbuffer::{AllocRingBuffer, RingBuffer, RingBufferExt, RingBufferWrite};
use vario_display::*;

struct MockDisplay(pub SimulatorDisplay<Colors>);

impl MockDisplay {
    /// Creates a new display.
    ///
    /// The display is filled with `C::from(BinaryColor::Off)`.
    pub fn new(size: Size) -> Self {
        let sd = SimulatorDisplay::with_default_color(size, Colors::Black);
        MockDisplay(sd)
    }
}

impl DrawImage for MockDisplay {
    fn draw_img(&mut self, img: &[u8], offset: Point) -> Result<(), CoreError> {
        // Safety: the img format has been defined in terms of compatibility, so the conversion is ok here
        let img16 =
            unsafe { core::slice::from_raw_parts(img.as_ptr() as *const u16, img.len() / 2) };
        // At the moment we only know format 1
        assert!(img16[0] == 1);

        // The image is really built for our display?
        assert!(img16[1] == DISPLAY_WIDTH as u16);
        assert!(img16[2] + offset.y as u16 <= DISPLAY_HEIGHT as u16);

        // Let's write the pixels
        let color_cnt = img16[3];
        let mut idx = 4;
        for _ in 0..color_cnt {
            let color = Colors::from(img16[idx] as u8);
            let px_cnt = img16[idx + 1] as usize;
            idx += 2;
            for idx in idx..idx + px_cnt {
                let i_idx = img16[idx];
                let y = i_idx / (DISPLAY_WIDTH as u16);
                let x = i_idx as u16 - y * DISPLAY_WIDTH as u16;
                let p = Point::new(offset.x + x as i32, offset.y + y as i32);
                let _ = Pixel(p, color).draw(self);
            }
            idx += px_cnt;
        }
        Ok(())
    }
}

impl DrawTarget for MockDisplay {
    type Color = Colors;
    type Error = CoreError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.0.draw_iter(pixels).unwrap();
        Ok(())
    }
}

impl OriginDimensions for MockDisplay {
    fn size(&self) -> Size {
        self.0.size()
    }
}

fn simulate(
    core_model: &mut CoreModel,
    climb: &mut f32,
    average_buffer: &mut AllocRingBuffer<f32>,
    opt_climb: &mut f32,
    climb_end: f32,
    wind_speed: &mut f32,
    wind_angle: &mut Angle,
    speed_to_fly_dif: &mut f32,
    time: f32,
    rng: &mut rand::rngs::ThreadRng,
    display: &mut MockDisplay,
    window: &mut Window,
) {
    println!("climb_end: {:.1}, time: {:.1}", climb_end, time);

    let mut wind_incr = 0.05;
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
        if *wind_speed > 10.0 {
            wind_incr = -1.0 * wind_incr;
        }

        *speed_to_fly_dif += speed_to_fly_incr;
        if *speed_to_fly_dif > 50.0 {
            speed_to_fly_incr = -0.5;
        }
        if *speed_to_fly_dif < -50.0 {
            speed_to_fly_incr = 0.5;
        }

        if *speed_to_fly_dif > 0.0 {
            core_model.modes.fly_mode = FlyMode::Circling;
        } else {
            core_model.modes.fly_mode = FlyMode::StraightFlight;
        }

        if wind_incr > 0.0 {
            core_model.modes.vario_mode = VarioMode::SpeedToFly;
        } else {
            core_model.modes.vario_mode = VarioMode::Vario;
        }

        // set values and draw display
        core_model.measured.average_climb_rate = average_climb.m_s();
        core_model.measured.climb_rate = (*opt_climb).m_s();
        core_model.measured.wind_angle = *wind_angle;
        core_model.measured.wind_speed = (*wind_speed).m_s();
        core_model.calculated.speed_to_fly_dif = (*speed_to_fly_dif).km_h();

        draw_view(display, core_model).unwrap();

        window.update(&display.0);

        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    const GLIDER_DATA: BasicGliderData = BasicGliderData {
        // 0
        name: "LS-3 WL",
        wing_area: 10.5,
        max_speed: 270.0,
        empty_mass: 280.0,
        max_ballast: 121.0,
        reference_weight: 396.0,
        handicap: 107,
        polar_values: [[80.0, -0.604], [105.0, -0.700], [180.0, -1.939]],
    };

    let mut polar = Polar::new(&GLIDER_DATA);
    assert_float_eq!(
        polar.gliding_ratio(AirSpeed::from_tas_at_nn(101.86.km_h())),
        41.68
    );
    assert_float_eq!(polar.min_sink_speed().tas.to_km_h(), 74.77);
    assert_float_eq!(
        polar.gliding_ratio(AirSpeed::from_tas_at_nn(180.0.km_h())),
        24.56
    );

    assert_float_eq!(
        polar
            .sink_rate(AirSpeed::from_tas_at_nn(90.0.km_h()))
            .to_m_s(),
        -0.613
    );
    assert_float_eq!(
        polar
            .sink_rate(AirSpeed::from_tas_at_nn(135.0.km_h()))
            .to_m_s(),
        -1.059
    );
    assert_float_eq!(
        polar
            .sink_rate(AirSpeed::from_tas_at_nn(200.0.km_h()))
            .to_m_s(),
        -2.64
    );

    assert_float_eq!(
        polar.speed_to_fly(0.0.m_s(), 0.0.m_s()).tas.to_km_h(),
        100.2
    );
    assert_float_eq!(
        polar.speed_to_fly(0.62.m_s(), 0.0.m_s()).tas.to_km_h(),
        74.77
    );
    assert_float_eq!(
        polar.speed_to_fly(1.0.m_s(), 1.0.m_s()).tas.to_km_h(),
        100.2
    );
    assert_float_eq!(
        polar.speed_to_fly(-1.0.m_s(), 0.0.m_s()).tas.to_km_h(),
        132.9
    );
    assert_float_eq!(
        polar.speed_to_fly(-2.0.m_s(), 0.0.m_s()).tas.to_km_h(),
        159.0
    );
    assert_float_eq!(
        polar.speed_to_fly(-3.0.m_s(), 0.0.m_s()).tas.to_km_h(),
        181.4
    );
    assert_float_eq!(
        polar.speed_to_fly(10.0.m_s(), 0.0.m_s()).tas.to_km_h(),
        74.77
    );
    assert_float_eq!(
        polar.speed_to_fly(-99.0.m_s(), 0.0.m_s()).tas.to_km_h(),
        270.0
    );

    polar.set_water_ballast(121.0.kg());
    assert_float_eq!(
        polar.speed_to_fly(0.0.m_s(), 0.0.m_s()).tas.to_km_h(),
        115.4
    );
    assert_float_eq!(
        polar.speed_to_fly(-3.0.m_s(), 0.0.m_s()).tas.to_km_h(),
        199.15
    );

    polar.set_water_ballast(0.0.kg());
    assert_float_eq!(
        polar.speed_to_fly(0.0.m_s(), 0.0.m_s()).tas.to_km_h(),
        100.2
    );

    polar.set_pilot_weight(120.0.kg());
    assert_float_eq!(
        polar.speed_to_fly(0.0.m_s(), 0.0.m_s()).tas.to_km_h(),
        104.2
    );

    polar.set_pilot_weight(90.0.kg());
    assert_float_eq!(
        polar.speed_to_fly(0.0.m_s(), 0.0.m_s()).tas.to_km_h(),
        100.2
    );

    polar.set_empty_weight(260.0.kg());
    assert_float_eq!(
        polar.speed_to_fly(0.0.m_s(), 0.0.m_s()).tas.to_km_h(),
        97.4
    );

    polar.set_empty_weight(280.0.kg());
    assert_float_eq!(
        polar.speed_to_fly(0.0.m_s(), 0.0.m_s()).tas.to_km_h(),
        100.2
    );

    polar.set_bugs(1.1);
    assert_float_eq!(
        polar.gliding_ratio(AirSpeed::from_tas_at_nn(105.0.km_h())),
        37.7
    );

    polar.set_bugs(1.0);
    assert_float_eq!(
        polar.gliding_ratio(AirSpeed::from_tas_at_nn(105.0.km_h())),
        41.5
    );

    polar.set_density(0.913.kg_m3());
    let speed = polar.speed_to_fly(0.0.m_s(), 0.0.m_s());
    assert_float_eq!(speed.ias.to_km_h(), 100.2);
    assert_float_eq!(speed.tas.to_km_h(), 116.0);

    polar.set_density(Density::AT_NN());
    assert_float_eq!(
        polar.speed_to_fly(0.0.m_s(), 0.0.m_s()).tas.to_km_h(),
        100.2
    );

    let mut display = MockDisplay::new(Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT));
    let mut window = Window::new("Vario - Mock", &OutputSettings::default());

    let mut climb: f32 = 0.0;
    let mut average_buffer = AllocRingBuffer::with_capacity(256);
    average_buffer.fill_with(|| 0.0f32);
    let mut opt_climb: f32 = 0.0;

    let mut wind_angle = 150.0.deg();
    let mut wind_speed: f32 = 1.0;

    let mut speed_to_fly_dif: f32 = 0.0;

    let mut rng = rand::thread_rng();
    let mut blackboard = CoreModel::default();
    blackboard.calculated.mc_cready = 1.7.m_s();
    blackboard.measured.average_wind_speed = 14.0.km_h();
    blackboard.measured.average_wind_angle = 330.0.deg();

    for _ in 1..20 {
        let climb_end = rng.gen_range(-1.5..4.5) as f32;
        let time = rng.gen_range(1.0..5.0) as f32;
        simulate(
            &mut blackboard,
            &mut climb,
            &mut average_buffer,
            &mut opt_climb,
            climb_end,
            &mut wind_speed,
            &mut wind_angle,
            &mut speed_to_fly_dif,
            time,
            &mut rng,
            &mut &mut display,
            &mut window,
        );

        let time = rng.gen_range(2.0..30.0) as f32;
        simulate(
            &mut blackboard,
            &mut climb,
            &mut average_buffer,
            &mut opt_climb,
            climb_end,
            &mut wind_speed,
            &mut wind_angle,
            &mut speed_to_fly_dif,
            time,
            &mut rng,
            &mut display,
            &mut window,
        );
    }

    Ok(())
}
