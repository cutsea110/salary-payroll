use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::{EmployeeId, MemberId};
use tx_app::Transaction;
use tx_impl::affiliation::{ChangeUnionMemberTransaction, UnionChangeableEmployee};

#[derive(Debug, Clone)]
pub struct ChangeUnionMemberTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub member_id: EmployeeId,
    pub dues: f32,
}
impl HaveEmployeeDao<()> for ChangeUnionMemberTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl UnionChangeableEmployee for ChangeUnionMemberTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_member_id(&self) -> MemberId {
        self.member_id
    }
    fn get_dues(&self) -> f32 {
        self.dues
    }
}
impl Transaction<()> for ChangeUnionMemberTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        ChangeUnionMemberTransaction::execute(self).run(&mut ())
    }
}
