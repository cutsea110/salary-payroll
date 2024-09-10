use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::general::{DeletableEmployee, DeleteEmployeeTransaction};

#[derive(Debug, Clone)]
pub struct DeleteEmployeeTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
}
impl HaveEmployeeDao<()> for DeleteEmployeeTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl DeletableEmployee for DeleteEmployeeTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
}
impl Transaction<()> for DeleteEmployeeTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        DeleteEmployeeTransaction::execute(self)
            .run(&mut ())
            .map(|_| ())
    }
}
