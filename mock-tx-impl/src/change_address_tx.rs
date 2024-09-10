use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::general::{AddressChangeableEmployee, ChangeAddressTransaction};

#[derive(Debug, Clone)]
pub struct ChangeAddressTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub address: String,
}
impl HaveEmployeeDao<()> for ChangeAddressTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl AddressChangeableEmployee for ChangeAddressTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_address(&self) -> &str {
        &self.address
    }
}
impl Transaction<()> for ChangeAddressTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        ChangeAddressTransaction::execute(self).run(&mut ())
    }
}
