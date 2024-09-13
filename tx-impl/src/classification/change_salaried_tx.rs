use std::{cell::RefCell, rc::Rc};

use abstract_tx::{ChangeClassificationTransaction, UsecaseError};
use payroll_domain::EmployeeId;
use payroll_impl::{classification::SalariedClassification, schedule::MonthlySchedule};

pub trait ChangeSalariedTransaction<Ctx>: ChangeClassificationTransaction<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        salary: f32,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeClassificationTransaction::<Ctx>::execute(
            self,
            emp_id,
            Rc::new(RefCell::new(SalariedClassification::new(salary))),
            Rc::new(RefCell::new(MonthlySchedule)),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeSalariedTransaction<Ctx> for T where T: ChangeClassificationTransaction<Ctx> {}
