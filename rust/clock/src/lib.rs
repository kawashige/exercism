use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::create(hours, minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::create(self.hours, self.minutes + minutes)
    }

    fn create(new_hours: i32, new_minutes: i32) -> Self {
        let mut hours = new_hours;
        let mut minutes = new_minutes;
        while minutes < 0 {
            minutes += 60;
            hours -= 1;
        }
        while hours < 0 {
            hours += 24
        }
        hours = (hours + minutes / 60) % 24;
        minutes %= 60;
        Self { hours, minutes }
    }
}
