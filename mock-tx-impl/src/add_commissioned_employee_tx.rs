use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::general::AddCommissionedEmployeeTransaction;

#[derive(Debug, Clone)]
pub struct AddCommissionedEmployeeTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub name: String,
    pub address: String,
    pub salary: f32,
    pub commission_rate: f32,
}
impl HaveEmployeeDao<()> for AddCommissionedEmployeeTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl Transaction<()> for AddCommissionedEmployeeTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        AddCommissionedEmployeeTransaction::execute(
            self,
            self.emp_id,
            &self.name,
            &self.address,
            self.salary,
            self.commission_rate,
        )
        .run(&mut ())
        .map(|_| ())
    }
}
