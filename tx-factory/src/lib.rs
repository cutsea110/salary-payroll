use tx_app::Transaction;

pub trait TransactionFactory<Ctx> {
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
