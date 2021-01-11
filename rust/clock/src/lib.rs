use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    pub minutes: i32,
    _secret: (),
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::create(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::create(self.minutes + minutes)
    }

    fn create(new_minutes: i32) -> Self {
        let minutes = new_minutes.rem_euclid(60 * 24);
        Self {
            minutes,
            _secret: (),
        }
    }
}
