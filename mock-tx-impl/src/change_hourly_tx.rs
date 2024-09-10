use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::classification::{ChangeHourlyTransaction, HourlyChangeableEmployee};

#[derive(Debug, Clone)]
pub struct ChangeHourlyTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub hourly_rate: f32,
}
impl HaveEmployeeDao<()> for ChangeHourlyTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl HourlyChangeableEmployee for ChangeHourlyTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_hourly_rate(&self) -> f32 {
        self.hourly_rate
    }
}
impl Transaction<()> for ChangeHourlyTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        ChangeHourlyTransaction::execute(self).run(&mut ())
    }
}
