#[derive(Debug,Eq,PartialEq)]
pub struct Clock{
    minutes:i32
}

use std::fmt::{Display,Formatter,Result};
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes : ((hours * 60 + minutes) % 1440 + 1440) % 1440
        }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Self::new(0, minutes + self.minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result{
        write!(f,"{:02}:{:02}",self.minutes/60,self.minutes%60)
    }
}

