use chrono::NaiveDate;
use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::MemberId;
use tx_app::Transaction;
use tx_impl::affiliation::{ServiceChargeTransaction, ServiceChargeableMember};

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
impl ServiceChargeableMember for ServiceChargeTransactionImpl {
    fn get_member_id(&self) -> MemberId {
        self.member_id
    }
    fn get_date(&self) -> NaiveDate {
        self.date
    }
    fn get_amount(&self) -> f32 {
        self.amount
    }
}
impl Transaction<()> for ServiceChargeTransactionImpl {
    fn execute(&mut self) -> Result<(), UsecaseError> {
        ServiceChargeTransaction::execute(self).run(&mut ())
    }
}
