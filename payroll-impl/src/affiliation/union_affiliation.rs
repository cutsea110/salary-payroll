use chrono::{Datelike, Weekday};
use std::any::Any;

use crate::affiliation::ServiceCharge;
use payroll_domain::{Affiliation, MemberId, Paycheck};

#[derive(Debug, Clone, PartialEq)]
pub struct UnionAffiliation {
    member_id: MemberId,
    dues: f32,

    service_charges: Vec<ServiceCharge>,
}
impl UnionAffiliation {
    pub fn new(member_id: MemberId, dues: f32) -> Self {
        Self {
            member_id,
            dues,
            service_charges: vec![],
        }
    }
    pub fn get_member_id(&self) -> MemberId {
        self.member_id
    }
    pub fn get_dues(&self) -> f32 {
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
    fn calculate_deductions(&self, pc: &Paycheck) -> f32 {
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
