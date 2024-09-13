use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::general::AddHourlyEmployeeTransaction;

#[derive(Debug, Clone)]
pub struct AddHourlyEmployeeTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub name: String,
    pub address: String,
    pub hourly_rate: f32,
}
impl HaveEmployeeDao<()> for AddHourlyEmployeeTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl Transaction<()> for AddHourlyEmployeeTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        AddHourlyEmployeeTransaction::execute(
            self,
            self.emp_id,
            &self.name,
            &self.address,
            self.hourly_rate,
        )
        .run(&mut ())
        .map(|_| ())
    }
}
