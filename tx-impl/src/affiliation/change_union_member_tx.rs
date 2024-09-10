use std::{cell::RefCell, rc::Rc};
use tx_rs::Tx;

use abstract_tx::{ChangeAffiliationTransaction, UsecaseError};
use dao::EmployeeDao;
use payroll_domain::{EmployeeId, MemberId};
use payroll_impl::affiliation::UnionAffiliation;

pub trait UnionChangeableEmployee {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_member_id(&self) -> MemberId;
    fn get_dues(&self) -> f32;
}
pub trait ChangeUnionMemberTransaction<Ctx>:
    ChangeAffiliationTransaction<Ctx> + UnionChangeableEmployee
{
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
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
                    .map_err(UsecaseError::AddUnionMemberFailed)
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
