use std::{cell::RefCell, rc::Rc};

use abstract_tx::{AddEmployeeTransaction, UsecaseError};
use payroll_domain::EmployeeId;
use payroll_impl::{classification::HourlyClassification, schedule::WeeklySchedule};

pub trait HourlyEmployee {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_name(&self) -> &str;
    fn get_address(&self) -> &str;
    fn get_hourly_rate(&self) -> f32;
}
pub trait AddHourlyEmployeeTransaction<Ctx>: AddEmployeeTransaction<Ctx> + HourlyEmployee {
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = UsecaseError>
    where
        Ctx: 'a,
    {
        let emp_id = self.get_emp_id();
        let name = self.get_name();
        let address = self.get_address();
        let classification = Rc::new(RefCell::new(HourlyClassification::new(
            self.get_hourly_rate(),
        )));
        let schedule = Rc::new(RefCell::new(WeeklySchedule));

        AddEmployeeTransaction::<Ctx>::execute(
            self,
            emp_id,
            name,
            address,
            classification,
            schedule,
        )
    }
}
// blanket implementation
impl<T, Ctx> AddHourlyEmployeeTransaction<Ctx> for T where
    T: AddEmployeeTransaction<Ctx> + HourlyEmployee
{
}
