use chrono::NaiveDate;
use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::general::TimeCardTransaction;

#[derive(Debug, Clone)]
pub struct TimeCardTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub date: NaiveDate,
    pub hours: f32,
}
impl HaveEmployeeDao<()> for TimeCardTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl Transaction<()> for TimeCardTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        TimeCardTransaction::execute(self, self.emp_id, self.date, self.hours).run(&mut ())
    }
}
