

// pub struct Clock;

use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i32, // total minutes since 00:00
}

impl Clock {
    const MINUTES_PER_DAY: i32 = 24 * 60;

    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        let normalized = ((total_minutes % Self::MINUTES_PER_DAY) + Self::MINUTES_PER_DAY) % Self::MINUTES_PER_DAY;
        Clock { minutes: normalized }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }

    pub fn subtract_minutes(&self, minutes: i32) -> Self {
        self.add_minutes(-minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.minutes / 60;
        let minutes = self.minutes % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}


// impl Clock {
//     pub fn new(hours: i32, minutes: i32) -> Self {
//         let total = hours * 60 + minutes;

//         let new_hours = total / 60;
//         let new_minutes = total % 60;
//         Clock{new_hours, new_minutes}
//     }

//     pub fn add_minutes(&self, minutes: i32) -> Self {
//         let total = self.hours * 60 + (self.minutes + minutes);
//         let new_hours = total / 60;
//         let new_minutes = total % 60;
//         Clock{new_hours, new_minutes}
//     }
// }
