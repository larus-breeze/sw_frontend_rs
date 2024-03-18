use heapless::String;

#[derive(Clone, Copy, Debug)]
pub struct DateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    min: u8,
    sec: u8,
}

impl DateTime {
    pub const fn new() -> Self {
        DateTime {
            year: 2000,
            month: 1,
            day: 1,
            hour: 0,
            min: 0,
            sec: 0,
        }
    }

    pub fn set_date(&mut self, year: u16, month: u8, day: u8) {
        self.year = year;
        self.month = month;
        self.day = day;
    }

    pub fn set_time(&mut self, hour: u8, min: u8, sec: u8) {
        self.hour = hour;
        self.min = min;
        self.sec = sec;
    }

    pub fn set_date_time(&mut self, year: u16, month: u8, day: u8, hour: u8, min: u8, sec: u8) {
        self.set_date(year, month, day);
        self.set_time(hour, min, sec);
    }

    pub fn to_bytes(&self) -> [u8; 20] {
        fn insert(buf: &mut [u8; 20], mut val: u16, mut idx: usize) {
            while val > 0 {
                idx -= 1;
                buf[idx] = (val % 10) as u8 + b'0';
                val /= 10;
            }
        }
        let mut buf = [0u8; 20];
        buf.copy_from_slice(b"0000-00-00T00:00:00Z");
        insert(&mut buf, self.year, 4);
        insert(&mut buf, self.month as u16, 7);
        insert(&mut buf, self.day as u16, 10);
        insert(&mut buf, self.hour as u16, 13);
        insert(&mut buf, self.min as u16, 16);
        insert(&mut buf, self.sec as u16, 19);
        buf
    }

    pub fn to_string(&self) -> String<20> {
        let bytes = self.to_bytes();
        let mut r = String::<20>::new();
        for c in bytes {
            r.push(c as char).unwrap();
        }
        r
    }
}
