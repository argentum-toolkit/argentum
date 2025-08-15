use chrono::{Duration, NaiveDate, NaiveDateTime};
use std::mem;

#[derive(Clone)]
pub struct DateRange(pub NaiveDate, pub NaiveDate);

impl DateRange {
    pub fn date_in_range(&self, d: NaiveDate) -> bool {
        self.0 <= d && d <= self.1
    }

    /// NOTE: This function ignores leap seconds
    pub fn datetime_in_range(&self, dt: NaiveDateTime) -> Result<bool, String> {
        //TODDO: check leap second
        let from = self
            .0
            .and_hms_opt(0, 0, 0)
            .ok_or("Cah't convert NativeDate into NativeDateTime")?;
        let to = self
            .1
            .and_hms_opt(23, 59, 59)
            .ok_or("Cah't convert NativeDate into NativeDateTime")?;

        Ok(from <= dt && dt <= to)
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
    use std::error::Error;

    use crate::data_type::DateRange;

    use chrono::{NaiveDate, NaiveDateTime};

    fn date(d: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(d, "%Y-%m-%dT%H:%M:%S%Z")
            .expect("Date and format should be valid")
    }

    #[test]
    fn date_in_range() -> Result<(), Box<dyn Error>> {
        let d = DateRange(
            NaiveDate::from_ymd_opt(2020, 10, 1).ok_or("Can't create date")?,
            NaiveDate::from_ymd_opt(2020, 10, 12).ok_or("Can't create date")?,
        );
        assert!(d.date_in_range(NaiveDate::from_ymd_opt(2020, 10, 1).ok_or("Can't create date")?));
        assert!(d.date_in_range(NaiveDate::from_ymd_opt(2020, 10, 5).ok_or("Can't create date")?));
        assert!(d.date_in_range(NaiveDate::from_ymd_opt(2020, 10, 12).ok_or("Can't create date")?));

        Ok(())
    }

    #[test]
    fn date_not_in_range() -> Result<(), Box<dyn Error>> {
        let d = DateRange(
            NaiveDate::from_ymd_opt(2020, 10, 1).ok_or("Can't create date")?,
            NaiveDate::from_ymd_opt(2020, 10, 12).ok_or("Can't create date")?,
        );

        assert!(!d.date_in_range(NaiveDate::from_ymd_opt(2020, 9, 20).ok_or("Can't create date")?));
        assert!(!d.date_in_range(NaiveDate::from_ymd_opt(2020, 9, 30).ok_or("Can't create date")?));
        assert!(
            !d.date_in_range(NaiveDate::from_ymd_opt(2020, 10, 13).ok_or("Can't create date")?)
        );
        assert!(
            !d.date_in_range(NaiveDate::from_ymd_opt(2020, 10, 15).ok_or("Can't create date")?)
        );

        Ok(())
    }

    #[test]
    fn datetime_in_range() -> Result<(), Box<dyn Error>> {
        let d = DateRange(
            NaiveDate::from_ymd_opt(2020, 10, 1).ok_or("Can't create date")?,
            NaiveDate::from_ymd_opt(2020, 10, 12).ok_or("Can't create date")?,
        );

        assert!(d.datetime_in_range(date("2020-10-1T0:0:0Z"))?);
        assert!(d.datetime_in_range(date("2020-10-11T10:10:10Z"))?);
        assert!(d.datetime_in_range(date("2020-10-12T23:59:59Z"))?);

        Ok(())
    }

    #[test]
    fn datetime_not_in_range() -> Result<(), Box<dyn Error>> {
        let d = DateRange(
            NaiveDate::from_ymd_opt(2020, 10, 1).ok_or("Can't create date")?,
            NaiveDate::from_ymd_opt(2020, 10, 12).ok_or("Can't create date")?,
        );

        assert!(!d.datetime_in_range(date("2020-9-30T23:59:59Z"))?);
        assert!(!d.datetime_in_range(date("2020-10-13T0:0:0Z"))?);

        Ok(())
    }
}
