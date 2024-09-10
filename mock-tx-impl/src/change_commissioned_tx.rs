use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::classification::{ChangeCommissionedTransaction, CommissionedChangeableEmployee};

#[derive(Debug, Clone)]
pub struct ChangeCommissionedTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub salary: f32,
    pub commission_rate: f32,
}
impl HaveEmployeeDao<()> for ChangeCommissionedTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl CommissionedChangeableEmployee for ChangeCommissionedTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_salary(&self) -> f32 {
        self.salary
    }
    fn get_commission_rate(&self) -> f32 {
        self.commission_rate
    }
}
impl Transaction<()> for ChangeCommissionedTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        ChangeCommissionedTransaction::execute(self).run(&mut ())
    }
}
