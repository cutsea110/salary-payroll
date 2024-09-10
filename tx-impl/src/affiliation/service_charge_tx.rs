use chrono::NaiveDate;
use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::MemberId;
use payroll_impl::affiliation::{ServiceCharge, UnionAffiliation};

pub trait ServiceChargeableMember {
    fn get_member_id(&self) -> MemberId;
    fn get_date(&self) -> NaiveDate;
    fn get_amount(&self) -> f32;
}
pub trait ServiceChargeTransaction<Ctx>: HaveEmployeeDao<Ctx> + ServiceChargeableMember {
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError> {
        tx_rs::with_tx(move |ctx| {
            let emp_id = self
                .dao()
                .find_union_member(self.get_member_id())
                .run(ctx)
                .map_err(UsecaseError::NotFound)?;
            let emp = self
                .dao()
                .fetch(emp_id)
                .run(ctx)
                .map_err(UsecaseError::NotFound)?;
            emp.get_affiliation()
                .borrow_mut()
                .as_any_mut()
                .downcast_mut::<UnionAffiliation>()
                .ok_or(UsecaseError::NotUnionMember(
                    format!("emp_id: {0}", emp_id,),
                ))?
                .add_service_charge(ServiceCharge::new(self.get_date(), self.get_amount()));
            self.dao()
                .update(emp)
                .run(ctx)
                .map_err(UsecaseError::UpdateEmployeeFailed)
        })
    }
}
// blanket implementation
impl<T, Ctx> ServiceChargeTransaction<Ctx> for T where
    T: HaveEmployeeDao<Ctx> + ServiceChargeableMember
{
}
