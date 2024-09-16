use mock_db::MockDb;
use payroll_domain::{EmployeeId, MemberId};
use tx_app::Transaction;
use tx_factory::TransactionFactory;

#[derive(Debug)]
pub struct TransactionFactoryImpl {
    db: MockDb,
}
impl TransactionFactoryImpl {
    pub fn new(db: MockDb) -> Self {
        Self { db }
    }
}
impl TransactionFactory<()> for TransactionFactoryImpl {
    fn mk_add_salary_employee_tx(
        &self,
        emp_id: EmployeeId,
        name: String,
        address: String,
        salary: f32,
    ) -> impl Transaction<()> {
        crate::add_salaried_employee_tx::AddSalariedEmployeeTransactionImpl {
            db: self.db.clone(),
            emp_id,
            name,
            address,
            salary,
        }
    }
    fn mk_add_hourly_employee_tx(
        &self,
        emp_id: EmployeeId,
        name: String,
        address: String,
        hourly_rate: f32,
    ) -> impl Transaction<()> {
        crate::add_hourly_employee_tx::AddHourlyEmployeeTransactionImpl {
            db: self.db.clone(),
            emp_id,
            name,
            address,
            hourly_rate,
        }
    }
    fn mk_add_commissioned_employee_tx(
        &self,
        emp_id: EmployeeId,
        name: String,
        address: String,
        salary: f32,
        commission_rate: f32,
    ) -> impl Transaction<()> {
        crate::add_commissioned_employee_tx::AddCommissionedEmployeeTransactionImpl {
            db: self.db.clone(),
            emp_id,
            name,
            address,
            salary,
            commission_rate,
        }
    }
    fn mk_delete_employee_tx(&self, emp_id: EmployeeId) -> impl Transaction<()> {
        crate::delete_employee_tx::DeleteEmployeeTransactionImpl {
            db: self.db.clone(),
            emp_id,
        }
    }
    fn mk_timecard_tx(
        &self,
        emp_id: EmployeeId,
        date: chrono::NaiveDate,
        hours: f32,
    ) -> impl Transaction<()> {
        crate::timecard_tx::TimeCardTransactionImpl {
            db: self.db.clone(),
            emp_id,
            date,
            hours,
        }
    }
    fn mk_sales_receipt_tx(
        &self,
        emp_id: EmployeeId,
        date: chrono::NaiveDate,
        amount: f32,
    ) -> impl Transaction<()> {
        crate::sales_receipt_tx::SalesReceiptTransactionImpl {
            db: self.db.clone(),
            emp_id,
            date,
            amount,
        }
    }
    fn mk_change_name_tx(&self, emp_id: EmployeeId, name: String) -> impl Transaction<()> {
        crate::change_name_tx::ChangeNameTransactionImpl {
            db: self.db.clone(),
            emp_id,
            name,
        }
    }
    fn mk_change_address_tx(&self, emp_id: EmployeeId, address: String) -> impl Transaction<()> {
        crate::change_address_tx::ChangeAddressTransactionImpl {
            db: self.db.clone(),
            emp_id,
            address: address.to_string(),
        }
    }
    fn mk_change_salaried_tx(&self, emp_id: EmployeeId, salary: f32) -> impl Transaction<()> {
        crate::change_salaried_tx::ChangeSalariedTransactionImpl {
            db: self.db.clone(),
            emp_id,
            salary,
        }
    }
    fn mk_change_hourly_tx(&self, emp_id: EmployeeId, hourly_rate: f32) -> impl Transaction<()> {
        crate::change_hourly_tx::ChangeHourlyTransactionImpl {
            db: self.db.clone(),
            emp_id,
            hourly_rate,
        }
    }
    fn mk_change_commissioned_tx(
        &self,
        emp_id: EmployeeId,
        salary: f32,
        commission_rate: f32,
    ) -> impl Transaction<()> {
        crate::change_commissioned_tx::ChangeCommissionedTransactionImpl {
            db: self.db.clone(),
            emp_id,
            salary,
            commission_rate,
        }
    }
    fn mk_change_direct_tx(
        &self,
        emp_id: EmployeeId,
        bank: String,
        account: String,
    ) -> impl Transaction<()> {
        crate::change_direct_tx::ChangeDirectTransactionImpl {
            db: self.db.clone(),
            emp_id,
            bank,
            account,
        }
    }
    fn mk_change_mail_tx(&self, emp_id: EmployeeId, address: String) -> impl Transaction<()> {
        crate::change_mail_tx::ChangeMailTransactionImpl {
            db: self.db.clone(),
            emp_id,
            address,
        }
    }
    fn mk_change_hold_tx(&self, emp_id: EmployeeId) -> impl Transaction<()> {
        crate::change_hold_tx::ChangeHoldTransactionImpl {
            db: self.db.clone(),
            emp_id,
        }
    }
    fn mk_change_union_member_tx(
        &self,
        emp_id: EmployeeId,
        member_id: MemberId,
        dues: f32,
    ) -> impl Transaction<()> {
        crate::change_union_member_tx::ChangeUnionMemberTransactionImpl {
            db: self.db.clone(),
            emp_id,
            member_id,
            dues,
        }
    }
    fn mk_change_unaffiliated_tx(&self, emp_id: EmployeeId) -> impl Transaction<()> {
        crate::change_unaffiliated_tx::ChangeUnaffiliatedTransactionImpl {
            db: self.db.clone(),
            emp_id,
        }
    }
    fn mk_service_charge_tx(
        &self,
        member_id: MemberId,
        date: chrono::prelude::NaiveDate,
        amount: f32,
    ) -> impl Transaction<()> {
        crate::service_charge_tx::ServiceChargeTransactionImpl {
            db: self.db.clone(),
            member_id,
            date,
            amount,
        }
    }
    fn mk_payday_tx(&self, pay_date: chrono::NaiveDate) -> impl Transaction<()> {
        crate::payday_tx::PaydayTransactionImpl {
            db: self.db.clone(),
            pay_date,
        }
    }
}
