use chrono::NaiveDate;
use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::EmployeeId;
use payroll_impl::classification::{CommissionedClassification, SalesReceipt};

pub trait SalesReceiptTransaction<Ctx>: HaveEmployeeDao<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        date: NaiveDate,
        amount: f32,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError> {
        tx_rs::with_tx(move |ctx| {
            let emp = self
                .dao()
                .fetch(emp_id)
                .run(ctx)
                .map_err(UsecaseError::NotFound)?;
            emp.get_classification()
                .borrow_mut()
                .as_any_mut()
                .downcast_mut::<CommissionedClassification>()
                .ok_or(UsecaseError::NotCommissionedSalary(format!(
                    "emp_id: {}",
                    emp_id
                )))?
                .add_sales_receipt(SalesReceipt::new(date, amount));
            self.dao()
                .update(emp)
                .run(ctx)
                .map_err(UsecaseError::UpdateEmployeeFailed)
        })
    }
}
// blanket implementation
impl<Ctx, T> SalesReceiptTransaction<Ctx> for T where T: HaveEmployeeDao<Ctx> {}
