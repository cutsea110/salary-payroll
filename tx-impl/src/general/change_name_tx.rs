use abstract_tx::{ChangeEmployeeTransaction, UsecaseError};
use payroll_domain::EmployeeId;

pub trait ChangeNameTransaction<Ctx>: ChangeEmployeeTransaction<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        name: &str,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeeTransaction::<Ctx>::execute(self, emp_id, |_ctx, emp| {
            emp.set_name(name);
            Ok(())
        })
    }
}
// blanket implementation
impl<T, Ctx> ChangeNameTransaction<Ctx> for T where T: ChangeEmployeeTransaction<Ctx> {}