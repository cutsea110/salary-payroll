use chrono::NaiveDate;
use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::EmployeeId;
use payroll_impl::classification::{HourlyClassification, TimeCard};

pub trait TimeCardEmployee {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_date(&self) -> NaiveDate;
    fn get_hours(&self) -> f32;
}
pub trait TimeCardTransaction<Ctx>: HaveEmployeeDao<Ctx> + TimeCardEmployee {
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError> {
        tx_rs::with_tx(move |ctx| {
            let emp = self
                .dao()
                .fetch(self.get_emp_id())
                .run(ctx)
                .map_err(UsecaseError::NotFound)?;
            emp.get_classification()
                .borrow_mut()
                .as_any_mut()
                .downcast_mut::<HourlyClassification>()
                .ok_or(UsecaseError::NotHourlySalary(format!(
                    "emp_id: {}",
                    self.get_emp_id()
                )))?
                .add_timecard(TimeCard::new(self.get_date(), self.get_hours()));
            self.dao()
                .update(emp)
                .run(ctx)
                .map_err(UsecaseError::UpdateEmployeeFailed)
        })
    }
}
// blanket implementation
impl<Ctx, T> TimeCardTransaction<Ctx> for T where T: HaveEmployeeDao<Ctx> + TimeCardEmployee {}
