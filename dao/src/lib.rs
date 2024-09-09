use payroll_domain::{Employee, EmployeeId, MemberId, Paycheck};
use thiserror::Error;

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum EmployeeDaoError {
    #[error("insert error: {0}")]
    InsertError(String),
    #[error("delete error: {0}")]
    DeleteError(String),
    #[error("fetch error: {0}")]
    FetchError(String),
    #[error("update error: {0}")]
    UpdateError(String),
}
pub trait EmployeeDao<Ctx> {
    fn insert(
        &self,
        emp: Employee,
    ) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = EmployeeDaoError>;
    fn delete(&self, emp_id: EmployeeId) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeDaoError>;
    fn fetch(
        &self,
        emp_id: EmployeeId,
    ) -> impl tx_rs::Tx<Ctx, Item = Employee, Err = EmployeeDaoError>;
    fn update(&self, emp: Employee) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeDaoError>;
    fn get_all(&self) -> impl tx_rs::Tx<Ctx, Item = Vec<Employee>, Err = EmployeeDaoError>;
    fn add_union_member(
        &self,
        member_id: MemberId,
        emp_id: EmployeeId,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeDaoError>;
    fn remove_union_member(
        &self,
        member_id: MemberId,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeDaoError>;
    fn find_union_member(
        &self,
        member_id: MemberId,
    ) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = EmployeeDaoError>;
    fn record_paycheck(
        &self,
        emp_id: EmployeeId,
        pc: Paycheck,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = EmployeeDaoError>;
}
pub trait HaveEmployeeDao<Ctx> {
    fn dao(&self) -> Box<&impl EmployeeDao<Ctx>>;
}
