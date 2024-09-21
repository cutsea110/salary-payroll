use std::{cell::RefCell, rc::Rc};

use abstract_tx::{ChangeClassificationTransaction, UsecaseError};
use payroll_domain::EmployeeId;
use payroll_impl::{classification::HourlyClassification, schedule::WeeklySchedule};

pub trait ChangeHourlyTransaction<Ctx>: ChangeClassificationTransaction<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        hourly_rate: f32,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeClassificationTransaction::execute(
            self,
            emp_id,
            Rc::new(RefCell::new(HourlyClassification::new(hourly_rate))),
            Rc::new(RefCell::new(WeeklySchedule)),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeHourlyTransaction<Ctx> for T where T: ChangeClassificationTransaction<Ctx> {}
