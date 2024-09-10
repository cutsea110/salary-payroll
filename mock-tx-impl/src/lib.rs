use chrono::NaiveDate;
use tx_rs::Tx;

use abstract_tx::EmployeeUsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use mock_db::MockDb;
use payroll_domain::{EmployeeId, MemberId};
use tx::Transaction;
use tx_impl::{affiliation::*, classification::*, general::*, method::*};

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
    fn execute(&mut self) -> Result<(), EmployeeUsecaseError> {
        AddSalaryEmployeeTransaction::execute(self)
            .run(&mut ())
            .map(|_| ())
    }
}

#[derive(Debug, Clone)]
pub struct AddHourlyEmployeeTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub name: String,
    pub address: String,
    pub hourly_rate: f32,
}
impl HaveEmployeeDao<()> for AddHourlyEmployeeTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl HourlyEmployee for AddHourlyEmployeeTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_address(&self) -> &str {
        &self.address
    }
    fn get_hourly_rate(&self) -> f32 {
        self.hourly_rate
    }
}
impl Transaction<()> for AddHourlyEmployeeTransactionImpl {
    fn execute(&mut self) -> Result<(), EmployeeUsecaseError> {
        AddHourlyEmployeeTransaction::execute(self)
            .run(&mut ())
            .map(|_| ())
    }
}

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
    fn execute(&mut self) -> Result<(), EmployeeUsecaseError> {
        AddCommissionedEmployeeTransaction::execute(self)
            .run(&mut ())
            .map(|_| ())
    }
}

#[derive(Debug, Clone)]
pub struct DeleteEmployeeTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
}
impl HaveEmployeeDao<()> for DeleteEmployeeTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl DeletableEmployee for DeleteEmployeeTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
}
impl Transaction<()> for DeleteEmployeeTransactionImpl {
    fn execute(&mut self) -> Result<(), EmployeeUsecaseError> {
        DeleteEmployeeTransaction::execute(self)
            .run(&mut ())
            .map(|_| ())
    }
}

#[derive(Debug, Clone)]
pub struct TimeCardTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub date: NaiveDate,
    pub hours: f32,
}
impl HaveEmployeeDao<()> for TimeCardTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl TimeCardEmployee for TimeCardTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_date(&self) -> NaiveDate {
        self.date
    }
    fn get_hours(&self) -> f32 {
        self.hours
    }
}
impl Transaction<()> for TimeCardTransactionImpl {
    fn execute(&mut self) -> Result<(), EmployeeUsecaseError> {
        TimeCardTransaction::execute(self).run(&mut ())
    }
}

#[derive(Debug, Clone)]
pub struct SalesReceiptTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub date: NaiveDate,
    pub amount: f32,
}
impl HaveEmployeeDao<()> for SalesReceiptTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl SalesReceiptEmployee for SalesReceiptTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_date(&self) -> NaiveDate {
        self.date
    }
    fn get_amount(&self) -> f32 {
        self.amount
    }
}
impl Transaction<()> for SalesReceiptTransactionImpl {
    fn execute(&mut self) -> Result<(), EmployeeUsecaseError> {
        SalesReceiptTransaction::execute(self).run(&mut ())
    }
}

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
    fn execute(&mut self) -> Result<(), EmployeeUsecaseError> {
        ServiceChargeTransaction::execute(self).run(&mut ())
    }
}

#[derive(Debug, Clone)]
pub struct ChangeNameTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub name: String,
}
impl HaveEmployeeDao<()> for ChangeNameTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl NameChangeableEmployee for ChangeNameTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}
impl Transaction<()> for ChangeNameTransactionImpl {
    fn execute(&mut self) -> Result<(), EmployeeUsecaseError> {
        ChangeNameTransaction::execute(self).run(&mut ())
    }
}

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
    fn execute(&mut self) -> Result<(), EmployeeUsecaseError> {
        ChangeAddressTransaction::execute(self).run(&mut ())
    }
}

#[derive(Debug, Clone)]
pub struct ChangeSalaryTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub salary: f32,
}
impl HaveEmployeeDao<()> for ChangeSalaryTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl SalaryChangeableEmployee for ChangeSalaryTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_salary(&self) -> f32 {
        self.salary
    }
}
impl Transaction<()> for ChangeSalaryTransactionImpl {
    fn execute(&mut self) -> Result<(), EmployeeUsecaseError> {
        ChangeSalariedTransaction::execute(self).run(&mut ())
    }
}

#[derive(Debug, Clone)]
pub struct ChangeHourlyTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub hourly_rate: f32,
}
impl HaveEmployeeDao<()> for ChangeHourlyTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl HourlyChangeableEmployee for ChangeHourlyTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_hourly_rate(&self) -> f32 {
        self.hourly_rate
    }
}
impl Transaction<()> for ChangeHourlyTransactionImpl {
    fn execute(&mut self) -> Result<(), EmployeeUsecaseError> {
        ChangeHourlyTransaction::execute(self).run(&mut ())
    }
}

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
    fn execute(&mut self) -> Result<(), EmployeeUsecaseError> {
        ChangeCommissionedTransaction::execute(self).run(&mut ())
    }
}

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
impl DirectChangeableEmployee for ChangeDirectTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_bank(&self) -> &str {
        &self.bank
    }
    fn get_account(&self) -> &str {
        &self.account
    }
}
impl Transaction<()> for ChangeDirectTransactionImpl {
    fn execute(&mut self) -> Result<(), EmployeeUsecaseError> {
        ChangeDirectTransaction::execute(self).run(&mut ())
    }
}

#[derive(Debug, Clone)]
pub struct ChangeMailTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
    pub address: String,
}
impl HaveEmployeeDao<()> for ChangeMailTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl MailChangeableEmployee for ChangeMailTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_address(&self) -> &str {
        &self.address
    }
}
impl Transaction<()> for ChangeMailTransactionImpl {
    fn execute(&mut self) -> Result<(), EmployeeUsecaseError> {
        ChangeMailTransaction::execute(self).run(&mut ())
    }
}

#[derive(Debug, Clone)]
pub struct ChangeHoldTransactionImpl {
    pub db: MockDb,

    pub emp_id: EmployeeId,
}
impl HaveEmployeeDao<()> for ChangeHoldTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl HoldChangeableEmployee for ChangeHoldTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
}
impl Transaction<()> for ChangeHoldTransactionImpl {
    fn execute(&mut self) -> Result<(), EmployeeUsecaseError> {
        ChangeHoldTransaction::execute(self).run(&mut ())
    }
}

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
    fn execute(&mut self) -> Result<(), EmployeeUsecaseError> {
        ChangeUnionMemberTransaction::execute(self).run(&mut ())
    }
}

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
impl NoAffiliationChangeableEmployee for ChangeNoMemberTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
}
impl Transaction<()> for ChangeNoMemberTransactionImpl {
    fn execute(&mut self) -> Result<(), EmployeeUsecaseError> {
        ChangeUnaffiliatedTransaction::execute(self).run(&mut ())
    }
}

#[derive(Debug, Clone)]
pub struct PaydayTransactionImpl {
    pub db: MockDb,

    pub pay_date: NaiveDate,
}
impl HaveEmployeeDao<()> for PaydayTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl PayableEmployee for PaydayTransactionImpl {
    fn get_pay_date(&self) -> NaiveDate {
        self.pay_date
    }
}
impl Transaction<()> for PaydayTransactionImpl {
    fn execute(&mut self) -> Result<(), EmployeeUsecaseError> {
        PaydayTransaction::execute(self).run(&mut ())
    }
}
