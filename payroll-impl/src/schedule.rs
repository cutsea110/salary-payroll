use chrono::{Datelike, Days, NaiveDate, Weekday};
use std::ops::RangeInclusive;

use payroll_domain::PaymentSchedule;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MonthlySchedule;
impl PaymentSchedule for MonthlySchedule {
    fn is_pay_date(&self, date: NaiveDate) -> bool {
        self.is_last_day_of_month(date)
    }
    fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
        // pay_date should be last_day of month
        pay_date.with_day(1).unwrap()..=pay_date
    }
}
impl MonthlySchedule {
    pub fn is_last_day_of_month(&self, date: NaiveDate) -> bool {
        date.month() != date.checked_add_days(Days::new(1)).unwrap().month()
    }
}
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
