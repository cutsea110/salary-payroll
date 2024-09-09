use std::{cell::RefCell, rc::Rc};

use abstract_tx::{ChangeClassificationTransaction, EmployeeUsecaseError};
use payroll_domain::EmployeeId;
use payroll_impl::{
    classification::{CommissionedClassification, HourlyClassification, SalariedClassification},
    schedule::{BiweeklySchedule, MonthlySchedule, WeeklySchedule},
};

pub trait SalaryChangeableEmployee {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_salary(&self) -> f32;
}

pub trait ChangeSalariedTransaction<Ctx>:
    ChangeClassificationTransaction<Ctx> + SalaryChangeableEmployee
{
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
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

pub trait HourlyChangeableEmployee {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_hourly_rate(&self) -> f32;
}
pub trait ChangeHourlyTransaction<Ctx>:
    ChangeClassificationTransaction<Ctx> + HourlyChangeableEmployee
{
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
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

pub trait CommissionedChangeableEmployee {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_salary(&self) -> f32;
    fn get_commission_rate(&self) -> f32;
}
pub trait ChangeCommissionedTransaction<Ctx>:
    ChangeClassificationTransaction<Ctx> + CommissionedChangeableEmployee
{
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
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
