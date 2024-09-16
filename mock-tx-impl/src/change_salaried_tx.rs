use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::classification::ChangeSalariedTransaction;

#[derive(Debug, Clone)]
pub struct ChangeSalariedTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub salary: f32,
}
impl HaveEmployeeDao<()> for ChangeSalariedTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl Transaction<()> for ChangeSalariedTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        ChangeSalariedTransaction::execute(self, self.emp_id, self.salary).run(&mut ())
    }
}
