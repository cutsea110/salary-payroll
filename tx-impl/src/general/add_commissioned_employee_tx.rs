use std::{cell::RefCell, rc::Rc};

use abstract_tx::{AddEmployeeTransaction, UsecaseError};

use payroll_domain::EmployeeId;
use payroll_impl::{classification::CommissionedClassification, schedule::BiweeklySchedule};

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
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = UsecaseError>
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
