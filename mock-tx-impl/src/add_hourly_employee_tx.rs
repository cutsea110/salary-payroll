use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::general::{AddHourlyEmployeeTransaction, HourlyEmployee};

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
impl HourlyEmployee for AddHourlyEmployeeTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_address(&self) -> &str {
        &self.address
    }
    fn get_hourly_rate(&self) -> f32 {
        self.hourly_rate
    }
}
impl Transaction<()> for AddHourlyEmployeeTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        AddHourlyEmployeeTransaction::execute(self)
            .run(&mut ())
            .map(|_| ())
    }
}
