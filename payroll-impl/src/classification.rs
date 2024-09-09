use chrono::NaiveDate;
use std::any::Any;

use payroll_domain::{PayCheck, PaymentClassification};

#[derive(Debug, Clone, PartialEq)]
pub struct SalariedClassification {
    salary: f32,
}
impl PaymentClassification for SalariedClassification {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn calculate_pay(&self, _pc: &PayCheck) -> f32 {
        self.salary
    }
}
impl SalariedClassification {
    pub fn new(salary: f32) -> Self {
        Self { salary }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct HourlyClassification {
    hourly_rate: f32,
    timecards: Vec<TimeCard>,
}
impl PaymentClassification for HourlyClassification {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn calculate_pay(&self, pc: &PayCheck) -> f32 {
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
#[derive(Debug, Clone, PartialEq)]
pub struct CommissionedClassification {
    salary: f32,
    commission_rate: f32,
    sales_receipts: Vec<SalesReceipt>,
}
impl PaymentClassification for CommissionedClassification {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn calculate_pay(&self, pc: &PayCheck) -> f32 {
        let mut total_pay = self.salary;
        let pay_period = pc.get_pay_period();
        for sr in self.sales_receipts.iter() {
            if pay_period.contains(&sr.get_date()) {
                total_pay += self.calculate_pay_for_sales_receipt(sr);
            }
        }
        total_pay
    }
}
impl CommissionedClassification {
    pub fn new(salary: f32, commission_rate: f32) -> Self {
        Self {
            salary,
            commission_rate,
            sales_receipts: vec![],
        }
    }
    pub fn add_sales_receipt(&mut self, sr: SalesReceipt) {
        self.sales_receipts.push(sr);
    }
    pub fn calculate_pay_for_sales_receipt(&self, sr: &SalesReceipt) -> f32 {
        self.commission_rate * sr.get_amount()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimeCard {
    date: NaiveDate,
    hours: f32,
}
impl TimeCard {
    pub fn new(date: NaiveDate, hours: f32) -> Self {
        Self { date, hours }
    }
    pub fn get_date(&self) -> NaiveDate {
        self.date
    }
    pub fn get_hours(&self) -> f32 {
        self.hours
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SalesReceipt {
    date: NaiveDate,
    amount: f32,
}
impl SalesReceipt {
    pub fn new(date: NaiveDate, amount: f32) -> Self {
        Self { date, amount }
    }
    pub fn get_date(&self) -> NaiveDate {
        self.date
    }
    pub fn get_amount(&self) -> f32 {
        self.amount
    }
}
