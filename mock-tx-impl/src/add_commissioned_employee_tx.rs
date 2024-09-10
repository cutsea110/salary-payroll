use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::general::{AddCommissionedEmployeeTransaction, CommissionedEmployee};

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
impl CommissionedEmployee for AddCommissionedEmployeeTransactionImpl {
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
    fn get_commission_rate(&self) -> f32 {
        self.commission_rate
    }
}
impl Transaction<()> for AddCommissionedEmployeeTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        AddCommissionedEmployeeTransaction::execute(self)
            .run(&mut ())
            .map(|_| ())
    }
}
