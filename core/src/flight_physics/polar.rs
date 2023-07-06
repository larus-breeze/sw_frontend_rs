use crate::flight_physics::{AirSpeed, BasicGliderData};
use crate::system_of_units::{Density, Float, Mass, Speed};

// use micromath::F32Ext;

#[derive(Clone, Debug)]
pub struct PolarKoefs {
    pub a: Float,
    pub b: Float,
    pub c: Float,
    pub v_min: Float,
    pub weight: Float,
}

pub struct Polar {
    v_max: Float,         // km/Gh
    empty_weight: Float,  // kg
    pilot_weight: Float,  // kg
    water_ballast: Float, // kg
    density: Float,       // kg/m³
    density_ratio: Float, // -
    bugs: Float,          // 1.0
    curr: PolarKoefs,     // current koefs
    refer: PolarKoefs,    // reference coefs
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
    pub fn new(gd: &BasicGliderData) -> Self {
        let (v1, w1) = (gd.polar_values[0][0] / 3.6, gd.polar_values[0][1]);
        let (v2, w2) = (gd.polar_values[1][0] / 3.6, gd.polar_values[1][1]);
        let (v3, w3) = (gd.polar_values[2][0] / 3.6, gd.polar_values[2][1]);

        let a = ((v2 - v3) * (w1 - w3) + (v3 - v1) * (w2 - w3))
            / (v1 * v1 * (v2 - v3) + v2 * v2 * (v3 - v1) + v3 * v3 * (v1 - v2));
        let b = (w2 - w3 - a * (v2 * v2 - v3 * v3)) / (v2 - v3);
        let c = w3 - a * v3 * v3 - b * v3;

        let v_min = -b / a / 2.0;
        let weight = gd.reference_weight;

        let curr = PolarKoefs {
            a,
            b,
            c,
            v_min,
            weight,
        };
        let refer = curr.clone();

        let v_max = gd.max_speed / 3.6;
        let empty_weight = gd.empty_mass;
        let pilot_weight = 90.0;
        let water_ballast = 0.0;
        let density = Density::AT_NN().to_kg_m3();
        let density_ratio = 1.0;
        let bugs = 1.0;

        let mut p = Self {
            v_max,
            empty_weight,
            pilot_weight,
            water_ballast,
            density,
            density_ratio,
            bugs,
            curr,
            refer,
        };
        // empty weight + pilot weight is usualy not equal to reference weight, so recalc the polar
        p.recalc();
        p
    }

    /// Returns the sink rate of the glider
    ///
    /// It is checked that the speed is within the permissible range.
    pub fn sink_rate(&self, speed: AirSpeed) -> Speed {
        let v = self.clamp_speed(speed.tas.to_m_s());
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
        let stf = ((met - mc_cready + self.curr.c) / self.curr.a).sqrt();
        let stf = self.clamp_speed(stf);
        self.airspeed_from_tas(stf)
    }

    /// Set the water ballast mass
    ///
    /// Triggers a recalculation
    pub fn set_water_ballast(&mut self, weight: Mass) {
        self.water_ballast = weight.to_kg();
        self.recalc();
    }

    /// Set the pilot weight
    ///
    /// Triggers a recalculation
    pub fn set_pilot_weight(&mut self, weight: Mass) {
        self.pilot_weight = weight.to_kg();
        self.recalc();
    }

    /// Set the empty weight
    ///
    /// Triggers a recalculation
    pub fn set_empty_weight(&mut self, weight: Mass) {
        self.empty_weight = weight.to_kg();
        self.recalc();
    }

    /// Set the bug ratio
    ///
    /// A value of 1.0 means that there is no degradation in performance. A value of 1.1
    /// increases the sink rate by 10% over the entire speed. Triggers a recalculation
    pub fn set_bugs(&mut self, bugs: Float) {
        self.bugs = bugs;
        self.recalc();
    }

    /// Set the air density
    ///
    /// Triggers a recalculation
    pub fn set_density(&mut self, density: Density) {
        self.density = density.to_kg_m3();
        self.recalc();
    }

    /// Returns the current possible minimum speed
    pub fn v_min(&self) -> AirSpeed {
        self.airspeed_from_tas(self.curr.v_min)
    }

    /// Returns the gliding ratio
    pub fn gliding_ratio(&self, speed: AirSpeed) -> Float {
        let v = self.clamp_speed(speed.tas.to_m_s());
        let sink_rate = v * v * self.curr.a + v * self.curr.b + self.curr.c;
        -v / sink_rate
    }

    fn recalc(&mut self) {
        let weight = self.empty_weight + self.pilot_weight + self.water_ballast;
        let ratio_weight = (weight / self.refer.weight).sqrt();
        self.density_ratio = (Density::AT_NN().0 / self.density).sqrt();
        let ratio = ratio_weight * self.density_ratio;

        self.curr.a = self.bugs * self.refer.a / ratio;
        self.curr.b = self.bugs * self.refer.b;
        self.curr.c = self.bugs * self.refer.c * ratio;

        self.curr.v_min = self.refer.v_min * ratio;
        self.curr.weight = weight;
    }

    fn clamp_speed(&self, speed: Float) -> Float {
        match speed {
            v if v > self.v_max => self.v_max,
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

    const GLIDER_DATA: BasicGliderData = BasicGliderData {
        // 0
        name: "LS-3 WL",    // D-2817, erste Winglets
        wing_area: 10.5,
        max_speed: 270.0,
        empty_mass: 280.0,
        max_ballast: 121.0,
        reference_weight: 396.0,
        handicap: 107,
        polar_values: [[80.0, -0.604], [105.0, -0.700], [180.0, -1.939]],
    };

    #[test]
    fn test_basic_functions() {
        #[allow(unused_mut)]
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
    }
}
