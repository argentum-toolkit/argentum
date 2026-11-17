use jiff::ToSpan;
use jiff::civil::{Date, DateTime};
use std::mem;

#[derive(Clone)]
pub struct DateRange(pub Date, pub Date);

impl DateRange {
    pub fn date_in_range(&self, d: Date) -> bool {
        self.0 <= d && d <= self.1
    }

    /// NOTE: This function ignores leap seconds
    pub fn datetime_in_range(&self, dt: DateTime) -> Result<bool, String> {
        //TODDO: check leap second
        let from = self.0.at(0, 0, 0, 0);
        let to = self.1.at(23, 59, 59, 0);

        Ok(from <= dt && dt <= to)
    }
}

impl Iterator for DateRange {
    type Item = Date;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next = self.0 + 1.day();
            Some(mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::data_type::DateRange;

    use jiff::civil::{Date, DateTime};

    fn date(d: &str) -> DateTime {
        DateTime::strptime("%Y-%m-%dT%H:%M:%SZ", d).expect("Date and format should be valid")
    }

    #[test]
    fn date_in_range() -> Result<(), Box<dyn Error>> {
        let d = DateRange(Date::new(2020, 10, 1)?, Date::new(2020, 10, 12)?);
        assert!(d.date_in_range(Date::new(2020, 10, 1)?));
        assert!(d.date_in_range(Date::new(2020, 10, 5)?));
        assert!(d.date_in_range(Date::new(2020, 10, 12)?));

        Ok(())
    }

    #[test]
    fn date_not_in_range() -> Result<(), Box<dyn Error>> {
        let d = DateRange(Date::new(2020, 10, 1)?, Date::new(2020, 10, 12)?);

        assert!(!d.date_in_range(Date::new(2020, 9, 20)?));
        assert!(!d.date_in_range(Date::new(2020, 9, 30)?));
        assert!(!d.date_in_range(Date::new(2020, 10, 13)?));
        assert!(!d.date_in_range(Date::new(2020, 10, 15)?));

        Ok(())
    }

    #[test]
    fn datetime_in_range() -> Result<(), Box<dyn Error>> {
        let d = DateRange(Date::new(2020, 10, 1)?, Date::new(2020, 10, 12)?);

        assert!(d.datetime_in_range(date("2020-10-1T0:0:0Z"))?);
        assert!(d.datetime_in_range(date("2020-10-11T10:10:10Z"))?);
        assert!(d.datetime_in_range(date("2020-10-12T23:59:59Z"))?);

        Ok(())
    }

    #[test]
    fn datetime_not_in_range() -> Result<(), Box<dyn Error>> {
        let d = DateRange(Date::new(2020, 10, 1)?, Date::new(2020, 10, 12)?);

        assert!(!d.datetime_in_range(date("2020-9-30T23:59:59Z"))?);
        assert!(!d.datetime_in_range(date("2020-10-13T0:0:0Z"))?);

        Ok(())
    }
}
