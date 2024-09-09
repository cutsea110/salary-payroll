use chrono::NaiveDate;
use core::fmt::Debug;
use dyn_clone::DynClone;
use std::{any::Any, cell::RefCell, ops::RangeInclusive, rc::Rc};

pub type EmployeeId = u32;
pub type MemberId = u32;

#[derive(Debug, Clone)]
pub struct Employee {
    emp_id: EmployeeId,
    name: String,
    address: String,
    classification: Rc<RefCell<dyn PaymentClassification>>,
    schedule: Rc<RefCell<dyn PaymentSchedule>>,
    method: Rc<RefCell<dyn PaymentMethod>>,
    affiliation: Rc<RefCell<dyn Affiliation>>,
}
impl Employee {
    pub fn new(
        emp_id: EmployeeId,
        name: &str,
        address: &str,
        classification: Rc<RefCell<dyn PaymentClassification>>,
        schedule: Rc<RefCell<dyn PaymentSchedule>>,
        method: Rc<RefCell<dyn PaymentMethod>>,
        affiliation: Rc<RefCell<dyn Affiliation>>,
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
        self.schedule.borrow().is_pay_date(date)
    }
    pub fn get_pay_period(&self, date: NaiveDate) -> RangeInclusive<NaiveDate> {
        self.schedule.borrow().get_pay_period(date)
    }
    pub fn payday(&self, pc: &mut PayCheck) {
        let gross_pay = self.classification.borrow().calculate_pay(&pc);
        let deductions = self.affiliation.borrow().calculate_deductions(&pc);
        let net_pay = gross_pay - deductions;
        pc.set_gross_pay(gross_pay);
        pc.set_deductions(deductions);
        pc.set_net_pay(net_pay);
        self.method.borrow().pay(pc);
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
    pub fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
        self.classification.clone()
    }
    pub fn set_classification(&mut self, classification: Rc<RefCell<dyn PaymentClassification>>) {
        self.classification = classification;
    }
    pub fn set_schedule(&mut self, schedule: Rc<RefCell<dyn PaymentSchedule>>) {
        self.schedule = schedule;
    }
    pub fn set_method(&mut self, method: Rc<RefCell<dyn PaymentMethod>>) {
        self.method = method;
    }
    pub fn get_affiliation(&self) -> Rc<RefCell<dyn Affiliation>> {
        self.affiliation.clone()
    }
    pub fn set_affiliation(&mut self, affiliation: Rc<RefCell<dyn Affiliation>>) {
        self.affiliation = affiliation;
    }
}

pub trait PaymentClassification: DynClone + Debug {
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn calculate_pay(&self, pc: &PayCheck) -> f32;
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
    fn calculate_deductions(&self, _pc: &PayCheck) -> f32 {
        0.0
    }
}
dyn_clone::clone_trait_object!(Affiliation);

#[derive(Debug, Clone, PartialEq)]
pub struct PayCheck {
    period: RangeInclusive<NaiveDate>,

    gross_pay: f32,
    deductions: f32,
    net_pay: f32,
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
    pub fn set_gross_pay(&mut self, gross_pay: f32) {
        self.gross_pay = gross_pay;
    }
    pub fn set_deductions(&mut self, deductions: f32) {
        self.deductions = deductions;
    }
    pub fn set_net_pay(&mut self, net_pay: f32) {
        self.net_pay = net_pay;
    }
}
