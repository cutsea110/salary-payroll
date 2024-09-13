use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::method::ChangeHoldTransaction;

#[derive(Debug, Clone)]
pub struct ChangeHoldTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
}
impl HaveEmployeeDao<()> for ChangeHoldTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl Transaction<()> for ChangeHoldTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        ChangeHoldTransaction::execute(self, self.emp_id).run(&mut ())
    }
}
