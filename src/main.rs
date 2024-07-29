use chrono::{Datelike, Days, NaiveDate, Weekday};
use core::fmt::Debug;
use dyn_clone::DynClone;
use std::{any::Any, cell::RefCell, collections::HashMap, ops::RangeInclusive, rc::Rc};
use thiserror::Error;
use tx_rs::Tx;

#[derive(Debug, Clone, Eq, PartialEq, Error)]
enum EmployeeDaoError {
    #[error("insert error: {0}")]
    InsertError(String),
    #[error("delete error: {0}")]
    DeleteError(String),
    #[error("fetch error: {0}")]
    FetchError(String),
    #[error("update error: {0}")]
    UpdateError(String),
}
trait EmployeeDao<Ctx> {
    fn insert(
        &self,
        emp: Employee,
    ) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = EmployeeDaoError>;
    fn delete(&self, emp_id: EmployeeId) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeDaoError>;
    fn fetch(
        &self,
        emp_id: EmployeeId,
    ) -> impl tx_rs::Tx<Ctx, Item = Employee, Err = EmployeeDaoError>;
    fn update(&self, emp: Employee) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeDaoError>;
    fn get_all(&self) -> impl tx_rs::Tx<Ctx, Item = Vec<Employee>, Err = EmployeeDaoError>;
    fn add_union_member(
        &self,
        member_id: MemberId,
        emp_id: EmployeeId,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeDaoError>;
    fn remove_union_member(
        &self,
        member_id: MemberId,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeDaoError>;
    fn find_union_member(
        &self,
        member_id: MemberId,
    ) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = EmployeeDaoError>;
}
trait HaveEmployeeDao<Ctx> {
    fn dao(&self) -> Box<&impl EmployeeDao<Ctx>>;
}

#[derive(Debug, Clone, Eq, PartialEq, Error)]
enum EmployeeUsecaseError {
    #[error("register employee failed: {0}")]
    RegisterEmployeeFailed(EmployeeDaoError),
    #[error("unregister employee failed: {0}")]
    UnregisterEmployeeFailed(EmployeeDaoError),
    #[error("employee not found: {0}")]
    NotFound(EmployeeDaoError),
    #[error("can't get all employees: {0}")]
    GetAllFailed(EmployeeDaoError),
    #[error("employee is not hourly salary: {0}")]
    NotHourlySalary(String),
    #[error("employee is not commissioned salary: {0}")]
    NotCommissionedSalary(String),
    #[error("update employee failed: {0}")]
    UpdateEmployeeFailed(EmployeeDaoError),
    #[error("employee is not union member: {0}")]
    NotUnionMember(String),
    #[error("add union member failed: {0}")]
    AddUnionMemberFailed(EmployeeDaoError),
    #[error("remove union member failed: {0}")]
    RemoveUnionMemberFailed(EmployeeDaoError),
}

trait AddEmployeeTransaction<Ctx>: HaveEmployeeDao<Ctx> {
    fn exec<'a>(
        &'a self,
        emp_id: EmployeeId,
        name: &str,
        address: &str,
        classification: Box<dyn PaymentClassification>,
        schedule: Box<dyn PaymentSchedule>,
    ) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        let method = Box::new(HoldMethod);
        let affiliation = Box::new(NoAffiliation);
        let emp = Employee::new(
            emp_id,
            name,
            address,
            classification,
            schedule,
            method,
            affiliation,
        );
        self.dao()
            .insert(emp)
            .map_err(EmployeeUsecaseError::RegisterEmployeeFailed)
    }
}
// blanket implementation
impl<T, Ctx> AddEmployeeTransaction<Ctx> for T where T: HaveEmployeeDao<Ctx> {}

trait AddSalaryEmployeeTransaction<Ctx>: AddEmployeeTransaction<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_name(&self) -> &str;
    fn get_address(&self) -> &str;
    fn get_salary(&self) -> f64;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        let emp_id = self.get_emp_id();
        let name = self.get_name();
        let address = self.get_address();
        let classification = Box::new(SalariedClassification {
            salary: self.get_salary(),
        });
        let schedule = Box::new(MonthlySchedule);

        self.exec(emp_id, name, address, classification, schedule)
    }
}
trait AddHourlyEmployeeTransaction<Ctx>: AddEmployeeTransaction<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_name(&self) -> &str;
    fn get_address(&self) -> &str;
    fn get_hourly_rate(&self) -> f64;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        let emp_id = self.get_emp_id();
        let name = self.get_name();
        let address = self.get_address();
        let classification = Box::new(HourlyClassification {
            hourly_rate: self.get_hourly_rate(),
            timecards: vec![],
        });
        let schedule = Box::new(WeeklySchedule);

        self.exec(emp_id, name, address, classification, schedule)
    }
}
trait AddCommissionedEmployeeTransaction<Ctx>: AddEmployeeTransaction<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_name(&self) -> &str;
    fn get_address(&self) -> &str;
    fn get_salary(&self) -> f64;
    fn get_commission_rate(&self) -> f64;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        let emp_id = self.get_emp_id();
        let name = self.get_name();
        let address = self.get_address();
        let classification = Box::new(CommissionedClassification {
            salary: self.get_salary(),
            commission_rate: self.get_commission_rate(),
            sales_receipts: vec![],
        });
        let schedule = Box::new(BiweeklySchedule);

        self.exec(emp_id, name, address, classification, schedule)
    }
}

trait DeleteEmployeeTransaction<Ctx>: HaveEmployeeDao<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        self.dao()
            .delete(self.get_emp_id())
            .map_err(EmployeeUsecaseError::UnregisterEmployeeFailed)
    }
}

trait TimeCardTransaction<Ctx>: HaveEmployeeDao<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_date(&self) -> NaiveDate;
    fn get_hours(&self) -> f64;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError> {
        tx_rs::with_tx(move |ctx| {
            let mut emp = self
                .dao()
                .fetch(self.get_emp_id())
                .run(ctx)
                .map_err(EmployeeUsecaseError::NotFound)?;
            let hourly = emp
                .classification
                .as_any_mut()
                .downcast_mut::<HourlyClassification>()
                .ok_or(EmployeeUsecaseError::NotHourlySalary(format!(
                    "emp_id: {}",
                    self.get_emp_id()
                )))?;
            hourly
                .timecards
                .push(TimeCard::new(self.get_date(), self.get_hours()));
            self.dao()
                .update(emp)
                .run(ctx)
                .map_err(EmployeeUsecaseError::UpdateEmployeeFailed)
        })
    }
}

trait SalesReceiptTransaction<Ctx>: HaveEmployeeDao<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_date(&self) -> NaiveDate;
    fn get_amount(&self) -> f64;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError> {
        tx_rs::with_tx(move |ctx| {
            let mut emp = self
                .dao()
                .fetch(self.get_emp_id())
                .run(ctx)
                .map_err(EmployeeUsecaseError::NotFound)?;
            let commissioned = emp
                .classification
                .as_any_mut()
                .downcast_mut::<CommissionedClassification>()
                .ok_or(EmployeeUsecaseError::NotCommissionedSalary(format!(
                    "emp_id: {}",
                    self.get_emp_id()
                )))?;
            commissioned
                .sales_receipts
                .push(SalesReceipt::new(self.get_date(), self.get_amount()));
            self.dao()
                .update(emp)
                .run(ctx)
                .map_err(EmployeeUsecaseError::UpdateEmployeeFailed)
        })
    }
}

trait ServiceChargeTransaction<Ctx>: HaveEmployeeDao<Ctx> {
    fn get_member_id(&self) -> MemberId;
    fn get_date(&self) -> NaiveDate;
    fn get_amount(&self) -> f64;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError> {
        tx_rs::with_tx(move |ctx| {
            let emp_id = self
                .dao()
                .find_union_member(self.get_member_id())
                .run(ctx)
                .map_err(EmployeeUsecaseError::NotFound)?;
            let mut emp = self
                .dao()
                .fetch(emp_id)
                .run(ctx)
                .map_err(EmployeeUsecaseError::NotFound)?;
            let affiliation = emp
                .affiliation
                .as_any_mut()
                .downcast_mut::<UnionAffiliation>()
                .ok_or(EmployeeUsecaseError::NotUnionMember(format!(
                    "emp_id: {0}",
                    emp_id,
                )))?;
            affiliation
                .service_charges
                .push(ServiceCharge::new(self.get_date(), self.get_amount()));
            self.dao()
                .update(emp)
                .run(ctx)
                .map_err(EmployeeUsecaseError::UpdateEmployeeFailed)
        })
    }
}

trait ChangeEmployeeTransaction<Ctx>: HaveEmployeeDao<Ctx> {
    fn exec<'a, F>(
        &'a self,
        emp_id: EmployeeId,
        f: F,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        F: FnOnce(&mut Ctx, &mut Employee) -> Result<(), EmployeeUsecaseError>,
        Ctx: 'a,
    {
        tx_rs::with_tx(move |ctx| {
            let mut emp = self
                .dao()
                .fetch(emp_id)
                .run(ctx)
                .map_err(EmployeeUsecaseError::NotFound)?;
            f(ctx, &mut emp)?;
            self.dao()
                .update(emp)
                .run(ctx)
                .map_err(EmployeeUsecaseError::UpdateEmployeeFailed)
        })
    }
}
// blanket implementation
impl<Ctx, T> ChangeEmployeeTransaction<Ctx> for T where T: HaveEmployeeDao<Ctx> {}

trait ChangeNameTransaction<Ctx>: ChangeEmployeeTransaction<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_name(&self) -> &str;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        self.exec(self.get_emp_id(), |_ctx, emp| {
            emp.set_name(self.get_name());
            Ok(())
        })
    }
}
trait ChangeAddressTransaction<Ctx>: ChangeEmployeeTransaction<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_address(&self) -> &str;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        self.exec(self.get_emp_id(), |_ctx, emp| {
            emp.set_address(self.get_address());
            Ok(())
        })
    }
}
trait ChangeClassificationTransaction<Ctx>: ChangeEmployeeTransaction<Ctx> {
    fn exec_classification<'a>(
        &'a self,
        emp_id: EmployeeId,
        classification: Box<dyn PaymentClassification>,
        schedule: Box<dyn PaymentSchedule>,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        self.exec(emp_id, |_ctx, emp| {
            emp.set_classification(classification);
            emp.set_schedule(schedule);
            Ok(())
        })
    }
}
// blanket implementation
impl<Ctx, T> ChangeClassificationTransaction<Ctx> for T where T: ChangeEmployeeTransaction<Ctx> {}

trait ChangeSalariedTransaction<Ctx>: ChangeClassificationTransaction<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_salary(&self) -> f64;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        self.exec_classification(
            self.get_emp_id(),
            Box::new(SalariedClassification {
                salary: self.get_salary(),
            }),
            Box::new(MonthlySchedule),
        )
    }
}
trait ChangeHourlyTransaction<Ctx>: ChangeClassificationTransaction<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_hourly_rate(&self) -> f64;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        self.exec_classification(
            self.get_emp_id(),
            Box::new(HourlyClassification {
                hourly_rate: self.get_hourly_rate(),
                timecards: vec![],
            }),
            Box::new(WeeklySchedule),
        )
    }
}
trait ChangeCommissionedTransaction<Ctx>: ChangeClassificationTransaction<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_salary(&self) -> f64;
    fn get_commission_rate(&self) -> f64;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        self.exec_classification(
            self.get_emp_id(),
            Box::new(CommissionedClassification {
                salary: self.get_salary(),
                commission_rate: self.get_commission_rate(),
                sales_receipts: vec![],
            }),
            Box::new(BiweeklySchedule),
        )
    }
}

trait ChangeMethodTransaction<Ctx>: ChangeEmployeeTransaction<Ctx> {
    fn exec_method<'a>(
        &'a self,
        emp_id: EmployeeId,
        method: Box<dyn PaymentMethod>,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        self.exec(emp_id, |_ctx, emp| {
            emp.set_method(method);
            Ok(())
        })
    }
}
// blanket implementation
impl<Ctx, T> ChangeMethodTransaction<Ctx> for T where T: ChangeEmployeeTransaction<Ctx> {}

trait ChangeDirectTransaction<Ctx>: ChangeMethodTransaction<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_bank(&self) -> &str;
    fn get_account(&self) -> &str;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        self.exec_method(
            self.get_emp_id(),
            Box::new(DirectMethod {
                bank: self.get_bank().to_string(),
                account: self.get_account().to_string(),
            }),
        )
    }
}
trait ChangeMailTransaction<Ctx>: ChangeMethodTransaction<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_address(&self) -> &str;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        self.exec_method(
            self.get_emp_id(),
            Box::new(MailMethod {
                address: self.get_address().to_string(),
            }),
        )
    }
}
trait ChangeHoldTransaction<Ctx>: ChangeMethodTransaction<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        self.exec_method(self.get_emp_id(), Box::new(HoldMethod {}))
    }
}

trait ChangeAffiliationTransaction<Ctx>: ChangeEmployeeTransaction<Ctx> {
    fn exec_affiliation<'a, F>(
        &'a self,
        emp_id: EmployeeId,
        record_membership: F,
        affiliation: Box<dyn Affiliation>,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        F: FnOnce(&mut Ctx, &mut Employee) -> Result<(), EmployeeUsecaseError> + 'a,
        Ctx: 'a,
    {
        self.exec(emp_id, |ctx, emp| {
            record_membership(ctx, emp)?;
            emp.set_affiliation(affiliation);
            Ok(())
        })
    }
}
// blanket implementation
impl<Ctx, T> ChangeAffiliationTransaction<Ctx> for T where T: ChangeEmployeeTransaction<Ctx> {}

trait ChangeUnionMemberTransaction<Ctx>: ChangeAffiliationTransaction<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_member_id(&self) -> MemberId;
    fn get_dues(&self) -> f64;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        self.exec_affiliation(
            self.get_emp_id(),
            |ctx, _emp| {
                self.dao()
                    .add_union_member(self.get_member_id(), self.get_emp_id())
                    .run(ctx)
                    .map_err(EmployeeUsecaseError::AddUnionMemberFailed)
            },
            Box::new(UnionAffiliation {
                member_id: self.get_member_id(),
                dues: self.get_dues(),
                service_charges: vec![],
            }),
        )
    }
}
trait ChangeUnaffiliatedTransaction<Ctx>: ChangeAffiliationTransaction<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        self.exec_affiliation(
            self.get_emp_id(),
            |ctx, emp| {
                let member_id = emp
                    .get_affiliation()
                    .as_any()
                    .downcast_ref::<UnionAffiliation>()
                    .map_or(
                        Err(EmployeeUsecaseError::NotUnionMember(format!(
                            "emp_id: {}",
                            self.get_emp_id()
                        ))),
                        |a| Ok(a.member_id),
                    )?;
                self.dao()
                    .remove_union_member(member_id)
                    .run(ctx)
                    .map_err(EmployeeUsecaseError::RemoveUnionMemberFailed)
            },
            Box::new(NoAffiliation),
        )
    }
}

trait PaydayTransaction<Ctx>: HaveEmployeeDao<Ctx> {
    fn get_pay_date(&self) -> NaiveDate;
    fn record_paycheck(&mut self, pc: PayCheck);

    fn execute<'a>(&mut self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        tx_rs::with_tx(|ctx| {
            let mut employees = self
                .dao()
                .get_all()
                .run(ctx)
                .map_err(EmployeeUsecaseError::GetAllFailed)?;
            let pay_date = self.get_pay_date();
            for emp in employees.iter_mut() {
                if emp.is_pay_date(pay_date) {
                    let period = emp.get_pay_period(pay_date);
                    let mut pc = PayCheck::new(period);
                    emp.payday(&mut pc);
                    self.record_paycheck(pc);
                }
            }
            Ok(())
        })
    }
}

trait PaymentClassification: DynClone + Debug {
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn calculate_pay(&self, pc: &PayCheck) -> f64;
}
dyn_clone::clone_trait_object!(PaymentClassification);
#[derive(Debug, Clone, PartialEq)]
struct SalariedClassification {
    salary: f64,
}
impl PaymentClassification for SalariedClassification {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn calculate_pay(&self, _pc: &PayCheck) -> f64 {
        self.salary
    }
}
#[derive(Debug, Clone, PartialEq)]
struct HourlyClassification {
    hourly_rate: f64,
    timecards: Vec<TimeCard>,
}
impl PaymentClassification for HourlyClassification {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn calculate_pay(&self, pc: &PayCheck) -> f64 {
        let pay_period = pc.get_pay_period();
        let mut total_pay = 0.0;
        for tc in self.timecards.iter() {
            if pay_period.contains(&tc.get_date()) {
                total_pay += self.calculate_pay_for_timecard(tc);
            }
        }
        total_pay
    }
}
impl HourlyClassification {
    fn calculate_pay_for_timecard(&self, tc: &TimeCard) -> f64 {
        let hours = tc.get_hours();
        let overtime = (hours - 8.0).max(0.0);
        let straight_time = hours - overtime;
        straight_time * self.hourly_rate + overtime * self.hourly_rate * 1.5
    }
}
#[derive(Debug, Clone, PartialEq)]
struct CommissionedClassification {
    salary: f64,
    commission_rate: f64,
    sales_receipts: Vec<SalesReceipt>,
}
impl PaymentClassification for CommissionedClassification {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn calculate_pay(&self, pc: &PayCheck) -> f64 {
        let mut total_pay = 0.0;
        let pay_period = pc.get_pay_period();
        for sr in self.sales_receipts.iter() {
            if pay_period.contains(&sr.get_date()) {
                total_pay += self.calculate_pay_for_sales_receipt(sr);
            }
        }
        total_pay
    }
}
impl CommissionedClassification {
    fn calculate_pay_for_sales_receipt(&self, sr: &SalesReceipt) -> f64 {
        self.commission_rate * sr.get_amount()
    }
}

trait PaymentSchedule: DynClone + Debug {
    fn is_pay_date(&self, date: NaiveDate) -> bool;
    fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate>;
}
dyn_clone::clone_trait_object!(PaymentSchedule);
#[derive(Debug, Clone, Eq, PartialEq)]
struct MonthlySchedule;
impl PaymentSchedule for MonthlySchedule {
    fn is_pay_date(&self, date: NaiveDate) -> bool {
        self.is_last_day_of_month(date)
    }
    fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
        // pay_date should be last_day of month
        pay_date.with_day(1).unwrap()..=pay_date
    }
}
impl MonthlySchedule {
    fn is_last_day_of_month(&self, date: NaiveDate) -> bool {
        date.month() != date.checked_add_days(Days::new(1)).unwrap().month()
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
struct WeeklySchedule;
impl PaymentSchedule for WeeklySchedule {
    fn is_pay_date(&self, date: NaiveDate) -> bool {
        date.weekday() == Weekday::Fri
    }
    fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
        pay_date.checked_sub_days(Days::new(6)).unwrap()..=pay_date
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
struct BiweeklySchedule;
impl PaymentSchedule for BiweeklySchedule {
    fn is_pay_date(&self, date: NaiveDate) -> bool {
        date.weekday() == Weekday::Fri && date.iso_week().week() % 2 == 0
    }
    fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
        pay_date.checked_sub_days(Days::new(13)).unwrap()..=pay_date
    }
}

trait PaymentMethod: DynClone + Debug {
    // TODO: return type
    fn pay(&self, pc: &PayCheck);
}
dyn_clone::clone_trait_object!(PaymentMethod);
#[derive(Debug, Clone, Eq, PartialEq)]
struct HoldMethod;
impl PaymentMethod for HoldMethod {
    fn pay(&self, pc: &PayCheck) {
        // concrete implementation
        println!("HoldMethod: {:?}", pc);
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
struct MailMethod {
    address: String,
}
impl PaymentMethod for MailMethod {
    fn pay(&self, pc: &PayCheck) {
        // concrete implementation
        println!("MailMethod for {}: {:?}", self.address, pc);
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
struct DirectMethod {
    bank: String,
    account: String,
}
impl PaymentMethod for DirectMethod {
    fn pay(&self, pc: &PayCheck) {
        // concrete implementation
        println!("DirectMethod to {}{}: {:?}", self.bank, self.account, pc);
    }
}

trait Affiliation: DynClone + Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn calculate_deductions(&self, _pc: &PayCheck) -> f64 {
        0.0
    }
}
dyn_clone::clone_trait_object!(Affiliation);
#[derive(Debug, Clone, PartialEq)]
struct UnionAffiliation {
    member_id: MemberId,
    dues: f64,

    service_charges: Vec<ServiceCharge>,
}
impl UnionAffiliation {
    fn get_dues(&self) -> f64 {
        self.dues
    }
}
impl Affiliation for UnionAffiliation {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn calculate_deductions(&self, pc: &PayCheck) -> f64 {
        let mut total_deductions = 0.0;
        let pay_period = pc.get_pay_period();
        for d in pc.get_pay_period().start().iter_days() {
            if d > *pay_period.end() {
                break;
            }
            if d.weekday() == Weekday::Fri {
                total_deductions += self.get_dues();
            }
        }
        for sc in self.service_charges.iter() {
            if pay_period.contains(&sc.get_date()) {
                total_deductions += sc.get_amount();
            }
        }
        total_deductions
    }
}
#[derive(Debug, Clone, PartialEq)]
struct NoAffiliation;
impl Affiliation for NoAffiliation {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

type EmployeeId = u32;
type MemberId = u32;

#[derive(Debug, Clone)]
struct Employee {
    emp_id: EmployeeId,
    name: String,
    address: String,
    classification: Box<dyn PaymentClassification>,
    schedule: Box<dyn PaymentSchedule>,
    method: Box<dyn PaymentMethod>,
    affiliation: Box<dyn Affiliation>,
}
impl Employee {
    pub fn new(
        emp_id: EmployeeId,
        name: &str,
        address: &str,
        classification: Box<dyn PaymentClassification>,
        schedule: Box<dyn PaymentSchedule>,
        method: Box<dyn PaymentMethod>,
        affiliation: Box<dyn Affiliation>,
    ) -> Self {
        Self {
            emp_id,
            name: name.to_string(),
            address: address.to_string(),
            classification,
            schedule,
            method,
            affiliation,
        }
    }
    pub fn is_pay_date(&self, date: NaiveDate) -> bool {
        self.schedule.is_pay_date(date)
    }
    pub fn get_pay_period(&self, date: NaiveDate) -> RangeInclusive<NaiveDate> {
        self.schedule.get_pay_period(date)
    }
    pub fn payday(&self, pc: &mut PayCheck) {
        let gross_pay = self.classification.calculate_pay(&pc);
        let deductions = self.affiliation.calculate_deductions(&pc);
        let net_pay = gross_pay - deductions;
        pc.set_gross_pay(gross_pay);
        pc.set_deductions(deductions);
        pc.set_net_pay(net_pay);
        self.method.pay(pc);
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    pub fn set_address(&mut self, address: &str) {
        self.address = address.to_string();
    }
    pub fn set_classification(&mut self, classification: Box<dyn PaymentClassification>) {
        self.classification = classification;
    }
    pub fn set_schedule(&mut self, schedule: Box<dyn PaymentSchedule>) {
        self.schedule = schedule;
    }
    pub fn set_method(&mut self, method: Box<dyn PaymentMethod>) {
        self.method = method;
    }
    pub fn get_affiliation(&self) -> Box<dyn Affiliation> {
        self.affiliation.clone()
    }
    pub fn set_affiliation(&mut self, affiliation: Box<dyn Affiliation>) {
        self.affiliation = affiliation;
    }
}

#[derive(Debug, Clone, PartialEq)]
struct PayCheck {
    period: RangeInclusive<NaiveDate>,

    gross_pay: f64,
    deductions: f64,
    net_pay: f64,
}
impl PayCheck {
    fn new(period: RangeInclusive<NaiveDate>) -> Self {
        Self {
            period,
            gross_pay: 0.0,
            deductions: 0.0,
            net_pay: 0.0,
        }
    }
    fn get_pay_period(&self) -> RangeInclusive<NaiveDate> {
        self.period.clone()
    }
    fn set_gross_pay(&mut self, gross_pay: f64) {
        self.gross_pay = gross_pay;
    }
    fn set_deductions(&mut self, deductions: f64) {
        self.deductions = deductions;
    }
    fn set_net_pay(&mut self, net_pay: f64) {
        self.net_pay = net_pay;
    }
}

#[derive(Debug, Clone, PartialEq)]
struct TimeCard {
    date: NaiveDate,
    hours: f64,
}
impl TimeCard {
    fn new(date: NaiveDate, hours: f64) -> Self {
        Self { date, hours }
    }
    fn get_date(&self) -> NaiveDate {
        self.date
    }
    fn get_hours(&self) -> f64 {
        self.hours
    }
}

#[derive(Debug, Clone, PartialEq)]
struct SalesReceipt {
    date: NaiveDate,
    amount: f64,
}
impl SalesReceipt {
    fn new(date: NaiveDate, amount: f64) -> Self {
        Self { date, amount }
    }
    fn get_date(&self) -> NaiveDate {
        self.date
    }
    fn get_amount(&self) -> f64 {
        self.amount
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ServiceCharge {
    date: NaiveDate,
    amount: f64,
}
impl ServiceCharge {
    fn new(date: NaiveDate, amount: f64) -> Self {
        Self { date, amount }
    }
    fn get_date(&self) -> NaiveDate {
        self.date
    }
    fn get_amount(&self) -> f64 {
        self.amount
    }
}

#[derive(Debug, Clone)]
struct MockDb {
    employees: Rc<RefCell<HashMap<EmployeeId, Employee>>>,
    union_members: Rc<RefCell<HashMap<MemberId, EmployeeId>>>,
}
impl EmployeeDao<()> for MockDb {
    fn insert(
        &self,
        emp: Employee,
    ) -> impl tx_rs::Tx<(), Item = EmployeeId, Err = EmployeeDaoError> {
        tx_rs::with_tx(move |_| {
            let emp_id = emp.emp_id;
            if self.employees.borrow().contains_key(&emp_id) {
                return Err(EmployeeDaoError::InsertError(format!(
                    "emp_id={} already exists",
                    emp_id
                )));
            }
            self.employees.borrow_mut().insert(emp_id, emp);
            Ok(emp_id)
        })
    }
    fn delete(&self, emp_id: EmployeeId) -> impl tx_rs::Tx<(), Item = (), Err = EmployeeDaoError> {
        tx_rs::with_tx(move |_| {
            if self.employees.borrow_mut().remove(&emp_id).is_none() {
                return Err(EmployeeDaoError::DeleteError(format!(
                    "emp_id={} not found",
                    emp_id
                )));
            }
            Ok(())
        })
    }
    fn fetch(
        &self,
        emp_id: EmployeeId,
    ) -> impl tx_rs::Tx<(), Item = Employee, Err = EmployeeDaoError> {
        tx_rs::with_tx(move |_| match self.employees.borrow().get(&emp_id) {
            Some(emp) => Ok(emp.clone()),
            None => Err(EmployeeDaoError::FetchError(format!(
                "emp_id={} not found",
                emp_id
            ))),
        })
    }
    fn update(&self, emp: Employee) -> impl tx_rs::Tx<(), Item = (), Err = EmployeeDaoError> {
        tx_rs::with_tx(move |_| {
            let emp_id = emp.emp_id;
            if !self.employees.borrow().contains_key(&emp_id) {
                return Err(EmployeeDaoError::UpdateError(format!(
                    "emp_id={} not found",
                    emp_id
                )));
            }
            self.employees.borrow_mut().insert(emp_id, emp);
            Ok(())
        })
    }
    fn get_all(&self) -> impl tx_rs::Tx<(), Item = Vec<Employee>, Err = EmployeeDaoError> {
        tx_rs::with_tx(move |_| Ok(self.employees.borrow().values().cloned().collect()))
    }

    fn add_union_member(
        &self,
        member_id: MemberId,
        emp_id: EmployeeId,
    ) -> impl tx_rs::Tx<(), Item = (), Err = EmployeeDaoError> {
        tx_rs::with_tx(move |_| {
            if self.union_members.borrow().contains_key(&member_id) {
                return Err(EmployeeDaoError::InsertError(format!(
                    "member_id={} already exists",
                    member_id
                )));
            }
            if self.union_members.borrow().values().any(|&v| v == emp_id) {
                return Err(EmployeeDaoError::InsertError(format!(
                    "emp_id={} already exists",
                    emp_id
                )));
            }
            self.union_members.borrow_mut().insert(member_id, emp_id);
            Ok(())
        })
    }
    fn remove_union_member(
        &self,
        member_id: MemberId,
    ) -> impl tx_rs::Tx<(), Item = (), Err = EmployeeDaoError> {
        tx_rs::with_tx(move |_| {
            if self.union_members.borrow_mut().remove(&member_id).is_none() {
                return Err(EmployeeDaoError::DeleteError(format!(
                    "member_id={} not found",
                    member_id
                )));
            }
            Ok(())
        })
    }

    fn find_union_member(
        &self,
        member_id: MemberId,
    ) -> impl tx_rs::Tx<(), Item = EmployeeId, Err = EmployeeDaoError> {
        tx_rs::with_tx(move |_| match self.union_members.borrow().get(&member_id) {
            Some(&emp_id) => Ok(emp_id),
            None => Err(EmployeeDaoError::FetchError(format!(
                "member_id={} not found",
                member_id
            ))),
        })
    }
}

struct AddSalariedEmployeeTransactionImpl {
    db: MockDb,

    emp_id: EmployeeId,
    name: String,
    address: String,
    salary: f64,
}
impl HaveEmployeeDao<()> for AddSalariedEmployeeTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl AddSalaryEmployeeTransaction<()> for AddSalariedEmployeeTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_address(&self) -> &str {
        &self.address
    }
    fn get_salary(&self) -> f64 {
        self.salary
    }
}

struct AddHourlyEmployeeTransactionImpl {
    db: MockDb,

    emp_id: EmployeeId,
    name: String,
    address: String,
    hourly_rate: f64,
}
impl HaveEmployeeDao<()> for AddHourlyEmployeeTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl AddHourlyEmployeeTransaction<()> for AddHourlyEmployeeTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_address(&self) -> &str {
        &self.address
    }
    fn get_hourly_rate(&self) -> f64 {
        self.hourly_rate
    }
}

struct AddCommissionedEmployeeTransactionImpl {
    db: MockDb,

    emp_id: EmployeeId,
    name: String,
    address: String,
    salary: f64,
    commission_rate: f64,
}
impl HaveEmployeeDao<()> for AddCommissionedEmployeeTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl AddCommissionedEmployeeTransaction<()> for AddCommissionedEmployeeTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_address(&self) -> &str {
        &self.address
    }
    fn get_salary(&self) -> f64 {
        self.salary
    }
    fn get_commission_rate(&self) -> f64 {
        self.commission_rate
    }
}

struct DeleteEmployeeTransactionImpl {
    db: MockDb,

    emp_id: EmployeeId,
}
impl HaveEmployeeDao<()> for DeleteEmployeeTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl DeleteEmployeeTransaction<()> for DeleteEmployeeTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
}

struct TimeCardTransactionImpl {
    db: MockDb,

    emp_id: EmployeeId,
    date: NaiveDate,
    hours: f64,
}
impl HaveEmployeeDao<()> for TimeCardTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl TimeCardTransaction<()> for TimeCardTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_date(&self) -> NaiveDate {
        self.date
    }
    fn get_hours(&self) -> f64 {
        self.hours
    }
}

struct SalesReceiptTransactionImpl {
    db: MockDb,

    emp_id: EmployeeId,
    date: NaiveDate,
    amount: f64,
}
impl HaveEmployeeDao<()> for SalesReceiptTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl SalesReceiptTransaction<()> for SalesReceiptTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_date(&self) -> NaiveDate {
        self.date
    }
    fn get_amount(&self) -> f64 {
        self.amount
    }
}

struct ServiceChargeTransactionImpl {
    db: MockDb,

    member_id: MemberId,
    date: NaiveDate,
    amount: f64,
}
impl HaveEmployeeDao<()> for ServiceChargeTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl ServiceChargeTransaction<()> for ServiceChargeTransactionImpl {
    fn get_member_id(&self) -> MemberId {
        self.member_id
    }
    fn get_date(&self) -> NaiveDate {
        self.date
    }
    fn get_amount(&self) -> f64 {
        self.amount
    }
}

struct ChangeNameTransactionImpl {
    db: MockDb,

    emp_id: EmployeeId,
    name: String,
}
impl HaveEmployeeDao<()> for ChangeNameTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl ChangeNameTransaction<()> for ChangeNameTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}

struct ChangeAddressTransactionImpl {
    db: MockDb,

    emp_id: EmployeeId,
    address: String,
}
impl HaveEmployeeDao<()> for ChangeAddressTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl ChangeAddressTransaction<()> for ChangeAddressTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_address(&self) -> &str {
        &self.address
    }
}

struct ChangeSalaryTransactionImpl {
    db: MockDb,

    emp_id: EmployeeId,
    salary: f64,
}
impl HaveEmployeeDao<()> for ChangeSalaryTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl ChangeSalariedTransaction<()> for ChangeSalaryTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_salary(&self) -> f64 {
        self.salary
    }
}

struct ChangeHourlyTransactionImpl {
    db: MockDb,

    emp_id: EmployeeId,
    hourly_rate: f64,
}
impl HaveEmployeeDao<()> for ChangeHourlyTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl ChangeHourlyTransaction<()> for ChangeHourlyTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_hourly_rate(&self) -> f64 {
        self.hourly_rate
    }
}

struct ChangeCommissionedTransactionImpl {
    db: MockDb,

    emp_id: EmployeeId,
    salary: f64,
    commission_rate: f64,
}
impl HaveEmployeeDao<()> for ChangeCommissionedTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl ChangeCommissionedTransaction<()> for ChangeCommissionedTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_salary(&self) -> f64 {
        self.salary
    }
    fn get_commission_rate(&self) -> f64 {
        self.commission_rate
    }
}

struct ChangeDirectTransactionImpl {
    db: MockDb,

    emp_id: EmployeeId,
    bank: String,
    account: String,
}
impl HaveEmployeeDao<()> for ChangeDirectTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl ChangeDirectTransaction<()> for ChangeDirectTransactionImpl {
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

struct ChangeMailTransactionImpl {
    db: MockDb,

    emp_id: EmployeeId,
    address: String,
}
impl HaveEmployeeDao<()> for ChangeMailTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl ChangeMailTransaction<()> for ChangeMailTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_address(&self) -> &str {
        &self.address
    }
}

struct ChangeHoldTransactionImpl {
    db: MockDb,

    emp_id: EmployeeId,
}
impl HaveEmployeeDao<()> for ChangeHoldTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl ChangeHoldTransaction<()> for ChangeHoldTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
}

struct ChangeUnionMemberTransactionImpl {
    db: MockDb,

    emp_id: EmployeeId,
    member_id: EmployeeId,
    dues: f64,
}
impl HaveEmployeeDao<()> for ChangeUnionMemberTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl ChangeUnionMemberTransaction<()> for ChangeUnionMemberTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_member_id(&self) -> EmployeeId {
        self.member_id
    }
    fn get_dues(&self) -> f64 {
        self.dues
    }
}

struct ChangeNoMemberTransactionImpl {
    db: MockDb,

    emp_id: EmployeeId,
}
impl HaveEmployeeDao<()> for ChangeNoMemberTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl ChangeUnaffiliatedTransaction<()> for ChangeNoMemberTransactionImpl {
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
}

struct PaydayTransactionImpl {
    db: MockDb,

    pay_date: NaiveDate,
    paychecks: Vec<PayCheck>,
}
impl HaveEmployeeDao<()> for PaydayTransactionImpl {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl PaydayTransaction<()> for PaydayTransactionImpl {
    fn get_pay_date(&self) -> NaiveDate {
        self.pay_date
    }
    fn record_paycheck(&mut self, pc: PayCheck) {
        self.paychecks.push(pc);
    }
}

fn main() {
    let db = MockDb {
        employees: Rc::new(RefCell::new(HashMap::new())),
        union_members: Rc::new(RefCell::new(HashMap::new())),
    };

    let req = AddSalariedEmployeeTransactionImpl {
        db: db.clone(),
        emp_id: 1,
        name: "Bob".to_string(),
        address: "Home".to_string(),
        salary: 1000.00,
    };
    let emp_id = req.execute().run(&mut ()).expect("add employee");
    println!("emp_id: {:?}", emp_id);
    println!("registered: {:#?}", db);

    let req = ChangeNameTransactionImpl {
        db: db.clone(),
        emp_id: 1,
        name: "Robert".to_string(),
    };
    let _ = req.execute().run(&mut ()).expect("change name");
    println!("name changed: {:#?}", db);

    let req = ChangeAddressTransactionImpl {
        db: db.clone(),
        emp_id: 1,
        address: "Office".to_string(),
    };
    let _ = req.execute().run(&mut ()).expect("change address");
    println!("address changed: {:#?}", db);

    let req = AddHourlyEmployeeTransactionImpl {
        db: db.clone(),
        emp_id: 2,
        name: "Bill".to_string(),
        address: "Home".to_string(),
        hourly_rate: 15.25,
    };
    let emp_id = req.execute().run(&mut ()).expect("add employee");
    println!("emp_id: {:?}", emp_id);
    println!("registered: {:#?}", db);

    let req = TimeCardTransactionImpl {
        db: db.clone(),
        emp_id: 2,
        date: NaiveDate::from_ymd_opt(2024, 7, 25).unwrap(),
        hours: 8.0,
    };
    let _ = req.execute().run(&mut ()).expect("time card");

    let req = AddCommissionedEmployeeTransactionImpl {
        db: db.clone(),
        emp_id: 3,
        name: "Lance".to_string(),
        address: "Home".to_string(),
        salary: 2500.00,
        commission_rate: 3.2,
    };
    let emp_id = req.execute().run(&mut ()).expect("add employee");
    println!("emp_id: {:?}", emp_id);
    println!("registered: {:#?}", db);

    let req = SalesReceiptTransactionImpl {
        db: db.clone(),
        emp_id: 3,
        date: NaiveDate::from_ymd_opt(2024, 7, 25).unwrap(),
        amount: 1000.00,
    };
    let _ = req.execute().run(&mut ()).expect("sales receipt");

    let req = AddSalariedEmployeeTransactionImpl {
        db: db.clone(),
        emp_id: 4,
        name: "Anna".to_string(),
        address: "Home".to_string(),
        salary: 1500.00,
    };
    let emp_id = req.execute().run(&mut ()).expect("add employee");
    println!("emp_id: {:?}", emp_id);
    println!("registered: {:#?}", db);

    let req = ChangeHourlyTransactionImpl {
        db: db.clone(),
        emp_id: 4,
        hourly_rate: 20.00,
    };
    let _ = req.execute().run(&mut ()).expect("change hourly");
    println!("change hourly: {:#?}", db);

    let req = ChangeCommissionedTransactionImpl {
        db: db.clone(),
        emp_id: 4,
        salary: 2000.00,
        commission_rate: 2.5,
    };
    let _ = req.execute().run(&mut ()).expect("change commissioned");
    println!("change commissioned: {:#?}", db);

    let req = ChangeSalaryTransactionImpl {
        db: db.clone(),
        emp_id: 4,
        salary: 3000.00,
    };
    let _ = req.execute().run(&mut ()).expect("change salary");
    println!("change salary: {:#?}", db);

    let req = ChangeDirectTransactionImpl {
        db: db.clone(),
        emp_id: 4,
        bank: "mufg".to_string(),
        account: "1234567".to_string(),
    };
    let _ = req.execute().run(&mut ()).expect("change direct");
    println!("change direct: {:#?}", db);

    let req = ChangeMailTransactionImpl {
        db: db.clone(),
        emp_id: 4,
        address: "alice@gmail.com".to_string(),
    };
    let _ = req.execute().run(&mut ()).expect("change mail");
    println!("change mail: {:#?}", db);

    let req = ChangeHoldTransactionImpl {
        db: db.clone(),
        emp_id: 4,
    };
    let _ = req.execute().run(&mut ()).expect("change hold");
    println!("change hold: {:#?}", db);

    let req = ChangeUnionMemberTransactionImpl {
        db: db.clone(),
        emp_id: 4,
        member_id: 7734,
        dues: 99.42,
    };
    let _ = req.execute().run(&mut ()).expect("change union member");
    println!("change union member: {:#?}", db);

    let req = ServiceChargeTransactionImpl {
        db: db.clone(),
        member_id: 7734,
        date: NaiveDate::from_ymd_opt(2024, 7, 25).unwrap(),
        amount: 12.95,
    };
    let _ = req.execute().run(&mut ()).expect("service charge");
    println!("service charge: {:#?}", db);

    let req = ChangeNoMemberTransactionImpl {
        db: db.clone(),
        emp_id: 4,
    };
    let _ = req.execute().run(&mut ()).expect("change no member");
    println!("remove union member: {:#?}", db);

    for emp_id in 1..=4 {
        let req = DeleteEmployeeTransactionImpl {
            db: db.clone(),
            emp_id,
        };
        let _ = req.execute().run(&mut ()).expect("delete employee");
        println!("deleted: {:#?}", db);
    }

    // payday
    let req = AddSalariedEmployeeTransactionImpl {
        db: db.clone(),
        emp_id: 1,
        name: "Bob".to_string(),
        address: "Home".to_string(),
        salary: 1000.00,
    };
    let emp_id = req.execute().run(&mut ()).expect("add employee");
    println!("emp_id: {:?}", emp_id);
    println!("registered: {:#?}", db);
    let mut req = PaydayTransactionImpl {
        db: db.clone(),
        pay_date: NaiveDate::from_ymd_opt(2024, 7, 29).unwrap(),
        paychecks: vec![],
    };
    let _ = req.execute().run(&mut ()).expect("payday");
    println!("paychecks: {:#?}", req.paychecks);

    let mut req = PaydayTransactionImpl {
        db: db.clone(),
        pay_date: NaiveDate::from_ymd_opt(2024, 7, 31).unwrap(),
        paychecks: vec![],
    };
    let _ = req.execute().run(&mut ()).expect("payday");
    println!("paychecks: {:#?}", req.paychecks);

    let req = DeleteEmployeeTransactionImpl {
        db: db.clone(),
        emp_id: 1,
    };
    let _ = req.execute().run(&mut ()).expect("delete employee");

    let req = AddHourlyEmployeeTransactionImpl {
        db: db.clone(),
        emp_id: 2,
        name: "Bill".to_string(),
        address: "Home".to_string(),
        hourly_rate: 15.25,
    };
    let emp_id = req.execute().run(&mut ()).expect("add employee");
    println!("emp_id: {:?}", emp_id);
    println!("registered: {:#?}", db);

    let mut req = PaydayTransactionImpl {
        db: db.clone(),
        pay_date: NaiveDate::from_ymd_opt(2024, 7, 26).unwrap(), // Friday
        paychecks: vec![],
    };
    let _ = req.execute().run(&mut ()).expect("payday");
    println!("paychecks: {:#?}", req.paychecks);

    let req = TimeCardTransactionImpl {
        db: db.clone(),
        emp_id: 2,
        date: NaiveDate::from_ymd_opt(2024, 7, 26).unwrap(),
        hours: 2.0,
    };
    let _ = req.execute().run(&mut ()).expect("time card");

    let mut req = PaydayTransactionImpl {
        db: db.clone(),
        pay_date: NaiveDate::from_ymd_opt(2024, 7, 26).unwrap(), // Friday
        paychecks: vec![],
    };
    let _ = req.execute().run(&mut ()).expect("payday");
    println!("paychecks: {:#?}", req.paychecks);

    let req = TimeCardTransactionImpl {
        db: db.clone(),
        emp_id: 2,
        date: NaiveDate::from_ymd_opt(2024, 8, 9).unwrap(),
        hours: 9.0,
    };
    let _ = req.execute().run(&mut ()).expect("time card");

    let mut req = PaydayTransactionImpl {
        db: db.clone(),
        pay_date: NaiveDate::from_ymd_opt(2024, 8, 9).unwrap(), // Friday
        paychecks: vec![],
    };
    let _ = req.execute().run(&mut ()).expect("payday");
    println!("paychecks: {:#?}", req.paychecks);

    let req = TimeCardTransactionImpl {
        db: db.clone(),
        emp_id: 2,
        date: NaiveDate::from_ymd_opt(2024, 7, 25).unwrap(),
        hours: 5.0,
    };
    let _ = req.execute().run(&mut ()).expect("time card");

    let mut req = PaydayTransactionImpl {
        db: db.clone(),
        pay_date: NaiveDate::from_ymd_opt(2024, 7, 26).unwrap(), // Friday
        paychecks: vec![],
    };
    let _ = req.execute().run(&mut ()).expect("payday");
    println!("paychecks: {:#?}", req.paychecks);

    let mut req = PaydayTransactionImpl {
        db: db.clone(),
        pay_date: NaiveDate::from_ymd_opt(2024, 8, 8).unwrap(), // Thursday
        paychecks: vec![],
    };
    let _ = req.execute().run(&mut ()).expect("payday");
    println!("paychecks: {:#?}", req.paychecks);

    let req = ChangeUnionMemberTransactionImpl {
        db: db.clone(),
        emp_id: 2,
        member_id: 7734,
        dues: 9.42,
    };
    let _ = req.execute().run(&mut ()).expect("change union member");
    println!("emp_id: {:?}", emp_id);
    println!("registered: {:#?}", db);

    let mut req = PaydayTransactionImpl {
        db: db.clone(),
        pay_date: NaiveDate::from_ymd_opt(2024, 8, 9).unwrap(),
        paychecks: vec![],
    };
    let _ = req.execute().run(&mut ()).expect("payday");
    println!("paychecks: {:#?}", req.paychecks);

    let req = ServiceChargeTransactionImpl {
        db: db.clone(),
        member_id: 7734,
        date: NaiveDate::from_ymd_opt(2024, 8, 9).unwrap(),
        amount: 19.40,
    };
    let _ = req.execute().run(&mut ()).expect("service charge");

    let mut req = PaydayTransactionImpl {
        db: db.clone(),
        pay_date: NaiveDate::from_ymd_opt(2024, 8, 9).unwrap(),
        paychecks: vec![],
    };
    let _ = req.execute().run(&mut ()).expect("payday");
    println!("paychecks: {:#?}", req.paychecks);
}
