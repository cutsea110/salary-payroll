use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::EmployeeId;
use tx_app::Transaction;
use tx_impl::method::ChangeDirectTransaction;

#[derive(Debug, Clone)]
pub struct ChangeDirectTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub bank: String,
    pub account: String,
}
impl HaveEmployeeDao<()> for ChangeDirectTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl Transaction<()> for ChangeDirectTransactionImpl {
    fn execute(&self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeDirectTransaction::execute(self, self.emp_id, &self.bank, &self.account).run(ctx)
    }
}
