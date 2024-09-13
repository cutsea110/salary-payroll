use payroll_domain::{Paycheck, PaymentMethod};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MailMethod {
    address: String,
}
impl PaymentMethod for MailMethod {
    fn pay(&self, pc: &Paycheck) {
        // concrete implementation
        println!("MailMethod for {}: {:#?}", self.address, pc);
    }
}
impl MailMethod {
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
        }
    }
}
