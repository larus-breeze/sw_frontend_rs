use core::{cmp::Ordering, mem::transmute};
use heapless::String;
use crate::tformat;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl Date {
    /// Create Date
    pub const fn new(year: u16, month: u8, day: u8) -> Self {
        Date { year, month, day }
    }

    pub fn from_array_4u8(&mut self, data: [u8; 4]) {
        // unsafe is ok, because this is infallible
        let date = unsafe { transmute::<[u8; 4], Self>(data) };
        *self = date;
    }

    pub fn from_u32(val: u32) -> Self {
        let day = (val & 0x000f) as u8;
        let month = ((val & 0x00f0) >> 8) as u8;
        let year = ((val & 0xff00) >> 16) as u16;
        Date { year, month, day }
    }

    pub fn as_array_4u8(&self) -> [u8; 4] {
        // unsafe is ok, because this is infallible
        let r = unsafe { transmute::<&Self, &[u8; 4]>(self) };
        *r
    }

    pub fn as_string(&self) -> String<10> {
        let year = self.year;
        let month = self.month;
        let day = self.day;
        match tformat!(10, "{:04}-{:02}-{:02}", year, month, day) {
            Ok(s) => s,
            Err(_) => tformat!(10, "Date Error").unwrap()
        }
    }

    pub fn as_u32(&self) -> u32 {
        (self.year as u32) << 16 | 
        (self.month as u32) << 8 |
        (self.day as u32)
    }

    pub fn add_days(&mut self, days: i32) {
        let mut days_since_epoch = self.days_since_epoch();
        days_since_epoch += days;
        *self = Self::from_days_since_epoch(days_since_epoch);
    }

    pub fn days_since(&self, other: &Self) -> i32 {
        let self_since_epoch = self.days_since_epoch();
        let other_since_epoch = other.days_since_epoch();
        other_since_epoch - self_since_epoch
    }

    fn from_days_since_epoch(days: i32) -> Self {
        // 1. Calculate the era (400-year cycle)
        // 146097 is the number of days in 400 years        
        let n = days - 1;
        let era = n / 146097;
        let doe = n % 146097; // Day of Era

        // 2. Calculate the year within the era
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let year = yoe + era * 400 + 1;
        
        // 3. Loop-free Month and Day calculation
        let prev_y = year - 1;
        let doy = days - 365 * prev_y + prev_y / 4 - prev_y / 100 + prev_y / 400 + 1;

        let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
        let march_1_offset = if is_leap { 60 } else { 59 };

        // Determine "virtual" days since March 1st
        let (v_day, v_year) = if doy >= march_1_offset {
            (doy - march_1_offset, year)
        } else {
            // Current date is Jan or Feb; belongs to the previous "virtual" yea
            (doy + 306 + if is_leap { 1 } else { 0 }, year - 1)
        };

        // Magic formula: (5 * days + 2) / 153
        let v_month = (5 * v_day + 2) / 153;
        
        let day = v_day - (153 * v_month + 2) / 5 + 1;
        let month = if v_month < 10 { v_month + 3 } else { v_month - 9 };
        let actual_year = if v_month >= 10 { v_year + 1 } else { v_year };

        Self::new(actual_year as u16, month as u8, day as u8)
    }


    // Calculate the days since epoch
    fn days_since_epoch(&self) -> i32 {
        let mut y = self.year as i32;
        let mut m = self.month as i32;
        let d = self.day as i32;
        
        if m <= 2 {
            y -= 1;
            m += 12;
        }
        
        // The mathematical essence of the Gregorian calendar:
        (365 * y) 
        + (y / 4) 
        - (y / 100) 
        + (y / 400) 
        + (306 * (m + 1) / 10) 
        - 428 
        + d
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_u32().cmp(&other.as_u32())
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, NaiveDate};

    #[test]
    fn test_compare_with_chrono() {
        // From 2000-01-01 to 2099-12-31
        let start_chrono = NaiveDate::from_ymd_opt(3000, 1, 1).unwrap();
        let end_year = 2100;

        let mut current_chrono = start_chrono;
        let start_date = Date::new(2000, 1, 1);
        let mut date = start_date;

        while current_chrono.year() < end_year {
            let days_diff_chrono = current_chrono.signed_duration_since(start_chrono).num_days() as i32;
            let days_diff = date.days_since(&start_date);
            
            assert_eq!(
                days_diff, 
                days_diff_chrono, 
                "Error while calulating the difference",
            );

            // next day
            current_chrono = current_chrono.succ_opt().unwrap();
            date.add_days(1);
        }
    }

    #[test]
    fn test_less_equal_greater() {
        assert!(Date::new(2000,1,1) < Date::new(2001,1,1));
        assert!(Date::new(2030,5,3) < Date::new(2030,6,1));
        assert!(Date::new(2045,5,3) < Date::new(2045,5,4));
        assert!(Date::new(2045,5,3) <= Date::new(2045,5,3));
        assert!(Date::new(2045,5,3) <= Date::new(2045,5,4));
        assert!(Date::new(2045,12,12) == Date::new(2045,12,12));
        assert!(Date::new(2045,12,13) > Date::new(2045,12,12));
        assert!(Date::new(2045,12,12) >= Date::new(2045,12,12));
        assert!(Date::new(2045,12,13) >= Date::new(2045,12,12));
    }
}