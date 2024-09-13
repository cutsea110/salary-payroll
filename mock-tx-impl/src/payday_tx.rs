use chrono::NaiveDate;
use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use tx_app::Transaction;
use tx_impl::general::*;

#[derive(Debug, Clone)]
pub struct PaydayTransactionImpl {
    pub db: MockDb,

    pub pay_date: NaiveDate,
}
impl HaveEmployeeDao<()> for PaydayTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl Transaction<()> for PaydayTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        PaydayTransaction::execute(self, self.pay_date).run(&mut ())
    }
}
