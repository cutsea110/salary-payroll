use payroll_domain::{Paycheck, PaymentMethod};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HoldMethod;
impl PaymentMethod for HoldMethod {
    fn pay(&self, pc: &Paycheck) {
        // concrete implementation
        println!("HoldMethod: {:#?}", pc);
    }
}
