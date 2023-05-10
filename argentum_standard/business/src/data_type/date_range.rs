use chrono::{Duration, NaiveDate, NaiveDateTime};
use std::mem;

#[derive(Clone)]
pub struct DateRange(pub NaiveDate, pub NaiveDate);

impl DateRange {
    pub fn date_in_range(&self, d: NaiveDate) -> bool {
        self.0 <= d && d <= self.1
    }

    pub fn datetime_in_range(&self, dt: NaiveDateTime) -> bool {
        let from = self.0.and_hms_opt(0, 0, 0).unwrap();
        let to = self.1.and_hms_opt(23, 59, 59).unwrap();

        from <= dt && dt <= to
    }
}

impl Iterator for DateRange {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next = self.0 + Duration::days(1);
            Some(mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::DateRange;

    use chrono::{NaiveDate, NaiveDateTime};

    fn date(d: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(d, "%Y-%m-%dT%H:%M:%S%Z").unwrap()
    }

    #[test]
    fn date_in_range() {
        let d = DateRange(
            NaiveDate::from_ymd_opt(2020, 10, 1).unwrap(),
            NaiveDate::from_ymd_opt(2020, 10, 12).unwrap(),
        );
        assert!(d.date_in_range(NaiveDate::from_ymd_opt(2020, 10, 1).unwrap()));
        assert!(d.date_in_range(NaiveDate::from_ymd_opt(2020, 10, 5).unwrap()));
        assert!(d.date_in_range(NaiveDate::from_ymd_opt(2020, 10, 12).unwrap()));
    }

    #[test]
    fn date_not_in_range() {
        let d = DateRange(
            NaiveDate::from_ymd_opt(2020, 10, 1).unwrap(),
            NaiveDate::from_ymd_opt(2020, 10, 12).unwrap(),
        );

        assert!(!d.date_in_range(NaiveDate::from_ymd_opt(2020, 9, 20).unwrap()));
        assert!(!d.date_in_range(NaiveDate::from_ymd_opt(2020, 9, 30).unwrap()));
        assert!(!d.date_in_range(NaiveDate::from_ymd_opt(2020, 10, 13).unwrap()));
        assert!(!d.date_in_range(NaiveDate::from_ymd_opt(2020, 10, 15).unwrap()));
    }

    #[test]
    fn datetime_in_range() {
        let d = DateRange(
            NaiveDate::from_ymd_opt(2020, 10, 1).unwrap(),
            NaiveDate::from_ymd_opt(2020, 10, 12).unwrap(),
        );

        assert!(d.datetime_in_range(date("2020-10-1T0:0:0Z")));
        assert!(d.datetime_in_range(date("2020-10-11T10:10:10Z")));
        assert!(d.datetime_in_range(date("2020-10-12T23:59:59Z")));
    }

    #[test]
    fn datetime_not_in_range() {
        let d = DateRange(
            NaiveDate::from_ymd_opt(2020, 10, 1).unwrap(),
            NaiveDate::from_ymd_opt(2020, 10, 12).unwrap(),
        );

        assert!(!d.datetime_in_range(date("2020-9-30T23:59:59Z")));
        assert!(!d.datetime_in_range(date("2020-10-13T0:0:0Z")));
    }
}
