use std::{cell::RefCell, rc::Rc};
use thiserror::Error;
use tx_rs::Tx;

use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{
    Affiliation, Employee, EmployeeId, PaymentClassification, PaymentMethod, PaymentSchedule,
};
use payroll_impl::{affiliation::NoAffiliation, method::HoldMethod};

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum UsecaseError {
    #[error("register employee failed: {0}")]
    RegisterEmployeeFailed(DaoError),
    #[error("unregister employee failed: {0}")]
    UnregisterEmployeeFailed(DaoError),
    #[error("employee not found: {0}")]
    NotFound(DaoError),
    #[error("can't get all employees: {0}")]
    GetAllFailed(DaoError),
    #[error("employee is not hourly salary: {0}")]
    NotHourlySalary(String),
    #[error("employee is not commissioned salary: {0}")]
    NotCommissionedSalary(String),
    #[error("update employee failed: {0}")]
    UpdateEmployeeFailed(DaoError),
    #[error("employee is not union member: {0}")]
    NotUnionMember(String),
    #[error("add union member failed: {0}")]
    AddUnionMemberFailed(DaoError),
    #[error("remove union member failed: {0}")]
    RemoveUnionMemberFailed(DaoError),
}

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

pub trait ChangeEmployeeTransaction<Ctx>: HaveEmployeeDao<Ctx> {
    fn execute<'a, F>(
        &'a self,
        emp_id: EmployeeId,
        f: F,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        F: FnOnce(&mut Ctx, &mut Employee) -> Result<(), UsecaseError>,
        Ctx: 'a,
    {
        tx_rs::with_tx(move |ctx| {
            let mut emp = self
                .dao()
                .fetch(emp_id)
                .run(ctx)
                .map_err(UsecaseError::NotFound)?;
            f(ctx, &mut emp)?;
            self.dao()
                .update(emp)
                .run(ctx)
                .map_err(UsecaseError::UpdateEmployeeFailed)
        })
    }
}
// blanket implementation
impl<Ctx, T> ChangeEmployeeTransaction<Ctx> for T where T: HaveEmployeeDao<Ctx> {}

pub trait ChangeClassificationTransaction<Ctx>: ChangeEmployeeTransaction<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        classification: Rc<RefCell<dyn PaymentClassification>>,
        schedule: Rc<RefCell<dyn PaymentSchedule>>,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeeTransaction::<Ctx>::execute(self, emp_id, |_ctx, emp| {
            emp.set_classification(classification);
            emp.set_schedule(schedule);
            Ok(())
        })
    }
}
// blanket implementation
impl<Ctx, T> ChangeClassificationTransaction<Ctx> for T where T: ChangeEmployeeTransaction<Ctx> {}

pub trait ChangeMethodTransaction<Ctx>: ChangeEmployeeTransaction<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        method: Rc<RefCell<dyn PaymentMethod>>,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeeTransaction::<Ctx>::execute(self, emp_id, |_ctx, emp| {
            emp.set_method(method);
            Ok(())
        })
    }
}
// blanket implementation
impl<Ctx, T> ChangeMethodTransaction<Ctx> for T where T: ChangeEmployeeTransaction<Ctx> {}

pub trait ChangeAffiliationTransaction<Ctx>: ChangeEmployeeTransaction<Ctx> {
    fn execute<'a, F>(
        &'a self,
        emp_id: EmployeeId,
        record_membership: F,
        affiliation: Rc<RefCell<dyn Affiliation>>,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        F: FnOnce(&mut Ctx, &mut Employee) -> Result<(), UsecaseError>,
        Ctx: 'a,
    {
        ChangeEmployeeTransaction::<Ctx>::execute(self, emp_id, |ctx, emp| {
            record_membership(ctx, emp)?;
            emp.set_affiliation(affiliation);
            Ok(())
        })
    }
}
// blanket implementation
impl<Ctx, T> ChangeAffiliationTransaction<Ctx> for T where T: ChangeEmployeeTransaction<Ctx> {}
