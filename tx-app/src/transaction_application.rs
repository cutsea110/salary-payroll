use abstract_tx::UsecaseError;

use crate::transaction_source::TransactionSource;

pub trait TransactionApplication<Ctx> {
    fn tx_source(&self) -> impl TransactionSource<Ctx>;
    fn run(&mut self) -> Result<(), UsecaseError> {
        let mut tx_source = self.tx_source();
        while let Some(mut tx) = tx_source.get_transaction() {
            tx.execute()?;
        }
        Ok(())
    }
}
