/// These approximation equations were created using the Python library 'ambiance' as a reference,
/// which aligns the ISA tables. Ambiance is a full implementation of the ICAO standard atmosphere
/// 1993 written in Python.
///
/// - see also 'fit.ipynb' in the doc directory
/// - see [Ambiance](https://pypi.org/project/ambiance/)
///
use crate::system_of_units::{Density, Float, Length, Pressure};

#[inline]
fn horner(x: Float, coefs: &[Float]) -> Float {
    let mut accu = coefs[0];
    for coef in &coefs[1..] {
        accu = accu * x + coef;
    }
    accu
}

// Error below 0.01% between 0..10_000m
pub fn pressure(altitude: Length) -> Pressure {
    const COEFS: [Float; 4] = [
        -1.08815563e-08,
        5.57230085e-04,
        -1.19665733e+01,
        1.01302034e+05,
    ];
    Pressure(horner(altitude.0, &COEFS))
}

// Error below 0.02% between 0..10_000m
pub fn density(altitude: Length) -> Density {
    const COEFS: [Float; 4] = [
        -6.38071857e-14,
        4.26830074e-09,
        -1.17451181e-04,
        1.22492440e+00,
    ];
    Density(horner(altitude.0, &COEFS))
}

/* Not used at the moment
// Error below 0.01% between 0..10_000m
pub fn temperature(altitude: Length) -> Temperature {
    const COEFS: [Float; 2] = [-6.48978914e-03, 2.88133007e+02];
    Temperature(horner(altitude.0, &COEFS))
}*/

// Error below 4m between 0..10_000m
pub fn altitude(static_air_pressure: Pressure) -> Length {
    const COEFS: [Float; 6] = [
        -1.56145960e-21,
        6.09947157e-16,
        -9.88928010e-11,
        8.84184638e-06,
        -5.46163448e-01,
        1.98197754e+04,
    ];
    Length(horner(static_air_pressure.0, &COEFS))
}

#[derive(Clone, Copy)]
pub struct PressureAltitude {
    qnh_ref: Pressure,
    qfe_ref: Pressure,
    static_pressure: Pressure,
}

impl Default for PressureAltitude {
    fn default() -> Self {
        PressureAltitude {
            qnh_ref: Pressure::AT_NN(),
            qfe_ref: Pressure::AT_NN(),
            static_pressure: Pressure::AT_NN(),
        }
    }
}

impl PressureAltitude {
    /// returns QNH altitude (altitude above given QNH pressure level)
    pub fn qnh_altitude(&self) -> Length {
        let pressure = self.static_pressure - (self.qnh_ref - Pressure::AT_NN());
        altitude(pressure)
    }

    /// returns QNH
    pub fn qnh(&self) -> Pressure {
        return self.qnh_ref;
    }

    /// set QNH
    pub fn set_qnh(&mut self, qnh_pressure: Pressure) {
        self.qnh_ref = qnh_pressure
    }

    /// returns QFE altitude (altitude above given QFE pressure level)
    pub fn qfe_altitude(&self) -> Length {
        let pressure = self.static_pressure - (self.qfe_ref - Pressure::AT_NN());
        altitude(pressure)
    }

    /// returns QFE
    pub fn qfe(&self) -> Pressure {
        return self.qfe_ref;
    }

    /// set QFE
    pub fn set_qfe(&mut self, qfe_pressure: Pressure) {
        self.qfe_ref = qfe_pressure
    }

    /// returns altitude ISA pressure level 1013.25 hPa
    pub fn qne_altitude(&self) -> Length {
        altitude(self.static_pressure)
    }

    /// set static pressure
    pub fn set_static_pressure(&mut self, pressure: Pressure) {
        self.static_pressure = pressure
    }
}
