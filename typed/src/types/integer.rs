use std::cmp::max;
use std::fmt;

use super::{Type, TypeAlgebra};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntegerTypeSpec {
    pub signed: bool,
    pub size_bytes: usize,
}

impl IntegerTypeSpec {
    // integer with integer: signed if either side is, widened to the larger side
    fn arithmetic(&self, rhs: &Type) -> Option<Type> {
        match rhs {
            Type::Integer(rhs) => Some(Type::Integer(IntegerTypeSpec {
                signed: self.signed || rhs.signed,
                size_bytes: max(self.size_bytes, rhs.size_bytes),
            })),
            _ => None,
        }
    }
}

impl TypeAlgebra for IntegerTypeSpec {
    // negation only makes sense for signed integers
    fn negate(&self) -> Option<Type> {
        self.signed.then(|| Type::Integer(self.clone()))
    }

    fn add(&self, rhs: &Type) -> Option<Type> {
        self.arithmetic(rhs)
    }
    fn sub(&self, rhs: &Type) -> Option<Type> {
        self.arithmetic(rhs)
    }
    fn mul(&self, rhs: &Type) -> Option<Type> {
        self.arithmetic(rhs)
    }
    fn div(&self, rhs: &Type) -> Option<Type> {
        self.arithmetic(rhs)
    }
}

impl fmt::Display for IntegerTypeSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            if self.signed { "i" } else { "u" },
            self.size_bytes * 8
        )
    }
}
