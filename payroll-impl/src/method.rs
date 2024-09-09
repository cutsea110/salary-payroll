use payroll_domain::{Paycheck, PaymentMethod};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HoldMethod;
impl PaymentMethod for HoldMethod {
    fn pay(&self, pc: &Paycheck) {
        // concrete implementation
        println!("HoldMethod: {:#?}", pc);
    }
}
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
    pub fn new(address: String) -> Self {
        Self { address }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DirectMethod {
    bank: String,
    account: String,
}
impl PaymentMethod for DirectMethod {
    fn pay(&self, pc: &Paycheck) {
        // concrete implementation
        println!("DirectMethod to {}{}: {:#?}", self.bank, self.account, pc);
    }
}
impl DirectMethod {
    pub fn new(bank: String, account: String) -> Self {
        Self { bank, account }
    }
}
