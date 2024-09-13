use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::general::ChangeNameTransaction;

#[derive(Debug, Clone)]
pub struct ChangeNameTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub name: String,
}
impl HaveEmployeeDao<()> for ChangeNameTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl Transaction<()> for ChangeNameTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        ChangeNameTransaction::execute(self, self.emp_id, &self.name).run(&mut ())
    }
}
