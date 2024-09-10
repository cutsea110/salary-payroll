use abstract_tx::EmployeeUsecaseError;

pub trait Transaction<Ctx> {
    fn execute(&mut self) -> Result<(), EmployeeUsecaseError>;
}
pub trait TransactionSource<Ctx> {
    fn get_transaction(&mut self) -> Option<Box<dyn Transaction<Ctx>>>;
}
pub trait TransactionApplication<Ctx> {
    fn tx_source(&self) -> impl TransactionSource<Ctx>;
    fn run(&mut self) -> Result<(), EmployeeUsecaseError> {
        let mut tx_source = self.tx_source();
        while let Some(mut tx) = tx_source.get_transaction() {
            let _ = tx.execute();
        }
        Ok(())
    }
}
