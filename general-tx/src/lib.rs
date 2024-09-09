use chrono::NaiveDate;
use std::{cell::RefCell, rc::Rc};
use tx_rs::Tx;

use abstract_tx::{AddEmployeeTransaction, ChangeEmployeeTransaction, EmployeeUsecaseError};
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{EmployeeId, PayCheck};
use payroll_impl::{
    classification::{
        CommissionedClassification, HourlyClassification, SalariedClassification, SalesReceipt,
        TimeCard,
    },
    schedule::{BiweeklySchedule, MonthlySchedule, WeeklySchedule},
};

pub trait SalaryEmployee {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_name(&self) -> &str;
    fn get_address(&self) -> &str;
    fn get_salary(&self) -> f32;
}
pub trait AddSalaryEmployeeTransaction<Ctx>: AddEmployeeTransaction<Ctx> + SalaryEmployee {
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        let emp_id = self.get_emp_id();
        let name = self.get_name();
        let address = self.get_address();
        let classification = Rc::new(RefCell::new(SalariedClassification::new(self.get_salary())));
        let schedule = Rc::new(RefCell::new(MonthlySchedule));

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
impl<T, Ctx> AddSalaryEmployeeTransaction<Ctx> for T where
    T: AddEmployeeTransaction<Ctx> + SalaryEmployee
{
}

pub trait HourlyEmployee {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_name(&self) -> &str;
    fn get_address(&self) -> &str;
    fn get_hourly_rate(&self) -> f32;
}
pub trait AddHourlyEmployeeTransaction<Ctx>: AddEmployeeTransaction<Ctx> + HourlyEmployee {
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = EmployeeUsecaseError>
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

pub trait CommissionedEmployee {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_name(&self) -> &str;
    fn get_address(&self) -> &str;
    fn get_salary(&self) -> f32;
    fn get_commission_rate(&self) -> f32;
}
pub trait AddCommissionedEmployeeTransaction<Ctx>:
    AddEmployeeTransaction<Ctx> + CommissionedEmployee
{
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        let emp_id = self.get_emp_id();
        let name = self.get_name();
        let address = self.get_address();
        let classification = Rc::new(RefCell::new(CommissionedClassification::new(
            self.get_salary(),
            self.get_commission_rate(),
        )));
        let schedule = Rc::new(RefCell::new(BiweeklySchedule));

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
impl<T, Ctx> AddCommissionedEmployeeTransaction<Ctx> for T where
    T: AddEmployeeTransaction<Ctx> + CommissionedEmployee
{
}

pub trait DeletableEmployee {
    fn get_emp_id(&self) -> EmployeeId;
}
pub trait DeleteEmployeeTransaction<Ctx>: HaveEmployeeDao<Ctx> + DeletableEmployee {
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        self.dao()
            .delete(self.get_emp_id())
            .map_err(EmployeeUsecaseError::UnregisterEmployeeFailed)
    }
}
// blanket implementation
impl<Ctx, T> DeleteEmployeeTransaction<Ctx> for T where T: HaveEmployeeDao<Ctx> + DeletableEmployee {}

pub trait TimeCardEmployee {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_date(&self) -> NaiveDate;
    fn get_hours(&self) -> f32;
}
pub trait TimeCardTransaction<Ctx>: HaveEmployeeDao<Ctx> + TimeCardEmployee {
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError> {
        tx_rs::with_tx(move |ctx| {
            let emp = self
                .dao()
                .fetch(self.get_emp_id())
                .run(ctx)
                .map_err(EmployeeUsecaseError::NotFound)?;
            emp.get_classification()
                .borrow_mut()
                .as_any_mut()
                .downcast_mut::<HourlyClassification>()
                .ok_or(EmployeeUsecaseError::NotHourlySalary(format!(
                    "emp_id: {}",
                    self.get_emp_id()
                )))?
                .add_timecard(TimeCard::new(self.get_date(), self.get_hours()));
            self.dao()
                .update(emp)
                .run(ctx)
                .map_err(EmployeeUsecaseError::UpdateEmployeeFailed)
        })
    }
}
// blanket implementation
impl<Ctx, T> TimeCardTransaction<Ctx> for T where T: HaveEmployeeDao<Ctx> + TimeCardEmployee {}

pub trait SalesReceiptEmployee {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_date(&self) -> NaiveDate;
    fn get_amount(&self) -> f32;
}
pub trait SalesReceiptTransaction<Ctx>: HaveEmployeeDao<Ctx> + SalesReceiptEmployee {
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError> {
        tx_rs::with_tx(move |ctx| {
            let emp = self
                .dao()
                .fetch(self.get_emp_id())
                .run(ctx)
                .map_err(EmployeeUsecaseError::NotFound)?;
            emp.get_classification()
                .borrow_mut()
                .as_any_mut()
                .downcast_mut::<CommissionedClassification>()
                .ok_or(EmployeeUsecaseError::NotCommissionedSalary(format!(
                    "emp_id: {}",
                    self.get_emp_id()
                )))?
                .add_sales_receipt(SalesReceipt::new(self.get_date(), self.get_amount()));
            self.dao()
                .update(emp)
                .run(ctx)
                .map_err(EmployeeUsecaseError::UpdateEmployeeFailed)
        })
    }
}
// blanket implementation
impl<Ctx, T> SalesReceiptTransaction<Ctx> for T where T: HaveEmployeeDao<Ctx> + SalesReceiptEmployee {}

pub trait NameChangeableEmployee {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_name(&self) -> &str;
}
pub trait ChangeNameTransaction<Ctx>:
    ChangeEmployeeTransaction<Ctx> + NameChangeableEmployee
{
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeeTransaction::<Ctx>::execute(self, self.get_emp_id(), |_ctx, emp| {
            emp.set_name(self.get_name());
            Ok(())
        })
    }
}
// blanket implementation
impl<T, Ctx> ChangeNameTransaction<Ctx> for T where
    T: ChangeEmployeeTransaction<Ctx> + NameChangeableEmployee
{
}

pub trait AddressChangeableEmployee {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_address(&self) -> &str;
}
pub trait ChangeAddressTransaction<Ctx>:
    ChangeEmployeeTransaction<Ctx> + AddressChangeableEmployee
{
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeeTransaction::<Ctx>::execute(self, self.get_emp_id(), |_ctx, emp| {
            emp.set_address(self.get_address());
            Ok(())
        })
    }
}
// blanket implementation
impl<T, Ctx> ChangeAddressTransaction<Ctx> for T where
    T: ChangeEmployeeTransaction<Ctx> + AddressChangeableEmployee
{
}

pub trait PayableEmployee {
    fn get_pay_date(&self) -> NaiveDate;
}
pub trait PaydayTransaction<Ctx>: HaveEmployeeDao<Ctx> + PayableEmployee {
    fn execute<'a>(&mut self) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeUsecaseError>
    where
        Ctx: 'a,
    {
        tx_rs::with_tx(|ctx| {
            let mut employees = self
                .dao()
                .get_all()
                .run(ctx)
                .map_err(EmployeeUsecaseError::GetAllFailed)?;
            let pay_date = self.get_pay_date();
            for emp in employees.iter_mut() {
                if emp.is_pay_date(pay_date) {
                    let period = emp.get_pay_period(pay_date);
                    let mut pc = PayCheck::new(period);
                    emp.payday(&mut pc);
                    self.dao()
                        .record_paycheck(emp.get_emp_id(), pc)
                        .run(ctx)
                        .map_err(EmployeeUsecaseError::UpdateEmployeeFailed)?;
                }
            }
            Ok(())
        })
    }
}
// blanket implementation
impl<Ctx, T> PaydayTransaction<Ctx> for T where T: HaveEmployeeDao<Ctx> + PayableEmployee {}
