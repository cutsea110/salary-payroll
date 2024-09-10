use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::method::{ChangeDirectTransaction, DirectChangeableEmployee};

#[derive(Debug, Clone)]
pub struct ChangeDirectTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub bank: String,
    pub account: String,
}
impl HaveEmployeeDao<()> for ChangeDirectTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl DirectChangeableEmployee for ChangeDirectTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_bank(&self) -> &str {
        &self.bank
    }
    fn get_account(&self) -> &str {
        &self.account
    }
}
impl Transaction<()> for ChangeDirectTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        ChangeDirectTransaction::execute(self).run(&mut ())
    }
}
