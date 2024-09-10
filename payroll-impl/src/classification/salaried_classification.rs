use std::any::Any;

use payroll_domain::{Paycheck, PaymentClassification};

#[derive(Debug, Clone, PartialEq)]
pub struct SalariedClassification {
    salary: f32,
}
impl PaymentClassification for SalariedClassification {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn calculate_pay(&self, _pc: &Paycheck) -> f32 {
        self.salary
    }
}
impl SalariedClassification {
    pub fn new(salary: f32) -> Self {
        Self { salary }
    }
}
