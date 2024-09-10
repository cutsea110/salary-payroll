use std::{cell::RefCell, rc::Rc};

use crate::change_employee_tx::ChangeEmployeeTransaction;
use crate::error::UsecaseError;
use payroll_domain::{EmployeeId, PaymentClassification, PaymentSchedule};

pub trait ChangeClassificationTransaction<Ctx>: ChangeEmployeeTransaction<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        classification: Rc<RefCell<dyn PaymentClassification>>,
        schedule: Rc<RefCell<dyn PaymentSchedule>>,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeeTransaction::<Ctx>::execute(self, emp_id, |_ctx, emp| {
            emp.set_classification(classification);
            emp.set_schedule(schedule);
            Ok(())
        })
    }
}
// blanket implementation
impl<Ctx, T> ChangeClassificationTransaction<Ctx> for T where T: ChangeEmployeeTransaction<Ctx> {}
