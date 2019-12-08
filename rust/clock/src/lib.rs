use std::fmt;

#[derive(Debug, Eq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    fn hour(&self) -> i32 {
        self.minutes.wrapping_div_euclid(60).wrapping_rem_euclid(24)
    }

    fn minute(&self) -> i32 {
        self.minutes.wrapping_rem_euclid(60)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour(), self.minute())
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: hours * 60 + minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            minutes: self.minutes + minutes,
        }
    }
}
