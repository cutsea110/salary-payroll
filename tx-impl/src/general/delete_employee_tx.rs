use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::EmployeeId;

pub trait DeleteEmployeeTransaction<Ctx>: HaveEmployeeDao<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        self.dao()
            .delete(emp_id)
            .map_err(UsecaseError::UnregisterEmployeeFailed)
    }
}
// blanket implementation
impl<Ctx, T> DeleteEmployeeTransaction<Ctx> for T where T: HaveEmployeeDao<Ctx> {}
