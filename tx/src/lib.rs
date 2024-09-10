use abstract_tx::EmployeeUsecaseError;

pub trait Transaction<Ctx> {
    fn execute(&mut self) -> Result<(), EmployeeUsecaseError>;
}
