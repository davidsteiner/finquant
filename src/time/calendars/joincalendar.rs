use crate::time::calendars::Calendar;
use chrono::NaiveDate;

#[derive(Default)]
pub struct JoinCalendar<T1: Calendar, T2: Calendar> {
    pub c1: T1,
    pub c2: T2,
}

impl<T1: Calendar, T2: Calendar> JoinCalendar<T1, T2> {
    pub fn new(c1: T1, c2: T2) -> Self {
        Self { c1, c2 }
    }
}

impl<T1: Calendar, T2: Calendar> Calendar for JoinCalendar<T1, T2> {
    fn is_business_day(&self, date: NaiveDate) -> bool {
        self.c1.is_business_day(date) && self.c2.is_business_day(date)
    }
}