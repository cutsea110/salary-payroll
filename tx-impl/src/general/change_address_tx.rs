use abstract_tx::{ChangeEmployeeTransaction, UsecaseError};
use payroll_domain::EmployeeId;

pub trait ChangeAddressTransaction<Ctx>: ChangeEmployeeTransaction<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        address: &str,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeeTransaction::execute(self, emp_id, |_ctx, emp| {
            emp.set_address(address);
            Ok(())
        })
    }
}
// blanket implementation
impl<T, Ctx> ChangeAddressTransaction<Ctx> for T where T: ChangeEmployeeTransaction<Ctx> {}
