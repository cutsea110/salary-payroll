use payroll_domain::{Affiliation, PaymentClassification, PaymentMethod, PaymentSchedule};

pub trait PayrollFactory {
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
