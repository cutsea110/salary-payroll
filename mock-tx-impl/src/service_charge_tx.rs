use chrono::NaiveDate;
use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::MemberId;
use tx_app::Transaction;
use tx_impl::affiliation::ServiceChargeTransaction;

#[derive(Debug, Clone)]
pub struct ServiceChargeTransactionImpl {
    pub db: MockDb,

    pub member_id: MemberId,
    pub date: NaiveDate,
    pub amount: f32,
}
impl HaveEmployeeDao<()> for ServiceChargeTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl Transaction<()> for ServiceChargeTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        ServiceChargeTransaction::execute(self, self.member_id, self.date, self.amount).run(&mut ())
    }
}
