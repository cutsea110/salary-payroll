use chrono::NaiveDate;
use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::general::{TimeCardEmployee, TimeCardTransaction};

#[derive(Debug, Clone)]
pub struct TimeCardTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub date: NaiveDate,
    pub hours: f32,
}
impl HaveEmployeeDao<()> for TimeCardTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl TimeCardEmployee for TimeCardTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_date(&self) -> NaiveDate {
        self.date
    }
    fn get_hours(&self) -> f32 {
        self.hours
    }
}
impl Transaction<()> for TimeCardTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        TimeCardTransaction::execute(self).run(&mut ())
    }
}
