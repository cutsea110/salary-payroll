use std::{cell::RefCell, rc::Rc};

use abstract_tx::{AddEmployeeTransaction, UsecaseError};
use payroll_domain::EmployeeId;
use payroll_impl::{classification::HourlyClassification, schedule::WeeklySchedule};

pub trait AddHourlyEmployeeTransaction<Ctx>: AddEmployeeTransaction<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        name: &str,
        address: &str,
        hourly_rate: f32,
    ) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = UsecaseError>
    where
        Ctx: 'a,
    {
        AddEmployeeTransaction::<Ctx>::execute(
            self,
            emp_id,
            name,
            address,
            Rc::new(RefCell::new(HourlyClassification::new(hourly_rate))),
            Rc::new(RefCell::new(WeeklySchedule)),
        )
    }
}
// blanket implementation
impl<T, Ctx> AddHourlyEmployeeTransaction<Ctx> for T where T: AddEmployeeTransaction<Ctx> {}
