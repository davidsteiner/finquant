use crate::time::daycounters::DayCounters;
use chrono::NaiveDate;

#[derive(Default)]
pub struct Actual365Fixed;

impl DayCounters for Actual365Fixed {
    fn day_count(&self, d1: NaiveDate, d2: NaiveDate) -> i64 {
        let duration = d2 - d1;
        duration.num_days()
    }

    fn year_fraction(&self, d1: NaiveDate, d2: NaiveDate) -> f32 {
        self.day_count(d1, d2) as f32 / 365.0
    }
}