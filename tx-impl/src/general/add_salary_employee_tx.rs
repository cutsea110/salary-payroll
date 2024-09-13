use std::{cell::RefCell, rc::Rc};

use abstract_tx::{AddEmployeeTransaction, UsecaseError};
use payroll_domain::EmployeeId;
use payroll_impl::{classification::SalariedClassification, schedule::MonthlySchedule};

pub trait AddSalaryEmployeeTransaction<Ctx>: AddEmployeeTransaction<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        name: &str,
        address: &str,
        salary: f32,
    ) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = UsecaseError>
    where
        Ctx: 'a,
    {
        AddEmployeeTransaction::<Ctx>::execute(
            self,
            emp_id,
            name,
            address,
            Rc::new(RefCell::new(SalariedClassification::new(salary))),
            Rc::new(RefCell::new(MonthlySchedule)),
        )
    }
}
// blanket implementation
impl<T, Ctx> AddSalaryEmployeeTransaction<Ctx> for T where T: AddEmployeeTransaction<Ctx> {}
