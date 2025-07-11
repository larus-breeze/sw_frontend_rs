use crate::flight_physics::{polar_store::BasicGliderData, AirSpeed};
use crate::system_of_units::{Density, Float, FloatToMass, Mass, Speed};

#[allow(unused_imports)]
use micromath::F32Ext;

#[derive(Clone, Debug)]
pub struct PolarKoefs {
    pub a: Float,
    pub b: Float,
    pub c: Float,
    pub v_min: Float,
    pub weight: Float,
}

#[derive(Clone, Copy)]
pub struct GliderData {
    pub pilot_weight: Mass,
    pub water_ballast: Mass,
    pub bugs: f32,
    pub basic_glider_data: BasicGliderData,
}

impl Default for GliderData {
    fn default() -> Self {
        GliderData {
            pilot_weight: 90.0.kg(),
            water_ballast: 0.0.kg(),
            bugs: 1.0,
            basic_glider_data: BasicGliderData::default(),
        }
    }
}

impl GliderData {
    pub fn ballast_fraction(&self) -> f32 {
        self.water_ballast.to_kg() / self.basic_glider_data.max_ballast
    }

    pub fn set_ballast_fraction(&mut self, fraction: f32) {
        self.water_ballast = (fraction * self.basic_glider_data.max_ballast).kg();
    }
}
pub struct Polar {
    max_speed: Float,     // m/s
    density_ratio: Float, // -
    curr: PolarKoefs,     // current koefs
    refer: PolarKoefs,    // reference coefs
}

impl Default for Polar {
    fn default() -> Self {
        Self {
            curr: PolarKoefs {
                a: 0.0,
                b: 0.0,
                c: 0.0,
                v_min: 0.0,
                weight: 0.0,
            },
            refer: PolarKoefs {
                a: 0.0,
                b: 0.0,
                c: 0.0,
                v_min: 0.0,
                weight: 0.0,
            },
            max_speed: 0.0,
            density_ratio: 1.0,
        }
    }
}

/// Simple model of the polar curve for a glider
///
/// The polar of the glider is mapped with a simple model, a quadratic approximation. The
/// coefficients are calculated from 3 points of the polar curve.
///
/// If the pilot weight, water ballast, empty weight, pollution of the glider with mosquitoes
/// or air density changes, the polar curve is recalculated. Airspeeds are output as type
/// [AirSpeed], which contains both TAS and IAS.
impl Polar {
    /// calc polar coefficients
    pub fn recalc_glider(&mut self, glider_data: &GliderData) {
        let bgd = &glider_data.basic_glider_data;
        let (v1, w1) = (bgd.polar_values[0][0] / 3.6, bgd.polar_values[0][1]);
        let (v2, w2) = (bgd.polar_values[1][0] / 3.6, bgd.polar_values[1][1]);
        let (v3, w3) = (bgd.polar_values[2][0] / 3.6, bgd.polar_values[2][1]);

        let a = ((v2 - v3) * (w1 - w3) + (v3 - v1) * (w2 - w3))
            / (v1 * v1 * (v2 - v3) + v2 * v2 * (v3 - v1) + v3 * v3 * (v1 - v2));
        let b = (w2 - w3 - a * (v2 * v2 - v3 * v3)) / (v2 - v3);
        let c = w3 - a * v3 * v3 - b * v3;

        let v_min = -b / a / 2.0;
        let weight = bgd.reference_weight;

        self.curr = PolarKoefs {
            a,
            b,
            c,
            v_min,
            weight,
        };
        self.refer = self.curr.clone();
        self.max_speed = bgd.max_speed / 3.6;
        self.density_ratio = 1.0;
    }

    /// recalc polar to adopt weight and density changes
    pub fn recalc(&mut self, glider_data: &GliderData, density: Density) {
        let weight = (glider_data.basic_glider_data.empty_mass.kg()
            + glider_data.pilot_weight
            + glider_data.water_ballast)
            .to_kg();
        let ratio_weight = (weight / self.refer.weight).sqrt();
        self.density_ratio = (Density::AT_NN().0 / density.to_kg_m3()).sqrt();
        let ratio = ratio_weight * self.density_ratio;

        self.curr.a = glider_data.bugs * self.refer.a / ratio;
        self.curr.b = glider_data.bugs * self.refer.b;
        self.curr.c = glider_data.bugs * self.refer.c * ratio;

        self.curr.v_min = self.refer.v_min * ratio;
        self.curr.weight = weight;
    }

    /// Returns the sink rate of the glider
    ///
    /// It is checked that the speed is within the permissible range.
    pub fn sink_rate(&self, speed: AirSpeed) -> Speed {
        let v = self.clamp_speed(speed.tas().to_m_s());
        let sink_rate = v * v * self.curr.a + v * self.curr.b + self.curr.c;
        Speed(sink_rate)
    }

    /// Returns the speed for minimal sink
    pub fn min_sink_speed(&self) -> AirSpeed {
        let v = self.clamp_speed(-self.curr.b / self.curr.a / 2.0);
        self.airspeed_from_tas(v)
    }

    /// Returns the speed to fly, which is a function of the expected climb and the metereological
    /// sink.
    pub fn speed_to_fly(&self, si_met: Speed, st_mc_cready: Speed) -> AirSpeed {
        let (met, mc_cready) = (si_met.to_m_s(), st_mc_cready.to_m_s());
        let val = (self.curr.c + met - mc_cready) / self.curr.a;
        let stf = if val > 0.0 { val.sqrt() } else { 0.0 };
        let stf = self.clamp_speed(stf);
        self.airspeed_from_tas(stf)
    }

    /// Returns the current possible minimum speed
    pub fn v_min(&self) -> AirSpeed {
        self.airspeed_from_tas(self.curr.v_min)
    }

    /// Returns the gliding ratio
    pub fn gliding_ratio(&self, speed: AirSpeed) -> Float {
        let v = self.clamp_speed(speed.tas().to_m_s());
        let sink_rate = v * v * self.curr.a + v * self.curr.b + self.curr.c;
        -v / sink_rate
    }

    fn clamp_speed(&self, speed: Float) -> Float {
        match speed {
            v if v > self.max_speed => self.max_speed,
            v if v < self.curr.v_min => self.curr.v_min,
            v if v.is_nan() => self.curr.v_min,
            _ => speed,
        }
    }

    fn airspeed_from_tas(&self, tas: Float) -> AirSpeed {
        let ias = tas / self.density_ratio;
        AirSpeed::new(ias, tas)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_float_eq;
    use crate::{FloatToDensity, FloatToMass, FloatToSpeed};
    use std::{fs::File, io::*, vec::Vec};

    const LS3_GLIDER_DATA: BasicGliderData = BasicGliderData {
        // 0
        name: "LS-3 WL", // D-2817, erste Winglets
        wing_area: 10.5,
        max_speed: 270.0,
        empty_mass: 280.0,
        max_ballast: 121.0,
        reference_weight: 396.0,
        handicap: 107,
        polar_values: [[80.0, -0.604], [105.0, -0.700], [180.0, -1.939]],
    };

    const AS33_GLIDER_DATA: BasicGliderData = BasicGliderData {
        // No 167,  Manufacturer's data interpreted by Andreas Westkamp
        name: "AS-33 18m",
        wing_area: 10.00,
        max_speed: 270.0,
        empty_mass: 285.0,
        max_ballast: 220.0,
        reference_weight: 400.0,
        handicap: 122,
        polar_values: [[97.2, -0.511], [111.6, -0.556], [180.0, -1.369]],
    };

    fn write_stf_to_csv(file_name: &str, polar: &mut Polar) {
        fn write_stf_for_mc<W: Write>(f: &mut W, mc: f32, polar: &mut Polar) {
            let mut si_met = 0.0;
            while si_met > -3.1 {
                let stf = polar.speed_to_fly(si_met.m_s(), mc.m_s()).tas().to_km_h();
                writeln!(f, "{:.1};{:.1};{:.0}", mc, si_met, stf).unwrap();
                si_met -= 0.5;
            }
        }

        //let mut now_calculated = File::create(file_name).unwrap();
        let mut now_calculated = Vec::new();
        now_calculated.write(b"mc;si_met;speed_to_fly\n").unwrap();
        write_stf_for_mc(&mut now_calculated, 0.0, polar);
        write_stf_for_mc(&mut now_calculated, 1.0, polar);
        write_stf_for_mc(&mut now_calculated, 2.0, polar);
        write_stf_for_mc(&mut now_calculated, 3.0, polar);

        let mut r = File::open(file_name).unwrap();
        let mut should_be = Vec::new();
        let _ = r.read_to_end(&mut should_be).unwrap();
        assert_eq!(now_calculated, should_be);
    }

    #[test]
    fn test_stf_table() {
        let mut glider_data = GliderData::default();
        glider_data.basic_glider_data = LS3_GLIDER_DATA;
        let mut polar = Polar::default();
        polar.recalc_glider(&glider_data);
        polar.recalc(&glider_data, Density::AT_NN());

        write_stf_to_csv("tests/ls3_wl.csv", &mut polar);

        glider_data.basic_glider_data = AS33_GLIDER_DATA;
        let mut polar = Polar::default();
        polar.recalc_glider(&glider_data);
        polar.recalc(&glider_data, Density::AT_NN());

        write_stf_to_csv("tests/as33.csv", &mut polar);
    }

    #[test]
    fn test_basic_functions() {
        let mut glider_data = GliderData::default();
        glider_data.basic_glider_data = LS3_GLIDER_DATA;

        #[allow(unused_mut)]
        let mut polar = Polar::default();
        polar.recalc_glider(&glider_data);
        polar.recalc(&glider_data, Density::AT_NN());

        assert_float_eq!(
            polar.gliding_ratio(AirSpeed::from_tas_at_nn(101.86.km_h())),
            41.68
        );
        assert_float_eq!(polar.min_sink_speed().tas().to_km_h(), 74.77);
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
            polar.speed_to_fly(0.0.m_s(), 0.0.m_s()).tas().to_km_h(),
            100.2
        );
        assert_float_eq!(
            polar.speed_to_fly(0.62.m_s(), 0.0.m_s()).tas().to_km_h(),
            74.77
        );
        assert_float_eq!(
            polar.speed_to_fly(1.0.m_s(), 1.0.m_s()).tas().to_km_h(),
            100.2
        );
        assert_float_eq!(
            polar.speed_to_fly(-1.0.m_s(), 0.0.m_s()).tas().to_km_h(),
            132.9
        );
        assert_float_eq!(
            polar.speed_to_fly(-2.0.m_s(), 0.0.m_s()).tas().to_km_h(),
            159.0
        );
        assert_float_eq!(
            polar.speed_to_fly(-3.0.m_s(), 0.0.m_s()).tas().to_km_h(),
            181.4
        );
        assert_float_eq!(
            polar.speed_to_fly(10.0.m_s(), 0.0.m_s()).tas().to_km_h(),
            74.77
        );
        assert_float_eq!(
            polar.speed_to_fly(-99.0.m_s(), 0.0.m_s()).tas().to_km_h(),
            270.0
        );

        glider_data.water_ballast = 121.0.kg();
        polar.recalc(&glider_data, Density::AT_NN());
        assert_float_eq!(
            polar.speed_to_fly(0.0.m_s(), 0.0.m_s()).tas().to_km_h(),
            115.4
        );
        assert_float_eq!(
            polar.speed_to_fly(-3.0.m_s(), 0.0.m_s()).tas().to_km_h(),
            199.15
        );

        glider_data.water_ballast = 0.0.kg();
        polar.recalc(&glider_data, Density::AT_NN());
        assert_float_eq!(
            polar.speed_to_fly(0.0.m_s(), 0.0.m_s()).tas().to_km_h(),
            100.2
        );

        glider_data.pilot_weight = 120.0.kg();
        polar.recalc(&glider_data, Density::AT_NN());
        assert_float_eq!(
            polar.speed_to_fly(0.0.m_s(), 0.0.m_s()).tas().to_km_h(),
            104.2
        );

        glider_data.pilot_weight = 90.0.kg();
        polar.recalc(&glider_data, Density::AT_NN());
        assert_float_eq!(
            polar.speed_to_fly(0.0.m_s(), 0.0.m_s()).tas().to_km_h(),
            100.2
        );

        glider_data.basic_glider_data.empty_mass = 260.0;
        polar.recalc(&glider_data, Density::AT_NN());
        assert_float_eq!(
            polar.speed_to_fly(0.0.m_s(), 0.0.m_s()).tas().to_km_h(),
            97.4
        );

        glider_data.basic_glider_data.empty_mass = 280.0;
        polar.recalc(&glider_data, Density::AT_NN());
        assert_float_eq!(
            polar.speed_to_fly(0.0.m_s(), 0.0.m_s()).tas().to_km_h(),
            100.2
        );

        glider_data.bugs = 1.1;
        polar.recalc(&glider_data, Density::AT_NN());
        assert_float_eq!(
            polar.gliding_ratio(AirSpeed::from_tas_at_nn(105.0.km_h())),
            37.7
        );

        glider_data.bugs = 1.0;
        polar.recalc(&glider_data, Density::AT_NN());
        assert_float_eq!(
            polar.gliding_ratio(AirSpeed::from_tas_at_nn(105.0.km_h())),
            41.5
        );

        polar.recalc(&glider_data, 0.913.kg_m3());
        let speed = polar.speed_to_fly(0.0.m_s(), 0.0.m_s());
        assert_float_eq!(speed.ias().to_km_h(), 100.2);
        assert_float_eq!(speed.tas().to_km_h(), 116.0);

        polar.recalc(&glider_data, Density::AT_NN());
        assert_float_eq!(
            polar.speed_to_fly(0.0.m_s(), 0.0.m_s()).tas().to_km_h(),
            100.2
        );
    }
}
