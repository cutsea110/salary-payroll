use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::general::{ChangeNameTransaction, NameChangeableEmployee};

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
impl NameChangeableEmployee for ChangeNameTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}
impl Transaction<()> for ChangeNameTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        ChangeNameTransaction::execute(self).run(&mut ())
    }
}
