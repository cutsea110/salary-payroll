mod change_direct_tx;
mod change_hold_tx;
mod change_mail_tx;

pub use change_direct_tx::{ChangeDirectTransaction, DirectChangeableEmployee};
pub use change_hold_tx::{ChangeHoldTransaction, HoldChangeableEmployee};
pub use change_mail_tx::{ChangeMailTransaction, MailChangeableEmployee};
