use std::any::Any;

use crate::classification::sales_receipt::SalesReceipt;
use payroll_domain::{Paycheck, PaymentClassification};

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
    fn calculate_pay(&self, pc: &Paycheck) -> f32 {
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
