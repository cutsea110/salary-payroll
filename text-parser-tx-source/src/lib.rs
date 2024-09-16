mod command;
mod parser;

use command::Command;
use parser::transactions;

use parsec_rs::Parser;
use std::collections::VecDeque;

use mock_db::MockDb;
use mock_tx_impl::*;
use tx_app::{Transaction, TransactionSource};

pub struct TextParserTransactionSource {
    tx_factory: TransactionFactoryImpl,
    commands: VecDeque<Command>,
}
impl TransactionSource<()> for TextParserTransactionSource {
    fn get_transaction(&mut self) -> Option<Box<dyn Transaction<()> + '_>> {
        self.commands
            .pop_front()
            .map(|c| c.convert(&self.tx_factory))
    }
}
impl TextParserTransactionSource {
    pub fn new(db: MockDb, input: String) -> Self {
        let tx_factory = TransactionFactoryImpl::new(db.clone());
        let commands = transactions()
            .parse(&input)
            .map(|p| p.0.into())
            .unwrap_or_default();

        Self {
            tx_factory,
            commands,
        }
    }
}
