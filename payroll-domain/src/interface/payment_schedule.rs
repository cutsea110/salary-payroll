use chrono::NaiveDate;
use dyn_clone::DynClone;
use std::fmt::Debug;
use std::ops::RangeInclusive;

pub trait PaymentSchedule: DynClone + Debug {
    fn is_pay_date(&self, date: NaiveDate) -> bool;
    fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate>;
}
dyn_clone::clone_trait_object!(PaymentSchedule);
