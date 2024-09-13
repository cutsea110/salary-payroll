use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::method::ChangeMailTransaction;

#[derive(Debug, Clone)]
pub struct ChangeMailTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub address: String,
}
impl HaveEmployeeDao<()> for ChangeMailTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl Transaction<()> for ChangeMailTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        ChangeMailTransaction::execute(self, self.emp_id, &self.address).run(&mut ())
    }
}
