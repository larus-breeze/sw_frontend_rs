use crate::tformat;
use core::{cmp::Ordering, mem::transmute};
use heapless::String;

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
        let day = (val & 0xff) as u8;
        let month = ((val >> 8) & 0xff) as u8;
        let year = ((val >> 16) & 0xffff) as u16;
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
            Err(_) => tformat!(10, "Date Error").unwrap(),
        }
    }

    pub fn as_u32(&self) -> u32 {
        (self.year as u32) << 16 | (self.month as u32) << 8 | (self.day as u32)
    }

    pub fn add_days(&mut self, days: i32) {
        let mut days_since_epoch = self.days_since_epoch();
        days_since_epoch += days;
        let next_date = Self::from_days_since_epoch(days_since_epoch);
        *self = next_date;
    }

    pub fn days_since(&self, other: &Self) -> i32 {
        let self_since_epoch = self.days_since_epoch();
        let other_since_epoch = other.days_since_epoch();
        self_since_epoch - other_since_epoch
    }

    fn from_days_since_epoch(days: i32) -> Self {
        // 1. Calculate the number of 400-year cycles (eras) that have elapsed
        let n = days - 1;
        let eras = n / 146097;
        let day_of_era = n % 146097;

        // 2. Calculate the year within the era
        // We take leap years into account: 4, 100, 400
        let y_of_era =
            (day_of_era - day_of_era / 1460 + day_of_era / 36524 - day_of_era / 146096) / 365;
        let year = eras * 400 + y_of_era + 1;

        // 3. Calculate the day of the year
        let prev_y = year - 1;
        let rdn_at_start_of_year = 365 * prev_y + prev_y / 4 - prev_y / 100 + prev_y / 400 + 1;
        //let day_of_year = days - rdn_at_start_of_year;
        let mut remaining_days = days - rdn_at_start_of_year;

        // 4. Work out the month and day
        if remaining_days > 59 && !((year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)) {
            // Turn a non-leap year into a leap year
            remaining_days += 1;
        }

        const DAYS_TIL_MONTH: [i32; 13] =
            [0, 0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];

        let mut month = 12;
        while month > 1 {
            if DAYS_TIL_MONTH[month] <= remaining_days {
                break;
            }
            month -= 1;
        }
        let day = remaining_days - DAYS_TIL_MONTH[month] + 1;

        Date::new(year as u16, month as u8, day as u8)
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
        (365 * y) + (y / 4) - (y / 100) + (y / 400) + (306 * (m + 1) / 10) - 428 + d
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
        let start_chrono = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
        let end_year = 3000;

        let mut current_chrono = start_chrono;
        let start_date = Date::new(2000, 1, 1);
        let mut date = start_date;

        while current_chrono.year() < end_year {
            let days_diff_chrono = current_chrono
                .signed_duration_since(start_chrono)
                .num_days() as i32;
            let days_diff = date.days_since(&start_date);

            assert_eq!(
                days_diff, days_diff_chrono,
                "Error while calulating the difference",
            );

            // next day
            current_chrono = current_chrono.succ_opt().unwrap();
            date.add_days(1);
        }
    }

    #[test]
    fn test_less_equal_greater() {
        // year, month, day
        assert!(Date::new(2000, 1, 1) < Date::new(2001, 1, 1));
        assert!(Date::new(2030, 5, 3) < Date::new(2030, 6, 1));
        assert!(Date::new(2045, 5, 3) < Date::new(2045, 5, 4));

        // smaller, smaller than equal to
        assert!(Date::new(2045, 5, 3) <= Date::new(2045, 5, 3));
        assert!(Date::new(2045, 5, 3) <= Date::new(2045, 5, 4));

        // equal
        assert!(Date::new(2045, 12, 12) == Date::new(2045, 12, 12));

        // greater, greater than equal to
        assert!(Date::new(2045, 12, 13) > Date::new(2045, 12, 12));
        assert!(Date::new(2045, 12, 12) >= Date::new(2045, 12, 12));
        assert!(Date::new(2045, 12, 13) >= Date::new(2045, 12, 12));

        // invers
        assert!(!(Date::new(2045, 12, 13) < Date::new(2045, 12, 12)));
        assert!(!(Date::new(2045, 5, 3) > Date::new(2045, 5, 4)));
    }

    #[test]
    fn test_from_to_u32() {
        let u32 = 0x07e7_0612;
        let date = Date::from_u32(u32);
        assert!(date == Date::new(2023, 6, 18));

        let u32_copy = date.as_u32();
        assert!(u32 == u32_copy);
    }
}
