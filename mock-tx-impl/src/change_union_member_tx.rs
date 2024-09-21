use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::{EmployeeId, MemberId};
use tx_app::Transaction;
use tx_impl::affiliation::ChangeUnionMemberTransaction;

#[derive(Debug, Clone)]
pub struct ChangeUnionMemberTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub member_id: MemberId,
    pub dues: f32,
}
impl HaveEmployeeDao<()> for ChangeUnionMemberTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl Transaction<()> for ChangeUnionMemberTransactionImpl {
    fn execute(&self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeUnionMemberTransaction::execute(self, self.emp_id, self.member_id, self.dues).run(ctx)
    }
}
