use heapless::String;
use tfmt::{uDisplayFormatted, uWrite, Formatter, Padding, Convert};

#[derive(Clone, Copy, Debug)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct Time {
    hour: u8,
    min: u8,
    sec: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct DateTime {
    date: Date,
    time: Time,
}

impl DateTime {
    pub const fn new() -> Self {
        DateTime {
            date: Date { year: 2000, month: 1, day: 1 },
            time: Time { hour: 0, min: 0, sec: 0 }
        }
    }

    pub fn set_date(&mut self, year: u16, month: u8, day: u8) {
        self.date.year = year;
        self.date.month = month;
        self.date.day = day;
    }

    pub fn set_time(&mut self, hour: u8, min: u8, sec: u8) {
        self.time.hour = hour;
        self.time.min = min;
        self.time.sec = sec;
    }

    pub fn set_date_time(&mut self, year: u16, month: u8, day: u8, hour: u8, min: u8, sec: u8) {
        self.set_date(year, month, day);
        self.set_time(hour, min, sec);
    }

    fn insert(buf: &mut [u8], mut val: u16, mut idx: usize) {
        while val > 0 {
            idx -= 1;
            buf[idx] = (val % 10) as u8 + b'0';
            val /= 10;
        }
    }
    
    pub fn to_bytes(&self) -> [u8; 20] {
        let mut buf = [0u8; 20];
        buf.copy_from_slice(b"0000-00-00T00:00:00Z");
        Self::insert(&mut buf, self.date.year, 4);
        Self::insert(&mut buf, self.date.month as u16, 7);
        Self::insert(&mut buf, self.date.day as u16, 10);
        Self::insert(&mut buf, self.time.hour as u16, 13);
        Self::insert(&mut buf, self.time.min as u16, 16);
        Self::insert(&mut buf, self.time.sec as u16, 19);
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

    pub fn date(&self) -> &Date {
        &self.date
    }

    pub fn time(&self) -> &Time {
        &self.time
    }
}

impl uDisplayFormatted for &Date {
    fn fmt_formatted<W>(
        &self,
        fmt: &mut Formatter<'_, W>,
        _prefix: bool,
        cmd: char,
        _padding: Padding,
        _pad_char: char,
        _behind: usize
    ) -> Result<(), W::Error>
       where W: uWrite + ?Sized {
        if cmd == 'n' {
            let mut conv = Convert::<6>::new(b'0');
            conv.u32_pad(self.year as u32 - 2000, 2).unwrap();
            conv.u32_pad(self.month as u32, 2).unwrap();
            conv.u32_pad(self.day as u32, 2).unwrap();
            fmt.write_str(conv.as_str())
    
            } else {
                fmt.write_str("FormatError")
            }
    }
}

impl uDisplayFormatted for &Time {
    fn fmt_formatted<W>(
        &self,
        fmt: &mut Formatter<'_, W>,
        _prefix: bool,
        cmd: char,
        _padding: Padding,
        _pad_char: char,
        _behind: usize
    ) -> Result<(), W::Error>
       where W: uWrite + ?Sized {
        if cmd == 'n' {
            let mut conv = Convert::<9>::new(b'0');
            conv.write_str(".00").unwrap();
            conv.u32_pad(self.sec as u32, 2).unwrap();
            conv.u32_pad(self.min as u32, 2).unwrap();
            conv.u32_pad(self.hour as u32, 2).unwrap();
            fmt.write_str(conv.as_str())
        } else {
            fmt.write_str("FormatError")
        }
    }
}