use std::{cell::RefCell, rc::Rc};

use abstract_tx::{ChangeMethodTransaction, UsecaseError};
use payroll_domain::EmployeeId;
use payroll_impl::method::DirectMethod;

pub trait ChangeDirectTransaction<Ctx>: ChangeMethodTransaction<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        bank: &str,
        account: &str,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeMethodTransaction::execute(
            self,
            emp_id,
            Rc::new(RefCell::new(DirectMethod::new(bank, account))),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeDirectTransaction<Ctx> for T where T: ChangeMethodTransaction<Ctx> {}
