//! Sunrise/sunset solar-position model.
//!
//! Implementation of the "Almanac for Computers, 1990" algorithm
//! (U.S. Naval Observatory, Nautical Almanac Office), as transcribed by
//! Ed Williams:
//!     <https://edwilliams.org/sunrise_sunset_algorithm.htm>
//!
//! The official civil zenith of 90°50' (90.833°) is used.

#[allow(unused_imports)]
use micromath::F32Ext;

const ZENITH_DEG: f32 = 90.833;
const DEG_TO_RAD: f32 = core::f32::consts::PI / 180.0;
const RAD_TO_DEG: f32 = 180.0 / core::f32::consts::PI;

#[derive(Clone, Copy, Debug, Default)]
pub struct SunsetResult {
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
}

pub fn calc_sunset_utc(
    year: u16,
    month: u8,
    day: u8,
    lat_deg: f32,
    lon_deg: f32,
) -> Option<SunsetResult> {
    // Month/day are fully validated by `day_of_year` (leap years, short months).
    // Latitude is capped just shy of the poles to avoid a division by zero in
    // the hour-angle step; both coordinates must be finite.
    if !lat_deg.is_finite() || !lon_deg.is_finite() {
        return None;
    }
    if !(-89.5..=89.5).contains(&lat_deg) || !(-180.0..=180.0).contains(&lon_deg) {
        return None;
    }

    let n = day_of_year(year, month, day)? as f32;
    let lng_hour = lon_deg / 15.0;

    let t = n + (18.0 - lng_hour) / 24.0;
    let m = 0.9856 * t - 3.289;

    let mut l = m + 1.916 * sin_deg(m) + 0.020 * sin_deg(2.0 * m) + 282.634;
    l = norm_deg(l);

    let mut ra = atan_deg(0.91764 * tan_deg(l));
    ra = norm_deg(ra);

    let l_quadrant = (l / 90.0).floor() * 90.0;
    let ra_quadrant = (ra / 90.0).floor() * 90.0;
    ra += l_quadrant - ra_quadrant;
    ra /= 15.0;

    let sin_dec = 0.39782 * sin_deg(l);
    let cos_dec = (1.0 - sin_dec * sin_dec).sqrt();

    let denom = cos_dec * cos_deg(lat_deg);
    if denom.abs() < 1e-6 {
        return None;
    }
    let cos_h = (cos_deg(ZENITH_DEG) - sin_dec * sin_deg(lat_deg)) / denom;

    if !(-1.0..=1.0).contains(&cos_h) {
        return None;
    }

    let h_hours = acos_deg(cos_h) / 15.0;
    let t_local = h_hours + ra - 0.06571 * t - 6.622;
    let ut = norm_hours(t_local - lng_hour);

    Some(hours_to_hms(ut))
}

pub fn format_hhmm(result: SunsetResult) -> heapless::String<5> {
    crate::tformat!(5, "{:02}:{:02}", result.hour, result.min).unwrap()
}

fn is_leap_year(year: u16) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

fn day_of_year(year: u16, month: u8, day: u8) -> Option<u16> {
    let month_lengths = if is_leap_year(year) {
        [31_u16, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31_u16, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let month_idx = month.checked_sub(1)? as usize;
    let max_day = *month_lengths.get(month_idx)? as u8;
    if day == 0 || day > max_day {
        return None;
    }

    let days_before = month_lengths[..month_idx].iter().copied().sum::<u16>();
    Some(days_before + day as u16)
}

fn sin_deg(v: f32) -> f32 {
    (v * DEG_TO_RAD).sin()
}

fn cos_deg(v: f32) -> f32 {
    (v * DEG_TO_RAD).cos()
}

fn tan_deg(v: f32) -> f32 {
    (v * DEG_TO_RAD).tan()
}

fn atan_deg(v: f32) -> f32 {
    v.atan() * RAD_TO_DEG
}

fn acos_deg(v: f32) -> f32 {
    v.acos() * RAD_TO_DEG
}

fn norm_deg(v: f32) -> f32 {
    v.rem_euclid(360.0)
}

fn norm_hours(v: f32) -> f32 {
    v.rem_euclid(24.0)
}

fn hours_to_hms(hours: f32) -> SunsetResult {
    if !hours.is_finite() {
        return SunsetResult::default();
    }
    let total_seconds = (hours * 3600.0).round() as i32;
    let total_seconds = total_seconds.rem_euclid(24 * 3600);
    let hour = (total_seconds / 3600) as u8;
    let min = ((total_seconds % 3600) / 60) as u8;
    let sec = (total_seconds % 60) as u8;
    SunsetResult { hour, min, sec }
}
