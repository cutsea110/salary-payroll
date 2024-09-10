use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::general::{AddSalaryEmployeeTransaction, SalaryEmployee};

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
impl SalaryEmployee for AddSalariedEmployeeTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_address(&self) -> &str {
        &self.address
    }
    fn get_salary(&self) -> f32 {
        self.salary
    }
}
impl Transaction<()> for AddSalariedEmployeeTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        AddSalaryEmployeeTransaction::execute(self)
            .run(&mut ())
            .map(|_| ())
    }
}
