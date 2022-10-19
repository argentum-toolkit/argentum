use chrono::{Date, DateTime, Duration, Utc};
use std::mem;

pub struct DateRange(pub Date<Utc>, pub Date<Utc>);

impl DateRange {
    pub fn date_in_range(&self, d: Date<Utc>) -> bool {
        self.0 <= d && d <= self.1
    }

    pub fn datetime_in_range(&self, dt: DateTime<Utc>) -> bool {
        let from = self.0.and_hms(0, 0, 0);
        let to = self.1.and_hms(23, 59, 59);

        from <= dt && dt <= to
    }
}

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

#[cfg(test)]
mod tests {
    use crate::data_type::DateRange;

    use chrono::{DateTime, TimeZone, Utc};
    use std::str::FromStr;

    fn date(d: &str) -> DateTime<Utc> {
        DateTime::from_str(d).unwrap()
    }

    #[test]
    fn date_in_range() {
        let d = DateRange(Utc.ymd(2020, 10, 1), Utc.ymd(2020, 10, 12));
        assert!(d.date_in_range(Utc.ymd(2020, 10, 1)));
        assert!(d.date_in_range(Utc.ymd(2020, 10, 5)));
        assert!(d.date_in_range(Utc.ymd(2020, 10, 12)));
    }

    #[test]
    fn date_not_in_range() {
        let d = DateRange(Utc.ymd(2020, 10, 1), Utc.ymd(2020, 10, 12));

        assert!(!d.date_in_range(Utc.ymd(2020, 9, 20)));
        assert!(!d.date_in_range(Utc.ymd(2020, 9, 30)));
        assert!(!d.date_in_range(Utc.ymd(2020, 10, 13)));
        assert!(!d.date_in_range(Utc.ymd(2020, 10, 15)));
    }

    #[test]
    fn datetime_in_range() {
        let d = DateRange(Utc.ymd(2020, 10, 1), Utc.ymd(2020, 10, 12));

        assert!(d.datetime_in_range(date("2020-10-1T0:0:0Z")));
        assert!(d.datetime_in_range(date("2020-10-11T10:10:10Z")));
        assert!(d.datetime_in_range(date("2020-10-12T23:59:59Z")));
    }

    #[test]
    fn datetime_not_in_range() {
        let d = DateRange(Utc.ymd(2020, 10, 1), Utc.ymd(2020, 10, 12));

        assert!(!d.datetime_in_range(date("2020-9-30T23:59:59Z")));
        assert!(!d.datetime_in_range(date("2020-10-13T0:0:0Z")));
    }
}
