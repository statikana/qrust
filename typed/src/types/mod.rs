use std::fmt;

pub mod float;
pub mod integer;
pub mod matrix;
pub mod ptr;

pub use float::FloatTypeSpec;
pub use integer::IntegerTypeSpec;
pub use matrix::MatrixTypeSpec;
pub use ptr::PtrTypeSpec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Unit,
    Boolean,
    Integer(IntegerTypeSpec),
    Float(FloatTypeSpec),
    Ptr(PtrTypeSpec),
    Matrix(MatrixTypeSpec),
}

/// arithmetic rules for a type, one method per operator. `None` (the default) means the operation isn't supported
pub trait TypeAlgebra {
    /// the type of `-self`
    fn negate(&self) -> Option<Type> {
        None
    }
    /// the type of `self + rhs`
    fn add(&self, _rhs: &Type) -> Option<Type> {
        None
    }
    /// the type of `self - rhs`
    fn sub(&self, _rhs: &Type) -> Option<Type> {
        None
    }
    /// the type of `self * rhs`
    fn mul(&self, _rhs: &Type) -> Option<Type> {
        None
    }
    /// the type of `self / rhs`
    fn div(&self, _rhs: &Type) -> Option<Type> {
        None
    }
}

impl Type {
    /// the `TypeAlgebra` impl of the type inside, if it has one
    fn algebra(&self) -> Option<&dyn TypeAlgebra> {
        match self {
            Type::Unit => None,
            Type::Boolean => None,
            Type::Integer(t) => Some(t),
            Type::Float(t) => Some(t),
            Type::Ptr(t) => Some(t),
            Type::Matrix(t) => Some(t),
        }
    }

    /// applies the binary operation `op` to `self` and `rhs`
    fn binary(&self, rhs: &Type, op: fn(&dyn TypeAlgebra, &Type) -> Option<Type>) -> Option<Type> {
        match (self, rhs) {
            // (Type::Matrix(_), _) => op(self.algebra()?, rhs),
            // // scalar op matrix: broadcast the scalar over the matrix's elements
            // (_, Type::Matrix(m)) => Some(m.with_element_type(self.binary(&m.element_type, op)?)),
            _ => op(self.algebra()?, rhs),
        }
    }
}

impl TypeAlgebra for Type {
    fn negate(&self) -> Option<Type> {
        self.algebra()?.negate()
    }
    fn add(&self, rhs: &Type) -> Option<Type> {
        self.binary(rhs, |t, rhs| t.add(rhs))
    }
    fn sub(&self, rhs: &Type) -> Option<Type> {
        self.binary(rhs, |t, rhs| t.sub(rhs))
    }
    fn mul(&self, rhs: &Type) -> Option<Type> {
        self.binary(rhs, |t, rhs| t.mul(rhs))
    }
    fn div(&self, rhs: &Type) -> Option<Type> {
        self.binary(rhs, |t, rhs| t.div(rhs))
    }
}

// compact names, e.g. `i64`, `*f32`, `[2, 3]f64`
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Unit => f.write_str("unit"),
            Type::Boolean => f.write_str("bool"),
            Type::Integer(t) => t.fmt(f),
            Type::Float(t) => t.fmt(f),
            Type::Ptr(t) => t.fmt(f),
            Type::Matrix(t) => t.fmt(f),
        }
    }
}
