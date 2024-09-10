use std::{cell::RefCell, rc::Rc};

use abstract_tx::{ChangeClassificationTransaction, UsecaseError};
use payroll_domain::EmployeeId;
use payroll_impl::{classification::SalariedClassification, schedule::MonthlySchedule};

pub trait SalaryChangeableEmployee {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_salary(&self) -> f32;
}

pub trait ChangeSalariedTransaction<Ctx>:
    ChangeClassificationTransaction<Ctx> + SalaryChangeableEmployee
{
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeClassificationTransaction::<Ctx>::execute(
            self,
            self.get_emp_id(),
            Rc::new(RefCell::new(SalariedClassification::new(self.get_salary()))),
            Rc::new(RefCell::new(MonthlySchedule)),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeSalariedTransaction<Ctx> for T where
    T: ChangeClassificationTransaction<Ctx> + SalaryChangeableEmployee
{
}
