use chrono::NaiveDate;
use std::{cell::RefCell, rc::Rc};
use tx_rs::Tx;

use abstract_tx::{ChangeAffiliationTransaction, EmployeeUsecaseError};
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{EmployeeId, MemberId};
use payroll_impl::affiliation::{NoAffiliation, ServiceCharge, UnionAffiliation};

pub trait ServiceChargeableMember {
    fn get_member_id(&self) -> MemberId;
    fn get_date(&self) -> NaiveDate;
    fn get_amount(&self) -> f32;
}
pub trait ServiceChargeTransaction<Ctx>: HaveEmployeeDao<Ctx> + ServiceChargeableMember {
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
            emp.get_affiliation()
                .borrow_mut()
                .as_any_mut()
                .downcast_mut::<UnionAffiliation>()
                .ok_or(EmployeeUsecaseError::NotUnionMember(format!(
                    "emp_id: {0}",
                    emp_id,
                )))?
                .add_service_charge(ServiceCharge::new(self.get_date(), self.get_amount()));
            self.dao()
                .update(emp)
                .run(ctx)
                .map_err(EmployeeUsecaseError::UpdateEmployeeFailed)
        })
    }
}
// blanket implementation
impl<T, Ctx> ServiceChargeTransaction<Ctx> for T where
    T: HaveEmployeeDao<Ctx> + ServiceChargeableMember
{
}

pub trait UnionChangeableEmployee {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_member_id(&self) -> MemberId;
    fn get_dues(&self) -> f32;
}
pub trait ChangeUnionMemberTransaction<Ctx>:
    ChangeAffiliationTransaction<Ctx> + UnionChangeableEmployee
{
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        ChangeAffiliationTransaction::<Ctx>::execute(
            self,
            self.get_emp_id(),
            |ctx, _emp| {
                self.dao()
                    .add_union_member(self.get_member_id(), self.get_emp_id())
                    .run(ctx)
                    .map_err(EmployeeUsecaseError::AddUnionMemberFailed)
            },
            Rc::new(RefCell::new(UnionAffiliation::new(
                self.get_member_id(),
                self.get_dues(),
            ))),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeUnionMemberTransaction<Ctx> for T where
    T: ChangeAffiliationTransaction<Ctx> + UnionChangeableEmployee
{
}

pub trait NoAffiliationChangeableEmployee {
    fn get_emp_id(&self) -> EmployeeId;
}
pub trait ChangeUnaffiliatedTransaction<Ctx>:
    ChangeAffiliationTransaction<Ctx> + NoAffiliationChangeableEmployee
{
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        ChangeAffiliationTransaction::<Ctx>::execute(
            self,
            self.get_emp_id(),
            |ctx, emp| {
                let member_id = emp
                    .get_affiliation()
                    .borrow()
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
            Rc::new(RefCell::new(NoAffiliation)),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeUnaffiliatedTransaction<Ctx> for T where
    T: ChangeAffiliationTransaction<Ctx> + NoAffiliationChangeableEmployee
{
}
