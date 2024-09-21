use std::{cell::RefCell, rc::Rc};
use tx_rs::Tx;

use abstract_tx::{ChangeAffiliationTransaction, UsecaseError};
use dao::EmployeeDao;
use payroll_domain::EmployeeId;
use payroll_impl::affiliation::{NoAffiliation, UnionAffiliation};

pub trait ChangeUnaffiliatedTransaction<Ctx>: ChangeAffiliationTransaction<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeAffiliationTransaction::execute(
            self,
            emp_id,
            move |ctx, emp| {
                let member_id = emp
                    .get_affiliation()
                    .borrow()
                    .as_any()
                    .downcast_ref::<UnionAffiliation>()
                    .map_or(
                        Err(UsecaseError::NotUnionMember(format!("emp_id: {}", emp_id))),
                        |a| Ok(a.get_member_id()),
                    )?;
                self.dao()
                    .remove_union_member(member_id)
                    .run(ctx)
                    .map_err(UsecaseError::RemoveUnionMemberFailed)
            },
            Rc::new(RefCell::new(NoAffiliation)),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeUnaffiliatedTransaction<Ctx> for T where T: ChangeAffiliationTransaction<Ctx> {}
