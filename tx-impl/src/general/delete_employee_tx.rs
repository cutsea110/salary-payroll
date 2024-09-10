use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::EmployeeId;

pub trait DeletableEmployee {
    fn get_emp_id(&self) -> EmployeeId;
}
pub trait DeleteEmployeeTransaction<Ctx>: HaveEmployeeDao<Ctx> + DeletableEmployee {
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        self.dao()
            .delete(self.get_emp_id())
            .map_err(UsecaseError::UnregisterEmployeeFailed)
    }
}
// blanket implementation
impl<Ctx, T> DeleteEmployeeTransaction<Ctx> for T where T: HaveEmployeeDao<Ctx> + DeletableEmployee {}
