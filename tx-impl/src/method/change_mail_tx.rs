use std::{cell::RefCell, rc::Rc};

use abstract_tx::{ChangeMethodTransaction, UsecaseError};
use payroll_domain::EmployeeId;
use payroll_impl::method::MailMethod;

pub trait MailChangeableEmployee {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_address(&self) -> &str;
}
pub trait ChangeMailTransaction<Ctx>:
    ChangeMethodTransaction<Ctx> + MailChangeableEmployee
{
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeMethodTransaction::<Ctx>::execute(
            self,
            self.get_emp_id(),
            Rc::new(RefCell::new(MailMethod::new(
                self.get_address().to_string(),
            ))),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeMailTransaction<Ctx> for T where
    T: ChangeMethodTransaction<Ctx> + MailChangeableEmployee
{
}
