use std::{cell::RefCell, rc::Rc};

use abstract_tx::{ChangeMethodTransaction, UsecaseError};
use payroll_domain::EmployeeId;
use payroll_impl::method::MailMethod;

pub trait ChangeMailTransaction<Ctx>: ChangeMethodTransaction<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        address: &str,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeMethodTransaction::execute(
            self,
            emp_id,
            Rc::new(RefCell::new(MailMethod::new(address))),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeMailTransaction<Ctx> for T where T: ChangeMethodTransaction<Ctx> {}
