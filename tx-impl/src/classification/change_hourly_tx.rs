use std::{cell::RefCell, rc::Rc};

use abstract_tx::{ChangeClassificationTransaction, UsecaseError};
use payroll_domain::EmployeeId;
use payroll_impl::{classification::HourlyClassification, schedule::WeeklySchedule};

pub trait HourlyChangeableEmployee {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_hourly_rate(&self) -> f32;
}
pub trait ChangeHourlyTransaction<Ctx>:
    ChangeClassificationTransaction<Ctx> + HourlyChangeableEmployee
{
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeClassificationTransaction::<Ctx>::execute(
            self,
            self.get_emp_id(),
            Rc::new(RefCell::new(HourlyClassification::new(
                self.get_hourly_rate(),
            ))),
            Rc::new(RefCell::new(WeeklySchedule)),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeHourlyTransaction<Ctx> for T where
    T: ChangeClassificationTransaction<Ctx> + HourlyChangeableEmployee
{
}
