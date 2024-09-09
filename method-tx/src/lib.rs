use std::{cell::RefCell, rc::Rc};

use abstract_tx::{ChangeMethodTransaction, EmployeeUsecaseError};
use payroll_domain::EmployeeId;
use payroll_impl::method::{DirectMethod, HoldMethod, MailMethod};

pub trait DirectChangeableEmployee {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_bank(&self) -> &str;
    fn get_account(&self) -> &str;
}
pub trait ChangeDirectTransaction<Ctx>:
    ChangeMethodTransaction<Ctx> + DirectChangeableEmployee
{
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
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

pub trait MailChangeableEmployee {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_address(&self) -> &str;
}
pub trait ChangeMailTransaction<Ctx>:
    ChangeMethodTransaction<Ctx> + MailChangeableEmployee
{
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
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

pub trait HoldChangeableEmployee {
    fn get_emp_id(&self) -> EmployeeId;
}
pub trait ChangeHoldTransaction<Ctx>:
    ChangeMethodTransaction<Ctx> + HoldChangeableEmployee
{
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
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
