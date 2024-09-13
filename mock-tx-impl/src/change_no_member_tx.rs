use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::affiliation::ChangeUnaffiliatedTransaction;

#[derive(Debug, Clone)]
pub struct ChangeNoMemberTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
}
impl HaveEmployeeDao<()> for ChangeNoMemberTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl Transaction<()> for ChangeNoMemberTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        ChangeUnaffiliatedTransaction::execute(self, self.emp_id).run(&mut ())
    }
}
