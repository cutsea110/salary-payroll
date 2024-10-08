use payroll_domain::{Paycheck, PaymentMethod};

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
    pub fn new(bank: &str, account: &str) -> Self {
        Self {
            bank: bank.to_string(),
            account: account.to_string(),
        }
    }
}
