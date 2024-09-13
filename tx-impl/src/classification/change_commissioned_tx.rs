use std::{cell::RefCell, rc::Rc};

use abstract_tx::{ChangeClassificationTransaction, UsecaseError};
use payroll_domain::EmployeeId;
use payroll_impl::{classification::CommissionedClassification, schedule::BiweeklySchedule};

pub trait ChangeCommissionedTransaction<Ctx>: ChangeClassificationTransaction<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        salary: f32,
        commission_rate: f32,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeClassificationTransaction::<Ctx>::execute(
            self,
            emp_id,
            Rc::new(RefCell::new(CommissionedClassification::new(
                salary,
                commission_rate,
            ))),
            Rc::new(RefCell::new(BiweeklySchedule)),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeCommissionedTransaction<Ctx> for T where T: ChangeClassificationTransaction<Ctx> {}
