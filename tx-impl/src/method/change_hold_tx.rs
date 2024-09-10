use std::{cell::RefCell, rc::Rc};

use abstract_tx::{ChangeMethodTransaction, UsecaseError};
use payroll_domain::EmployeeId;
use payroll_impl::method::HoldMethod;

pub trait HoldChangeableEmployee {
    fn get_emp_id(&self) -> EmployeeId;
}
pub trait ChangeHoldTransaction<Ctx>:
    ChangeMethodTransaction<Ctx> + HoldChangeableEmployee
{
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeMethodTransaction::<Ctx>::execute(
            self,
            self.get_emp_id(),
            Rc::new(RefCell::new(HoldMethod)),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeHoldTransaction<Ctx> for T where
    T: ChangeMethodTransaction<Ctx> + HoldChangeableEmployee
{
}
