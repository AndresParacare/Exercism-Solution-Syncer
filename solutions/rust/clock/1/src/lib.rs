use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let t_minutes = ((minutes % 60) + 60) % 60;
        
        let round_hour = minutes as f32;
        let round_hour= (round_hour/ 60.0).floor();
        let round_hour= round_hour as i32;
        let t_hours = (24 + (hours % 24) + (round_hour % 24)) % 24;
        
        Self {
            hours: t_hours,
            minutes: t_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let t_minutes = (((minutes + self.minutes) % 60) + 60) % 60;
        
        let round_hour = (self.minutes + minutes) as f32;
        let round_hour= (round_hour/ 60.0).floor();
        let round_hour= round_hour as i32;
        let t_hours = (24 + (self.hours % 24) 
                       + round_hour) % 24;
        Self {
            hours: t_hours,
            minutes:t_minutes,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}