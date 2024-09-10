use thiserror::Error;

use dao::DaoError;

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
