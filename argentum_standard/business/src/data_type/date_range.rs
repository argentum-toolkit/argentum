use chrono::{Date, Duration, Utc};
use std::mem;

pub struct DateRange(pub Date<Utc>, pub Date<Utc>);

impl Iterator for DateRange {
    type Item = Date<Utc>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next = self.0 + Duration::days(1);
            Some(mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}

impl Clone for DateRange {
    fn clone(&self) -> DateRange {
        DateRange(self.0, self.1)
    }
}
