use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::classification::ChangeHourlyTransaction;

#[derive(Debug, Clone)]
pub struct ChangeHourlyTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub hourly_rate: f32,
}
impl HaveEmployeeDao<()> for ChangeHourlyTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl Transaction<()> for ChangeHourlyTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        ChangeHourlyTransaction::execute(self, self.emp_id, self.hourly_rate).run(&mut ())
    }
}
