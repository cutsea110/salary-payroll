use abstract_tx::{ChangeEmployeeTransaction, UsecaseError};
use payroll_domain::EmployeeId;

pub trait AddressChangeableEmployee {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_address(&self) -> &str;
}
pub trait ChangeAddressTransaction<Ctx>:
    ChangeEmployeeTransaction<Ctx> + AddressChangeableEmployee
{
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeeTransaction::<Ctx>::execute(self, self.get_emp_id(), |_ctx, emp| {
            emp.set_address(self.get_address());
            Ok(())
        })
    }
}
// blanket implementation
impl<T, Ctx> ChangeAddressTransaction<Ctx> for T where
    T: ChangeEmployeeTransaction<Ctx> + AddressChangeableEmployee
{
}
