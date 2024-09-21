use std::{cell::RefCell, rc::Rc};
use tx_rs::Tx;

use abstract_tx::{ChangeAffiliationTransaction, UsecaseError};
use dao::EmployeeDao;
use payroll_domain::{EmployeeId, MemberId};
use payroll_impl::affiliation::UnionAffiliation;

pub trait ChangeUnionMemberTransaction<Ctx>: ChangeAffiliationTransaction<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        member_id: MemberId,
        dues: f32,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeAffiliationTransaction::execute(
            self,
            emp_id,
            move |ctx, _emp| {
                self.dao()
                    .add_union_member(member_id, emp_id)
                    .run(ctx)
                    .map_err(UsecaseError::AddUnionMemberFailed)
            },
            Rc::new(RefCell::new(UnionAffiliation::new(member_id, dues))),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeUnionMemberTransaction<Ctx> for T where T: ChangeAffiliationTransaction<Ctx> {}
