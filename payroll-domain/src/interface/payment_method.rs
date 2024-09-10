use dyn_clone::DynClone;
use std::fmt::Debug;

use crate::bo::Paycheck;

pub trait PaymentMethod: DynClone + Debug {
    // TODO: return type
    fn pay(&self, pc: &Paycheck);
}
dyn_clone::clone_trait_object!(PaymentMethod);
