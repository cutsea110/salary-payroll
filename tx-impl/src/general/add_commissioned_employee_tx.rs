use std::{cell::RefCell, rc::Rc};

use abstract_tx::{AddEmployeeTransaction, UsecaseError};
use payroll_domain::EmployeeId;
use payroll_impl::{classification::CommissionedClassification, schedule::BiweeklySchedule};

pub trait AddCommissionedEmployeeTransaction<Ctx>: AddEmployeeTransaction<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        name: &str,
        address: &str,
        salary: f32,
        commission_rate: f32,
    ) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = UsecaseError>
    where
        Ctx: 'a,
    {
        AddEmployeeTransaction::<Ctx>::execute(
            self,
            emp_id,
            name,
            address,
            Rc::new(RefCell::new(CommissionedClassification::new(
                salary,
                commission_rate,
            ))),
            Rc::new(RefCell::new(BiweeklySchedule)),
        )
    }
}
// blanket implementation
impl<T, Ctx> AddCommissionedEmployeeTransaction<Ctx> for T where T: AddEmployeeTransaction<Ctx> {}
