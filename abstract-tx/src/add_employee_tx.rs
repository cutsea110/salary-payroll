use std::{cell::RefCell, rc::Rc};
use tx_rs::Tx;

use crate::error::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{Employee, EmployeeId, PaymentClassification, PaymentSchedule};
use payroll_impl::{affiliation::NoAffiliation, method::HoldMethod};

pub trait AddEmployeeTransaction<Ctx>: HaveEmployeeDao<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        name: &str,
        address: &str,
        classification: Rc<RefCell<dyn PaymentClassification>>,
        schedule: Rc<RefCell<dyn PaymentSchedule>>,
    ) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = UsecaseError>
    where
        Ctx: 'a,
    {
        let method = Rc::new(RefCell::new(HoldMethod));
        let affiliation = Rc::new(RefCell::new(NoAffiliation));
        let emp = Employee::new(
            emp_id,
            name,
            address,
            classification,
            schedule,
            method,
            affiliation,
        );
        self.dao()
            .insert(emp)
            .map_err(UsecaseError::RegisterEmployeeFailed)
    }
}
// blanket implementation
impl<T, Ctx> AddEmployeeTransaction<Ctx> for T where T: HaveEmployeeDao<Ctx> {}
