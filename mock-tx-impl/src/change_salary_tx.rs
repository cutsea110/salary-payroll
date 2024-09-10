use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::classification::{ChangeSalariedTransaction, SalaryChangeableEmployee};

#[derive(Debug, Clone)]
pub struct ChangeSalaryTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub salary: f32,
}
impl HaveEmployeeDao<()> for ChangeSalaryTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl SalaryChangeableEmployee for ChangeSalaryTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_salary(&self) -> f32 {
        self.salary
    }
}
impl Transaction<()> for ChangeSalaryTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        ChangeSalariedTransaction::execute(self).run(&mut ())
    }
}
