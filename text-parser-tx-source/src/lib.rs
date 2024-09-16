use parsec_rs::Parser;
use std::collections::VecDeque;

use mock_db::MockDb;
use mock_tx_impl::*;
use parser::{transactions, Command};
use tx_app::{Transaction, TransactionSource};
use tx_factory::TransactionFactory;

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

trait Converter<T, Ctx>
where
    T: TransactionFactory<Ctx>,
{
    fn convert(self, tx_factory: &T) -> Box<dyn Transaction<Ctx> + '_>;
}
impl Converter<TransactionFactoryImpl, ()> for Command {
    fn convert(self, tx_factory: &TransactionFactoryImpl) -> Box<dyn Transaction<()> + '_> {
        match self {
            Command::AddSalaryEmp {
                emp_id,
                name,
                address,
                salary,
            } => Box::new(tx_factory.mk_add_salary_employee_tx(emp_id, name, address, salary)),
            Command::AddHourlyEmp {
                emp_id,
                name,
                address,
                hourly_rate,
            } => Box::new(tx_factory.mk_add_hourly_employee_tx(emp_id, name, address, hourly_rate)),
            Command::AddCommissionedEmp {
                emp_id,
                name,
                address,
                salary,
                commission_rate,
            } => Box::new(tx_factory.mk_add_commissioned_employee_tx(
                emp_id,
                name,
                address,
                salary,
                commission_rate,
            )),
            Command::DelEmp { emp_id } => Box::new(tx_factory.mk_delete_employee_tx(emp_id)),
            Command::TimeCard {
                emp_id,
                date,
                hours,
            } => Box::new(tx_factory.mk_timecard_tx(emp_id, date, hours)),
            Command::SalesReceipt {
                emp_id,
                date,
                amount,
            } => Box::new(tx_factory.mk_sales_receipt_tx(emp_id, date, amount)),
            Command::ServiceCharge {
                member_id,
                date,
                amount,
            } => Box::new(tx_factory.mk_service_charge_tx(member_id, date, amount)),
            Command::ChgName { emp_id, name } => {
                Box::new(tx_factory.mk_change_name_tx(emp_id, name))
            }
            Command::ChgAddress { emp_id, address } => {
                Box::new(tx_factory.mk_change_address_tx(emp_id, address))
            }
            Command::ChgSalaried { emp_id, salary } => {
                Box::new(tx_factory.mk_change_salaried_tx(emp_id, salary))
            }
            Command::ChgHourly {
                emp_id,
                hourly_rate,
            } => Box::new(tx_factory.mk_change_hourly_tx(emp_id, hourly_rate)),
            Command::ChgCommissioned {
                emp_id,
                salary,
                commission_rate,
            } => Box::new(tx_factory.mk_change_commissioned_tx(emp_id, salary, commission_rate)),
            Command::ChgHold { emp_id } => Box::new(tx_factory.mk_change_hold_tx(emp_id)),
            Command::ChgDirect {
                emp_id,
                bank,
                account,
            } => Box::new(tx_factory.mk_change_direct_tx(emp_id, bank, account)),
            Command::ChgMail { emp_id, address } => {
                Box::new(tx_factory.mk_change_mail_tx(emp_id, address))
            }
            Command::ChgMember {
                emp_id,
                member_id,
                dues,
            } => Box::new(tx_factory.mk_change_union_member_tx(emp_id, member_id, dues)),
            Command::ChgNoMember { emp_id } => {
                Box::new(tx_factory.mk_change_unaffiliated_tx(emp_id))
            }
            Command::Payday { pay_date } => Box::new(tx_factory.mk_payday_tx(pay_date)),
        }
    }
}
