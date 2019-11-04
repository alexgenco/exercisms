#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl ToString for Clock {
    fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total = hours * 60 + minutes;
        let wrapped_mins = total.wrapping_rem_euclid(60);
        let wrapped_hrs = total.wrapping_div_euclid(60).wrapping_rem_euclid(24);

        Clock {
            hours: wrapped_hrs,
            minutes: wrapped_mins,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}
