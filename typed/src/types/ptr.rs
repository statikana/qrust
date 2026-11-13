use std::fmt;

use super::{Type, TypeAlgebra};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PtrTypeSpec {
    pub pointee: Box<Type>,
}

// no arithmetic on pointers
impl TypeAlgebra for PtrTypeSpec {}

impl fmt::Display for PtrTypeSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "*{}", self.pointee)
    }
}
