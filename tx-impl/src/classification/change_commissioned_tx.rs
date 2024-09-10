use std::{cell::RefCell, rc::Rc};

use abstract_tx::{ChangeClassificationTransaction, UsecaseError};
use payroll_domain::EmployeeId;
use payroll_impl::{classification::CommissionedClassification, schedule::BiweeklySchedule};

pub trait CommissionedChangeableEmployee {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_salary(&self) -> f32;
    fn get_commission_rate(&self) -> f32;
}
pub trait ChangeCommissionedTransaction<Ctx>:
    ChangeClassificationTransaction<Ctx> + CommissionedChangeableEmployee
{
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeClassificationTransaction::<Ctx>::execute(
            self,
            self.get_emp_id(),
            Rc::new(RefCell::new(CommissionedClassification::new(
                self.get_salary(),
                self.get_commission_rate(),
            ))),
            Rc::new(RefCell::new(BiweeklySchedule)),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeCommissionedTransaction<Ctx> for T where
    T: ChangeClassificationTransaction<Ctx> + CommissionedChangeableEmployee
{
}
