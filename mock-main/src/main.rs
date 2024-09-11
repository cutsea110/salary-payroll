use abstract_tx::UsecaseError;
use mock_app::TestPayrollApp;
use payroll_domain::{Affiliation, PaymentClassification, PaymentMethod, PaymentSchedule};
use tx_app::{Transaction, TransactionApplication};

fn main() -> Result<(), UsecaseError> {
    let mut app = TestPayrollApp::new("script/test.scr");
    app.run()?;
    println!("{:#?}", app);

    Ok(())
}

trait TransactionFactory<Ctx> {
    fn mk_add_salary_employee_tx(&self) -> impl Transaction<Ctx>;
    fn mk_add_hourly_employee_tx(&self) -> impl Transaction<Ctx>;
    fn mk_add_commissioned_employee_tx(&self) -> impl Transaction<Ctx>;
    fn mk_delete_employee_tx(&self) -> impl Transaction<Ctx>;
    fn mk_timecard_tx(&self) -> impl Transaction<Ctx>;
    fn mk_sales_receipt_tx(&self) -> impl Transaction<Ctx>;
    fn mk_service_charge_tx(&self) -> impl Transaction<Ctx>;
    fn mk_change_name_tx(&self) -> impl Transaction<Ctx>;
    fn mk_change_address_tx(&self) -> impl Transaction<Ctx>;
    fn mk_change_salary_tx(&self) -> impl Transaction<Ctx>;
    fn mk_change_hourly_tx(&self) -> impl Transaction<Ctx>;
    fn mk_change_commissioned_tx(&self) -> impl Transaction<Ctx>;
    fn mk_change_direct_tx(&self) -> impl Transaction<Ctx>;
    fn mk_change_mail_tx(&self) -> impl Transaction<Ctx>;
    fn mk_change_hold_tx(&self) -> impl Transaction<Ctx>;
    fn mk_change_union_member_tx(&self) -> impl Transaction<Ctx>;
    fn mk_change_no_member_tx(&self) -> impl Transaction<Ctx>;
    fn mk_payday_tx(&self) -> impl Transaction<Ctx>;
}

trait PayrollFactory {
    fn mk_salaried_classification(&self, salary: f64) -> impl PaymentClassification;
    fn mk_hourly_classification(&self, hourly_rate: f64) -> impl PaymentClassification;
    fn mk_commissioned_classification(
        &self,
        salary: f64,
        commission_rate: f64,
    ) -> impl PaymentClassification;

    fn mk_monthly_schedule(&self) -> impl PaymentSchedule;
    fn mk_weekly_schedule(&self) -> impl PaymentSchedule;
    fn mk_biweekly_schedule(&self) -> impl PaymentSchedule;

    fn mk_direct_method(&self, bank: String, account: String) -> impl PaymentMethod;
    fn mk_mail_method(&self, address: String) -> impl PaymentMethod;
    fn mk_hold_method(&self) -> impl PaymentMethod;

    fn mk_union_affiliation(&self, member_id: i32, dues: f64) -> impl Affiliation;
    fn mk_no_affiliation(&self) -> impl Affiliation;
}

trait PayrollApplication {}
