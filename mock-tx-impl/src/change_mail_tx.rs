use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::method::{ChangeMailTransaction, MailChangeableEmployee};

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
impl MailChangeableEmployee for ChangeMailTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_address(&self) -> &str {
        &self.address
    }
}
impl Transaction<()> for ChangeMailTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        ChangeMailTransaction::execute(self).run(&mut ())
    }
}
