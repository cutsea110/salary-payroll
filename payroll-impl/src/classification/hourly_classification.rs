use std::any::Any;

use crate::classification::timecard::TimeCard;
use payroll_domain::{Paycheck, PaymentClassification};

#[derive(Debug, Clone, PartialEq)]
pub struct HourlyClassification {
    hourly_rate: f32,
    timecards: Vec<TimeCard>,
}
impl PaymentClassification for HourlyClassification {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn calculate_pay(&self, pc: &Paycheck) -> f32 {
        let pay_period = pc.get_pay_period();
        let mut total_pay = 0.0;
        for tc in self.timecards.iter() {
            if pay_period.contains(&tc.get_date()) {
                total_pay += self.calculate_pay_for_timecard(tc);
            }
        }
        total_pay
    }
}
impl HourlyClassification {
    pub fn new(hourly_rate: f32) -> Self {
        Self {
            hourly_rate,
            timecards: vec![],
        }
    }
    pub fn add_timecard(&mut self, tc: TimeCard) {
        self.timecards.push(tc);
    }
    pub fn calculate_pay_for_timecard(&self, tc: &TimeCard) -> f32 {
        let hours = tc.get_hours();
        let overtime = (hours - 8.0).max(0.0);
        let straight_time = hours - overtime;
        straight_time * self.hourly_rate + overtime * self.hourly_rate * 1.5
    }
}
