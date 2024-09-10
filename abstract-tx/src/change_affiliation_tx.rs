use std::{cell::RefCell, rc::Rc};

use crate::change_employee_tx::ChangeEmployeeTransaction;
use crate::error::UsecaseError;
use payroll_domain::{Affiliation, Employee, EmployeeId};

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
