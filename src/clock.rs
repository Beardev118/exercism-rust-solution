use std::fmt;
const DAY: i64 = 24 * 60;
const HOUR: i64 = 60;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i64,
}

impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Clock {
        Clock {
            minutes: (((hours * HOUR + minutes) % DAY) + DAY) % DAY,
        }
    }
    pub fn add_minutes(self, minutes: i64) -> Clock {
        Clock::new(0, self.minutes + minutes)
    }
}
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / HOUR, self.minutes % HOUR)
    }
}

// ****
#[derive(Default, Eq, PartialEq, Debug)]
pub struct Clock {
    h: i32,
    m: i32,
}

impl Clock {
    pub fn to_string(&self) -> String {
        format!("{:0>2}:{:0>2}", self.h, self.m)
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hr = ((hours + minutes / 60) % 24 + 24) % 24;
        let mut mm = minutes % 60;
        if mm < 0 {
            hr = (hr + 23) % 24;
        }
        mm = (mm + 60) % 60;
        Self { h: hr, m: mm }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut hr = ((self.h + (self.m + minutes) / 60) % 24 + 24) % 24;
        let mut mm = (self.m + minutes) % 60;
        if mm < 0 {
            hr = (hr + 23) % 24;
        }
        mm = (mm + 60) % 60;
        Self { h: hr, m: mm }
    }
}
