use std::{cell::RefCell, rc::Rc};

use crate::change_employee_tx::ChangeEmployeeTransaction;
use crate::error::UsecaseError;
use payroll_domain::{EmployeeId, PaymentMethod};

pub trait ChangeMethodTransaction<Ctx>: ChangeEmployeeTransaction<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        method: Rc<RefCell<dyn PaymentMethod>>,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeeTransaction::<Ctx>::execute(self, emp_id, |_ctx, emp| {
            emp.set_method(method);
            Ok(())
        })
    }
}
// blanket implementation
impl<Ctx, T> ChangeMethodTransaction<Ctx> for T where T: ChangeEmployeeTransaction<Ctx> {}
