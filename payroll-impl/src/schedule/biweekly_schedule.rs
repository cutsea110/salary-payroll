use chrono::{Datelike, Days, NaiveDate, Weekday};
use std::ops::RangeInclusive;

use payroll_domain::PaymentSchedule;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BiweeklySchedule;
impl PaymentSchedule for BiweeklySchedule {
    fn is_pay_date(&self, date: NaiveDate) -> bool {
        date.weekday() == Weekday::Fri && date.iso_week().week() % 2 == 0
    }
    fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
        pay_date.checked_sub_days(Days::new(13)).unwrap()..=pay_date
    }
}
