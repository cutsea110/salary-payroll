use std::any::Any;

use payroll_domain::Affiliation;

#[derive(Debug, Clone, PartialEq)]
pub struct NoAffiliation;
impl Affiliation for NoAffiliation {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
