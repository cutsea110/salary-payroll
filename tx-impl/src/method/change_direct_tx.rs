use std::{cell::RefCell, rc::Rc};

use abstract_tx::{ChangeMethodTransaction, UsecaseError};
use payroll_domain::EmployeeId;
use payroll_impl::method::DirectMethod;

pub trait DirectChangeableEmployee {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_bank(&self) -> &str;
    fn get_account(&self) -> &str;
}
pub trait ChangeDirectTransaction<Ctx>:
    ChangeMethodTransaction<Ctx> + DirectChangeableEmployee
{
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeMethodTransaction::<Ctx>::execute(
            self,
            self.get_emp_id(),
            Rc::new(RefCell::new(DirectMethod::new(
                self.get_bank().to_string(),
                self.get_account().to_string(),
            ))),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeDirectTransaction<Ctx> for T where
    T: ChangeMethodTransaction<Ctx> + DirectChangeableEmployee
{
}
