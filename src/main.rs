use chrono::NaiveDate;
use core::fmt::Debug;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use tx_rs::Tx;

mod domain {
    use chrono::NaiveDate;
    use core::fmt::Debug;
    use dyn_clone::DynClone;
    use std::any::Any;
    use std::ops::RangeInclusive;

    pub type EmployeeId = u32;
    pub type MemberId = u32;

    #[derive(Debug, Clone)]
    pub struct Employee {
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
        pub fn get_emp_id(&self) -> EmployeeId {
            self.emp_id
        }
        pub fn set_name(&mut self, name: &str) {
            self.name = name.to_string();
        }
        pub fn set_address(&mut self, address: &str) {
            self.address = address.to_string();
        }
        pub fn get_classification(&self) -> Box<dyn PaymentClassification> {
            self.classification.clone()
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

    pub trait PaymentClassification: DynClone + Debug {
        fn as_any_mut(&mut self) -> &mut dyn Any;
        fn calculate_pay(&self, pc: &PayCheck) -> f64;
    }
    dyn_clone::clone_trait_object!(PaymentClassification);

    pub trait PaymentSchedule: DynClone + Debug {
        fn is_pay_date(&self, date: NaiveDate) -> bool;
        fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate>;
    }
    dyn_clone::clone_trait_object!(PaymentSchedule);

    pub trait PaymentMethod: DynClone + Debug {
        // TODO: return type
        fn pay(&self, pc: &PayCheck);
    }
    dyn_clone::clone_trait_object!(PaymentMethod);

    pub trait Affiliation: DynClone + Debug {
        fn as_any(&self) -> &dyn Any;
        fn as_any_mut(&mut self) -> &mut dyn Any;
        fn calculate_deductions(&self, _pc: &PayCheck) -> f64 {
            0.0
        }
    }
    dyn_clone::clone_trait_object!(Affiliation);

    #[derive(Debug, Clone, PartialEq)]
    pub struct PayCheck {
        period: RangeInclusive<NaiveDate>,

        gross_pay: f64,
        deductions: f64,
        net_pay: f64,
    }
    impl PayCheck {
        pub fn new(period: RangeInclusive<NaiveDate>) -> Self {
            Self {
                period,
                gross_pay: 0.0,
                deductions: 0.0,
                net_pay: 0.0,
            }
        }
        pub fn get_pay_period(&self) -> RangeInclusive<NaiveDate> {
            self.period.clone()
        }
        pub fn set_gross_pay(&mut self, gross_pay: f64) {
            self.gross_pay = gross_pay;
        }
        pub fn set_deductions(&mut self, deductions: f64) {
            self.deductions = deductions;
        }
        pub fn set_net_pay(&mut self, net_pay: f64) {
            self.net_pay = net_pay;
        }
    }
}
use domain::*;

mod dao {
    use thiserror::Error;

    use crate::domain::{Employee, EmployeeId, MemberId};

    #[derive(Debug, Clone, Eq, PartialEq, Error)]
    pub enum EmployeeDaoError {
        #[error("insert error: {0}")]
        InsertError(String),
        #[error("delete error: {0}")]
        DeleteError(String),
        #[error("fetch error: {0}")]
        FetchError(String),
        #[error("update error: {0}")]
        UpdateError(String),
    }
    pub trait EmployeeDao<Ctx> {
        fn insert(
            &self,
            emp: Employee,
        ) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = EmployeeDaoError>;
        fn delete(
            &self,
            emp_id: EmployeeId,
        ) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeDaoError>;
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
    pub trait HaveEmployeeDao<Ctx> {
        fn dao(&self) -> Box<&impl EmployeeDao<Ctx>>;
    }
}
use dao::*;

mod classification {
    use chrono::NaiveDate;
    use std::any::Any;

    use crate::domain::{PayCheck, PaymentClassification};

    #[derive(Debug, Clone, PartialEq)]
    pub struct SalariedClassification {
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
    impl SalariedClassification {
        pub fn new(salary: f64) -> Self {
            Self { salary }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct HourlyClassification {
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
        pub fn new(hourly_rate: f64) -> Self {
            Self {
                hourly_rate,
                timecards: vec![],
            }
        }
        pub fn add_timecard(&mut self, tc: TimeCard) {
            self.timecards.push(tc);
        }
        pub fn calculate_pay_for_timecard(&self, tc: &TimeCard) -> f64 {
            let hours = tc.get_hours();
            let overtime = (hours - 8.0).max(0.0);
            let straight_time = hours - overtime;
            straight_time * self.hourly_rate + overtime * self.hourly_rate * 1.5
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct CommissionedClassification {
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
        pub fn new(salary: f64, commission_rate: f64) -> Self {
            Self {
                salary,
                commission_rate,
                sales_receipts: vec![],
            }
        }
        pub fn add_sales_receipt(&mut self, sr: SalesReceipt) {
            self.sales_receipts.push(sr);
        }
        pub fn calculate_pay_for_sales_receipt(&self, sr: &SalesReceipt) -> f64 {
            self.commission_rate * sr.get_amount()
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TimeCard {
        date: NaiveDate,
        hours: f64,
    }
    impl TimeCard {
        pub fn new(date: NaiveDate, hours: f64) -> Self {
            Self { date, hours }
        }
        pub fn get_date(&self) -> NaiveDate {
            self.date
        }
        pub fn get_hours(&self) -> f64 {
            self.hours
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SalesReceipt {
        date: NaiveDate,
        amount: f64,
    }
    impl SalesReceipt {
        pub fn new(date: NaiveDate, amount: f64) -> Self {
            Self { date, amount }
        }
        pub fn get_date(&self) -> NaiveDate {
            self.date
        }
        pub fn get_amount(&self) -> f64 {
            self.amount
        }
    }
}
use classification::*;

mod schedule {
    use chrono::{Datelike, Days, NaiveDate, Weekday};
    use std::ops::RangeInclusive;

    use crate::domain::PaymentSchedule;

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct MonthlySchedule;
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
        pub fn is_last_day_of_month(&self, date: NaiveDate) -> bool {
            date.month() != date.checked_add_days(Days::new(1)).unwrap().month()
        }
    }
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct WeeklySchedule;
    impl PaymentSchedule for WeeklySchedule {
        fn is_pay_date(&self, date: NaiveDate) -> bool {
            date.weekday() == Weekday::Fri
        }
        fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
            pay_date.checked_sub_days(Days::new(6)).unwrap()..=pay_date
        }
    }
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct BiweeklySchedule;
    impl PaymentSchedule for BiweeklySchedule {
        fn is_pay_date(&self, date: NaiveDate) -> bool {
            date.weekday() == Weekday::Fri && date.iso_week().week() % 2 == 0
        }
        fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
            pay_date.checked_sub_days(Days::new(13)).unwrap()..=pay_date
        }
    }
}
use schedule::*;

mod method {
    use crate::domain::{PayCheck, PaymentMethod};

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct HoldMethod;
    impl PaymentMethod for HoldMethod {
        fn pay(&self, pc: &PayCheck) {
            // concrete implementation
            println!("HoldMethod: {:?}", pc);
        }
    }
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct MailMethod {
        address: String,
    }
    impl PaymentMethod for MailMethod {
        fn pay(&self, pc: &PayCheck) {
            // concrete implementation
            println!("MailMethod for {}: {:?}", self.address, pc);
        }
    }
    impl MailMethod {
        pub fn new(address: String) -> Self {
            Self { address }
        }
    }
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct DirectMethod {
        bank: String,
        account: String,
    }
    impl PaymentMethod for DirectMethod {
        fn pay(&self, pc: &PayCheck) {
            // concrete implementation
            println!("DirectMethod to {}{}: {:?}", self.bank, self.account, pc);
        }
    }
    impl DirectMethod {
        pub fn new(bank: String, account: String) -> Self {
            Self { bank, account }
        }
    }
}
use method::*;

mod affiliation {
    use chrono::{Datelike, NaiveDate, Weekday};
    use std::any::Any;

    use crate::domain::{Affiliation, MemberId, PayCheck};

    #[derive(Debug, Clone, PartialEq)]
    pub struct UnionAffiliation {
        member_id: MemberId,
        dues: f64,

        service_charges: Vec<ServiceCharge>,
    }
    impl UnionAffiliation {
        pub fn new(member_id: MemberId, dues: f64) -> Self {
            Self {
                member_id,
                dues,
                service_charges: vec![],
            }
        }
        pub fn get_member_id(&self) -> MemberId {
            self.member_id
        }
        pub fn get_dues(&self) -> f64 {
            self.dues
        }
        pub fn add_service_charge(&mut self, sc: ServiceCharge) {
            self.service_charges.push(sc);
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
    pub struct NoAffiliation;
    impl Affiliation for NoAffiliation {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ServiceCharge {
        date: NaiveDate,
        amount: f64,
    }
    impl ServiceCharge {
        pub fn new(date: NaiveDate, amount: f64) -> Self {
            Self { date, amount }
        }
        pub fn get_date(&self) -> NaiveDate {
            self.date
        }
        pub fn get_amount(&self) -> f64 {
            self.amount
        }
    }
}
use affiliation::*;

mod tx_base {
    use thiserror::Error;
    use tx_rs::Tx;

    use crate::affiliation::NoAffiliation;
    use crate::dao::{EmployeeDao, EmployeeDaoError, HaveEmployeeDao};
    use crate::domain::{
        Affiliation, Employee, EmployeeId, PaymentClassification, PaymentMethod, PaymentSchedule,
    };
    use crate::method::HoldMethod;

    #[derive(Debug, Clone, Eq, PartialEq, Error)]
    pub enum EmployeeUsecaseError {
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

    pub trait AddEmployeeTransaction<Ctx>: HaveEmployeeDao<Ctx> {
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

    pub trait DeleteEmployeeTransaction<Ctx>: HaveEmployeeDao<Ctx> {
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

    pub trait ChangeEmployeeTransaction<Ctx>: HaveEmployeeDao<Ctx> {
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

    pub trait ChangeClassificationTransaction<Ctx>: ChangeEmployeeTransaction<Ctx> {
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

    pub trait ChangeMethodTransaction<Ctx>: ChangeEmployeeTransaction<Ctx> {
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

    pub trait ChangeAffiliationTransaction<Ctx>: ChangeEmployeeTransaction<Ctx> {
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
}
use tx_base::*;

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
        let classification = Box::new(SalariedClassification::new(self.get_salary()));
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
        let classification = Box::new(HourlyClassification::new(self.get_hourly_rate()));
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
        let classification = Box::new(CommissionedClassification::new(
            self.get_salary(),
            self.get_commission_rate(),
        ));
        let schedule = Box::new(BiweeklySchedule);

        self.exec(emp_id, name, address, classification, schedule)
    }
}

trait TimeCardTransaction<Ctx>: HaveEmployeeDao<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_date(&self) -> NaiveDate;
    fn get_hours(&self) -> f64;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError> {
        tx_rs::with_tx(move |ctx| {
            let emp = self
                .dao()
                .fetch(self.get_emp_id())
                .run(ctx)
                .map_err(EmployeeUsecaseError::NotFound)?;
            let mut binding = emp.get_classification();
            let hourly = binding
                .as_any_mut()
                .downcast_mut::<HourlyClassification>()
                .ok_or(EmployeeUsecaseError::NotHourlySalary(format!(
                    "emp_id: {}",
                    self.get_emp_id()
                )))?;
            hourly.add_timecard(TimeCard::new(self.get_date(), self.get_hours()));
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
            let emp = self
                .dao()
                .fetch(self.get_emp_id())
                .run(ctx)
                .map_err(EmployeeUsecaseError::NotFound)?;
            let mut binding = emp.get_classification();
            let commissioned = binding
                .as_any_mut()
                .downcast_mut::<CommissionedClassification>()
                .ok_or(EmployeeUsecaseError::NotCommissionedSalary(format!(
                    "emp_id: {}",
                    self.get_emp_id()
                )))?;
            commissioned.add_sales_receipt(SalesReceipt::new(self.get_date(), self.get_amount()));
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
            let emp = self
                .dao()
                .fetch(emp_id)
                .run(ctx)
                .map_err(EmployeeUsecaseError::NotFound)?;
            let mut binding = emp.get_affiliation();
            let affiliation = binding
                .as_any_mut()
                .downcast_mut::<UnionAffiliation>()
                .ok_or(EmployeeUsecaseError::NotUnionMember(format!(
                    "emp_id: {0}",
                    emp_id,
                )))?;
            affiliation.add_service_charge(ServiceCharge::new(self.get_date(), self.get_amount()));
            self.dao()
                .update(emp)
                .run(ctx)
                .map_err(EmployeeUsecaseError::UpdateEmployeeFailed)
        })
    }
}

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

trait ChangeSalariedTransaction<Ctx>: ChangeClassificationTransaction<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_salary(&self) -> f64;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        self.exec_classification(
            self.get_emp_id(),
            Box::new(SalariedClassification::new(self.get_salary())),
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
            Box::new(HourlyClassification::new(self.get_hourly_rate())),
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
            Box::new(CommissionedClassification::new(
                self.get_salary(),
                self.get_commission_rate(),
            )),
            Box::new(BiweeklySchedule),
        )
    }
}

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
            Box::new(DirectMethod::new(
                self.get_bank().to_string(),
                self.get_account().to_string(),
            )),
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
            Box::new(MailMethod::new(self.get_address().to_string())),
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
            Box::new(UnionAffiliation::new(self.get_member_id(), self.get_dues())),
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
                        |a| Ok(a.get_member_id()),
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
            let emp_id = emp.get_emp_id();
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
            let emp_id = emp.get_emp_id();
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

// Parser
pub mod parser {
    use super::*;
    use parsec_rs::{char, float32, int32, keyword, spaces, string, uint32, Parser};

    #[derive(Debug, Clone, PartialEq)]
    pub enum Tran {
        AddSalaryEmp {
            emp_id: EmployeeId,
            name: String,
            address: String,
            salary: f32,
        },
        AddHourlyEmp {
            emp_id: EmployeeId,
            name: String,
            address: String,
            hourly_rate: f32,
        },
        AddCommissionedEmp {
            emp_id: EmployeeId,
            name: String,
            address: String,
            salary: f32,
            commission_rate: f32,
        },
        DelEmp {
            emp_id: EmployeeId,
        },
        TimeCard {
            emp_id: EmployeeId,
            date: NaiveDate,
            hours: f32,
        },
        SalesReceipt {
            emp_id: EmployeeId,
            date: NaiveDate,
            amount: f32,
        },
        ServiceCharge {
            member_id: EmployeeId,
            date: NaiveDate,
            amount: f32,
        },
        ChgName {
            emp_id: EmployeeId,
            name: String,
        },
        ChgAddress {
            emp_id: EmployeeId,
            address: String,
        },
        ChgHourly {
            emp_id: EmployeeId,
            hourly_rate: f32,
        },
        ChgSalaried {
            emp_id: EmployeeId,
            salary: f32,
        },
        ChgCommissioned {
            emp_id: EmployeeId,
            salary: f32,
            commission_rate: f32,
        },
        ChgHold {
            emp_id: EmployeeId,
        },
        ChgDirect {
            emp_id: EmployeeId,
            bank: String,
            account: String,
        },
        ChgMail {
            emp_id: EmployeeId,
            address: String,
        },
        ChgMember {
            emp_id: EmployeeId,
            member_id: EmployeeId,
            dues: f32,
        },
        ChgNoMember {
            emp_id: EmployeeId,
        },
        Payday {
            date: NaiveDate,
        },
    }
    pub fn transaction() -> impl Parser<Item = Tran> {
        add_salary_emp()
            .or(add_hourly_emp())
            .or(add_commissioned_emp())
            .or(del_emp())
            .or(time_card())
            .or(sales_receipt())
            .or(service_charge())
            .or(chg_name())
            .or(chg_address())
            .or(chg_hourly())
            .or(chg_salaried())
            .or(chg_commissioned())
            .or(chg_hold())
            .or(chg_direct())
            .or(chg_mail())
            .or(chg_member())
            .or(chg_no_member())
            .or(payday())
    }
    #[cfg(test)]
    mod test_transaction {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test_add_salary_emp() {
            let input = r#"AddEmp 42 "Bob" "Home" S 1000.0"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::AddSalaryEmp {
                        emp_id: 42,
                        name: "Bob".to_string(),
                        address: "Home".to_string(),
                        salary: 1000.0
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_add_hourly_emp() {
            let input = r#"AddEmp 42 "Bob" "Home" H 1000.0"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::AddHourlyEmp {
                        emp_id: 42,
                        name: "Bob".to_string(),
                        address: "Home".to_string(),
                        hourly_rate: 1000.0
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_add_commissioned_emp() {
            let input = r#"AddEmp 42 "Bob" "Home" C 1000.0 0.1"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::AddCommissionedEmp {
                        emp_id: 42,
                        name: "Bob".to_string(),
                        address: "Home".to_string(),
                        salary: 1000.0,
                        commission_rate: 0.1
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_del_emp() {
            let input = r#"DelEmp 42"#;
            let result = transaction().parse(input);
            assert_eq!(result, Ok((Tran::DelEmp { emp_id: 42 }, "")));
        }
        #[test]
        fn test_time_card() {
            let input = r#"TimeCard 42 2021-01-01 8.0"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::TimeCard {
                        emp_id: 42,
                        date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                        hours: 8.0
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_sales_receipt() {
            let input = r#"SalesReceipt 42 2021-01-01 1000.0"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::SalesReceipt {
                        emp_id: 42,
                        date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                        amount: 1000.0
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_service_charge() {
            let input = r#"ServiceCharge 42 2021-01-01 1000.0"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::ServiceCharge {
                        member_id: 42,
                        date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                        amount: 1000.0
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_chg_name() {
            let input = r#"ChgEmp 42 Name "Bob""#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::ChgName {
                        emp_id: 42,
                        name: "Bob".to_string()
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_chg_address() {
            let input = r#"ChgEmp 42 Address "123 Wall St.""#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::ChgAddress {
                        emp_id: 42,
                        address: "123 Wall St.".to_string()
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_chg_hourly() {
            let input = r#"ChgEmp 42 Hourly 1000.0"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::ChgHourly {
                        emp_id: 42,
                        hourly_rate: 1000.0
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_chg_salaried() {
            let input = r#"ChgEmp 42 Salaried 1000.0"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::ChgSalaried {
                        emp_id: 42,
                        salary: 1000.0
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_chg_commissioned() {
            let input = r#"ChgEmp 42 Commissioned 1000.0 0.1"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::ChgCommissioned {
                        emp_id: 42,
                        salary: 1000.0,
                        commission_rate: 0.1
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_chg_hold() {
            let input = r#"ChgEmp 42 Hold"#;
            let result = transaction().parse(input);
            assert_eq!(result, Ok((Tran::ChgHold { emp_id: 42 }, "")));
        }
        #[test]
        fn test_chg_direct() {
            let input = r#"ChgEmp 42 Direct "mufg" "1234567""#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::ChgDirect {
                        emp_id: 42,
                        bank: "mufg".to_string(),
                        account: "1234567".to_string()
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_chg_mail() {
            let input = r#"ChgEmp 42 Mail "bob@gmail.com""#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::ChgMail {
                        emp_id: 42,
                        address: "bob@gmail.com".to_string()
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_chg_member() {
            let input = r#"ChgEmp 42 Member 7234 Dues 9.45"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::ChgMember {
                        emp_id: 42,
                        member_id: 7234,
                        dues: 9.45,
                    },
                    "",
                ))
            );
        }
        #[test]
        fn test_no_member() {
            let input = r#"ChgEmp 42 NoMember"#;
            let result = transaction().parse(input);
            assert_eq!(result, Ok((Tran::ChgNoMember { emp_id: 42 }, "")));
        }
    }

    fn add_salary_emp() -> impl Parser<Item = Tran> {
        let prefix = keyword("AddEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let name = string().with(spaces());
        let address = string().with(spaces());
        let monthly_rate = char('S').skip(spaces()).skip(float32()).with(spaces());

        prefix
            .skip(emp_id)
            .join(name)
            .join(address)
            .join(monthly_rate)
            .map(|(((emp_id, name), address), salary)| Tran::AddSalaryEmp {
                emp_id,
                name,
                address,
                salary,
            })
    }
    #[cfg(test)]
    mod test_add_salary_emp {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"AddEmp 1 "Bob" "Home" S 1000.0"#;
            let result = add_salary_emp().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::AddSalaryEmp {
                        emp_id: 1,
                        name: "Bob".to_string(),
                        address: "Home".to_string(),
                        salary: 1000.0
                    },
                    ""
                ))
            );
        }
    }

    fn add_hourly_emp() -> impl Parser<Item = Tran> {
        let prefix = keyword("AddEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let name = string().with(spaces());
        let address = string().with(spaces());
        let hourly_rate = char('H').skip(spaces()).skip(float32()).with(spaces());

        prefix
            .skip(emp_id)
            .join(name)
            .join(address)
            .join(hourly_rate)
            .map(
                |(((emp_id, name), address), hourly_rate)| Tran::AddHourlyEmp {
                    emp_id,
                    name,
                    address,
                    hourly_rate,
                },
            )
    }
    #[cfg(test)]
    mod test_add_hourly_emp {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"AddEmp 1 "Bob" "Home" H 1000.0"#;
            let result = add_hourly_emp().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::AddHourlyEmp {
                        emp_id: 1,
                        name: "Bob".to_string(),
                        address: "Home".to_string(),
                        hourly_rate: 1000.0
                    },
                    ""
                ))
            );
        }
    }

    fn add_commissioned_emp() -> impl Parser<Item = Tran> {
        let prefix = keyword("AddEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let name = string().with(spaces());
        let address = string().with(spaces());
        let salary = char('C').skip(spaces()).skip(float32()).with(spaces());
        let commission_rate = float32().with(spaces());

        prefix
            .skip(emp_id)
            .join(name)
            .join(address)
            .join(salary)
            .join(commission_rate)
            .map(|((((emp_id, name), address), salary), commission_rate)| {
                Tran::AddCommissionedEmp {
                    emp_id,
                    name,
                    address,
                    salary,
                    commission_rate,
                }
            })
    }
    #[cfg(test)]
    mod test_add_commissioned_emp {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"AddEmp 1 "Bob" "Home" C 1000.0 0.1"#;
            let result = add_commissioned_emp().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::AddCommissionedEmp {
                        emp_id: 1,
                        name: "Bob".to_string(),
                        address: "Home".to_string(),
                        salary: 1000.0,
                        commission_rate: 0.1
                    },
                    ""
                ))
            );
        }
    }

    fn del_emp() -> impl Parser<Item = Tran> {
        let prefix = keyword("DelEmp").skip(spaces());
        let emp_id = uint32().with(spaces());

        prefix.skip(emp_id).map(|emp_id| Tran::DelEmp { emp_id })
    }
    #[cfg(test)]
    mod test_del_emp {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"DelEmp 1"#;
            let result = del_emp().parse(input);
            assert_eq!(result, Ok((Tran::DelEmp { emp_id: 1 }, "")));
        }
    }

    fn date() -> impl Parser<Item = NaiveDate> {
        let year = int32().with(char('-'));
        let month = uint32().with(char('-'));
        let day = uint32();

        year.join(month)
            .join(day)
            .map(|((y, m), d)| NaiveDate::from_ymd_opt(y as i32, m as u32, d as u32).expect("date"))
    }
    #[cfg(test)]
    mod test_date {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = "2021-01-01";
            let result = date().parse(input);
            assert_eq!(
                result,
                Ok((NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(), ""))
            );
        }
    }

    fn time_card() -> impl Parser<Item = Tran> {
        let prefix = keyword("TimeCard").skip(spaces());
        let emp_id = uint32().with(spaces());
        let date = date().with(spaces());
        let hours = float32().with(spaces());

        prefix
            .skip(emp_id)
            .join(date)
            .join(hours)
            .map(|((emp_id, date), hours)| Tran::TimeCard {
                emp_id,
                date,
                hours,
            })
    }
    #[cfg(test)]
    mod test_time_card {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"TimeCard 1 2021-01-01 8.0"#;
            let result = time_card().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::TimeCard {
                        emp_id: 1,
                        date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                        hours: 8.0
                    },
                    ""
                ))
            );
        }
    }

    fn sales_receipt() -> impl Parser<Item = Tran> {
        let prefix = keyword("SalesReceipt").skip(spaces());
        let emp_id = uint32().with(spaces());
        let date = date().with(spaces());
        let amount = float32().with(spaces());

        prefix
            .skip(emp_id)
            .join(date)
            .join(amount)
            .map(|((emp_id, date), amount)| Tran::SalesReceipt {
                emp_id,
                date,
                amount,
            })
    }
    #[cfg(test)]
    mod test_sales_receipt {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"SalesReceipt 1 2021-01-01 1000.0"#;
            let result = sales_receipt().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::SalesReceipt {
                        emp_id: 1,
                        date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                        amount: 1000.0
                    },
                    ""
                ))
            );
        }
    }

    fn service_charge() -> impl Parser<Item = Tran> {
        let prefix = keyword("ServiceCharge").skip(spaces());
        let member_id = uint32().with(spaces());
        let date = date().with(spaces());
        let amount = float32().with(spaces());

        prefix
            .skip(member_id)
            .join(date)
            .join(amount)
            .map(|((member_id, date), amount)| Tran::ServiceCharge {
                member_id,
                date,
                amount,
            })
    }
    #[cfg(test)]
    mod test_service_charge {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"ServiceCharge 1 2021-01-01 1000.0"#;
            let result = service_charge().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::ServiceCharge {
                        member_id: 1,
                        date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                        amount: 1000.0
                    },
                    ""
                ))
            );
        }
    }

    fn chg_name() -> impl Parser<Item = Tran> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let name = keyword("Name").skip(spaces()).skip(string()).with(spaces());

        prefix
            .skip(emp_id)
            .join(name)
            .map(|(emp_id, name)| Tran::ChgName { emp_id, name })
    }
    #[cfg(test)]
    mod test_chg_name {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"ChgEmp 1 Name "Bob""#;
            let result = chg_name().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::ChgName {
                        emp_id: 1,
                        name: "Bob".to_string()
                    },
                    ""
                ))
            );
        }
    }

    fn chg_address() -> impl Parser<Item = Tran> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let address = keyword("Address")
            .skip(spaces())
            .skip(string())
            .with(spaces());

        prefix
            .skip(emp_id)
            .join(address)
            .map(|(emp_id, address)| Tran::ChgAddress { emp_id, address })
    }
    #[cfg(test)]
    mod test_chg_address {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"ChgEmp 1 Address "123 Main St""#;
            let result = chg_address().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::ChgAddress {
                        emp_id: 1,
                        address: "123 Main St".to_string()
                    },
                    ""
                ))
            );
        }
    }

    fn chg_hourly() -> impl Parser<Item = Tran> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let hourly_rate = keyword("Hourly")
            .skip(spaces())
            .skip(float32())
            .with(spaces());

        prefix
            .skip(emp_id)
            .join(hourly_rate)
            .map(|(emp_id, hourly_rate)| Tran::ChgHourly {
                emp_id,
                hourly_rate,
            })
    }
    #[cfg(test)]
    mod test_chg_hourly {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"ChgEmp 1 Hourly 13.78"#;
            let result = chg_hourly().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::ChgHourly {
                        emp_id: 1,
                        hourly_rate: 13.78
                    },
                    ""
                ))
            );
        }
    }

    fn chg_salaried() -> impl Parser<Item = Tran> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let salaried = keyword("Salaried")
            .skip(spaces())
            .skip(float32())
            .with(spaces());

        prefix
            .skip(emp_id)
            .join(salaried)
            .map(|(emp_id, salary)| Tran::ChgSalaried { emp_id, salary })
    }
    #[cfg(test)]
    mod test_chg_salaried {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"ChgEmp 1 Salaried 1023.456"#;
            let result = chg_salaried().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::ChgSalaried {
                        emp_id: 1,
                        salary: 1023.456
                    },
                    ""
                ))
            );
        }
    }

    fn chg_commissioned() -> impl Parser<Item = Tran> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let salary = keyword("Commissioned")
            .skip(spaces())
            .skip(float32())
            .with(spaces());
        let commission_rate = float32().with(spaces());

        prefix.skip(emp_id).join(salary).join(commission_rate).map(
            |((emp_id, salary), commission_rate)| Tran::ChgCommissioned {
                emp_id,
                salary,
                commission_rate,
            },
        )
    }
    #[cfg(test)]
    mod test_chg_commissioned {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"ChgEmp 1 Commissioned 1018.91 0.19"#;
            let result = chg_commissioned().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::ChgCommissioned {
                        emp_id: 1,
                        salary: 1018.91,
                        commission_rate: 0.19
                    },
                    ""
                ))
            );
        }
    }

    fn chg_hold() -> impl Parser<Item = Tran> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let hold = keyword("Hold").skip(spaces());

        prefix
            .skip(emp_id)
            .with(hold)
            .map(|emp_id| Tran::ChgHold { emp_id })
    }
    #[cfg(test)]
    mod test_chg_hold {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"ChgEmp 1 Hold"#;
            let result = chg_hold().parse(input);
            assert_eq!(result, Ok((Tran::ChgHold { emp_id: 1 }, "")));
        }
    }

    fn chg_direct() -> impl Parser<Item = Tran> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let bank = keyword("Direct")
            .skip(spaces())
            .skip(string())
            .with(spaces());
        let account = string().with(spaces());

        prefix
            .skip(emp_id)
            .join(bank)
            .join(account)
            .map(|((emp_id, bank), account)| Tran::ChgDirect {
                emp_id,
                bank,
                account,
            })
    }
    #[cfg(test)]
    mod test_chg_direct {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"ChgEmp 1 Direct "Bank" "Account""#;
            let result = chg_direct().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::ChgDirect {
                        emp_id: 1,
                        bank: "Bank".to_string(),
                        account: "Account".to_string()
                    },
                    ""
                ))
            );
        }
    }

    fn chg_mail() -> impl Parser<Item = Tran> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let address = keyword("Mail").skip(spaces()).skip(string()).with(spaces());

        prefix
            .skip(emp_id)
            .join(address)
            .map(|(emp_id, address)| Tran::ChgMail { emp_id, address })
    }
    #[cfg(test)]
    mod test_chg_mail {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"ChgEmp 1 Mail "bob@gmail.com""#;
            let result = chg_mail().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::ChgMail {
                        emp_id: 1,
                        address: "bob@gmail.com".to_string()
                    },
                    ""
                ))
            );
        }
    }

    fn chg_member() -> impl Parser<Item = Tran> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let member_id = keyword("Member")
            .skip(spaces())
            .skip(uint32())
            .with(spaces());
        let dues = keyword("Dues")
            .skip(spaces())
            .skip(float32())
            .with(spaces());

        prefix
            .skip(emp_id)
            .join(member_id)
            .join(dues)
            .map(|((emp_id, member_id), dues)| Tran::ChgMember {
                emp_id,
                member_id,
                dues,
            })
    }
    #[cfg(test)]
    mod test_chg_member {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"ChgEmp 1 Member 2 Dues 100.0"#;
            let result = chg_member().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::ChgMember {
                        emp_id: 1,
                        member_id: 2,
                        dues: 100.0
                    },
                    ""
                ))
            );
        }
    }

    fn chg_no_member() -> impl Parser<Item = Tran> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let no_member = keyword("NoMember").skip(spaces());

        prefix
            .skip(emp_id)
            .with(no_member)
            .map(|emp_id| Tran::ChgNoMember { emp_id })
    }
    #[cfg(test)]
    mod test_chg_no_member {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"ChgEmp 1 NoMember"#;
            let result = chg_no_member().parse(input);
            assert_eq!(result, Ok((Tran::ChgNoMember { emp_id: 1 }, "")));
        }
    }

    fn payday() -> impl Parser<Item = Tran> {
        let prefix = keyword("Payday").skip(spaces());
        let date = date().with(spaces());

        prefix.skip(date).map(|date| Tran::Payday { date })
    }
    #[cfg(test)]
    mod test_payday {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"Payday 2021-01-01"#;
            let result = payday().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tran::Payday {
                        date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap()
                    },
                    ""
                ))
            );
        }
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
