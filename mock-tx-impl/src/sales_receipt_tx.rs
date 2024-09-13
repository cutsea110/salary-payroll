use chrono::NaiveDate;
use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::general::SalesReceiptTransaction;

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
impl Transaction<()> for SalesReceiptTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        SalesReceiptTransaction::execute(self, self.emp_id, self.date, self.amount).run(&mut ())
    }
}
