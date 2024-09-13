use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::general::AddSalaryEmployeeTransaction;

#[derive(Debug, Clone)]
pub struct AddSalariedEmployeeTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub name: String,
    pub address: String,
    pub salary: f32,
}
impl HaveEmployeeDao<()> for AddSalariedEmployeeTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl Transaction<()> for AddSalariedEmployeeTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        AddSalaryEmployeeTransaction::execute(
            self,
            self.emp_id,
            &self.name,
            &self.address,
            self.salary,
        )
        .run(&mut ())
        .map(|_| ())
    }
}
