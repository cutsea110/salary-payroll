use std::{cell::RefCell, rc::Rc};

use abstract_tx::{ChangeMethodTransaction, UsecaseError};
use payroll_domain::EmployeeId;
use payroll_impl::method::HoldMethod;

pub trait ChangeHoldTransaction<Ctx>: ChangeMethodTransaction<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeMethodTransaction::<Ctx>::execute(self, emp_id, Rc::new(RefCell::new(HoldMethod)))
    }
}
// blanket implementation
impl<T, Ctx> ChangeHoldTransaction<Ctx> for T where T: ChangeMethodTransaction<Ctx> {}
