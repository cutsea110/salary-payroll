use abstract_tx::{ChangeEmployeeTransaction, UsecaseError};
use payroll_domain::EmployeeId;

pub trait NameChangeableEmployee {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_name(&self) -> &str;
}
pub trait ChangeNameTransaction<Ctx>:
    ChangeEmployeeTransaction<Ctx> + NameChangeableEmployee
{
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeeTransaction::<Ctx>::execute(self, self.get_emp_id(), |_ctx, emp| {
            emp.set_name(self.get_name());
            Ok(())
        })
    }
}
// blanket implementation
impl<T, Ctx> ChangeNameTransaction<Ctx> for T where
    T: ChangeEmployeeTransaction<Ctx> + NameChangeableEmployee
{
}
