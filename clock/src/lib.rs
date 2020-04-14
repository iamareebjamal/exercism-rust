#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

#[inline]
fn modulo(n: i32, m: i32) -> i32 {
    (n % m + m) % m
}

#[inline]
fn adjust_hours(minutes: i32) -> i32 {
    if minutes < 0 {
        -1
    } else {
        0
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let extra_hours = minutes / 60 + adjust_hours(minutes % 60);
        Clock {
            hours: modulo(hours + extra_hours, 24),
            minutes: modulo(minutes, 60)
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}
