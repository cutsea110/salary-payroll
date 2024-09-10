use chrono::{Datelike, Days, NaiveDate, Weekday};
use std::ops::RangeInclusive;

use payroll_domain::PaymentSchedule;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WeeklySchedule;
impl PaymentSchedule for WeeklySchedule {
    fn is_pay_date(&self, date: NaiveDate) -> bool {
        date.weekday() == Weekday::Fri
    }
    fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
        pay_date.checked_sub_days(Days::new(6)).unwrap()..=pay_date
    }
}
