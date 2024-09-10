use abstract_tx::UsecaseError;

pub trait Transaction<Ctx> {
    fn execute(&mut self) -> Result<(), UsecaseError>;
}
