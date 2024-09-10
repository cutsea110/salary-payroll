use dyn_clone::DynClone;
use std::any::Any;
use std::fmt::Debug;

use crate::bo::Paycheck;

pub trait PaymentClassification: DynClone + Debug {
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn calculate_pay(&self, pc: &Paycheck) -> f32;
}
dyn_clone::clone_trait_object!(PaymentClassification);
