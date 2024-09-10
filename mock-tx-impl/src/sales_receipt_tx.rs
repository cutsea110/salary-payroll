use chrono::NaiveDate;
use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::general::{SalesReceiptEmployee, SalesReceiptTransaction};

#[derive(Debug, Clone)]
pub struct SalesReceiptTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub date: NaiveDate,
    pub amount: f32,
}
impl HaveEmployeeDao<()> for SalesReceiptTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl SalesReceiptEmployee for SalesReceiptTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_date(&self) -> NaiveDate {
        self.date
    }
    fn get_amount(&self) -> f32 {
        self.amount
    }
}
impl Transaction<()> for SalesReceiptTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        SalesReceiptTransaction::execute(self).run(&mut ())
    }
}
