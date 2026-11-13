use std::fmt;

use super::{Type, TypeAlgebra};

// type algebra
// display

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatrixTypeSpec {
    pub shape: Vec<usize>,
    pub element_type: Box<Type>,
}

impl MatrixTypeSpec {
    /// element-wise operations are performed on every element
    /// so the element type of the result is `self.element_type` and `rhs`
    /// under `op`
    fn elementwise(&self, op: fn(&Type, &Type) -> Option<Type>, rhs: &Type) -> Option<Type> {
        Some(Type::Matrix(MatrixTypeSpec {
            shape: self.shape.clone(),
            element_type: Box::new(op(&*self.element_type, rhs).unwrap()),
        }))
    }
}

impl TypeAlgebra for MatrixTypeSpec {
    // element-wise ops
    fn add(&self, rhs: &Type) -> Option<Type> {
        self.elementwise(TypeAlgebra::add, rhs)
    }
    fn sub(&self, rhs: &Type) -> Option<Type> {
        self.elementwise(TypeAlgebra::sub, rhs)
    }
    fn mul(&self, rhs: &Type) -> Option<Type> {
        self.elementwise(TypeAlgebra::mul, rhs)
    }
    fn div(&self, rhs: &Type) -> Option<Type> {
        self.elementwise(TypeAlgebra::div, rhs)
    }
}

impl fmt::Display for MatrixTypeSpec {
    // shape, then element type, e.g. `[2, 3]f64`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{}", self.shape, self.element_type)
    }
}
