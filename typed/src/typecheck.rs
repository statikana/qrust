use std::fmt;

use ast::nodes::{UntypedNode, Node, NodeKind, Span};
use ast::operator::{BinaryOperator, UnaryOperator};

use crate::operator::{BinaryTyping, UnaryTyping};
use crate::typed_debug;
use crate::types::{FloatTypeSpec, IntegerTypeSpec, MatrixTypeSpec, Type};

/// A typed `Node`, produced by `typecheck`
pub type TypedNode = Node<Type>;

#[derive(Debug)]
pub enum TypeError {
    UnsupportedUnaryOperation {
        op: UnaryOperator,
        subject: Type,
        span: Span,
    },
    UnsupportedBinaryOperation {
        op: BinaryOperator,
        lhs: Type,
        rhs: Type,
        span: Span,
    },
    NonuniformMatrixType {
        span: Span,
    },
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeError::UnsupportedUnaryOperation { op, subject, span } => write!(
                f,
                "unsupported unary operation `{} {}` at {}..{}",
                op.symbol(),
                subject,
                span.start,
                span.end
            ),
            TypeError::UnsupportedBinaryOperation { op, lhs, rhs, span } => write!(
                f,
                "unsupported binary operation `{} {} {}` at {}..{}",
                lhs,
                op.symbol(),
                rhs,
                span.start,
                span.end
            ),
            TypeError::NonuniformMatrixType { span } => write!(
                f,
                "non-uniform matrix element types at {}..{}",
                span.start, span.end
            ),
        }
    }
}

// types the children first, then lets the node decide its own type from theirs
pub fn typecheck(node: &UntypedNode) -> Result<TypedNode, TypeError> {
    let result = typecheck_node(node);
    typed_debug::log_typecheck(node, &result);
    result
}

fn typecheck_node(node: &UntypedNode) -> Result<TypedNode, TypeError> {
    let span = node.span;
    let (kind, ty) = match &node.kind {
        // statements are checked, but don't produce a value
        NodeKind::Scope { statements } => {
            let statements = statements
                .iter()
                .map(typecheck)
                .collect::<Result<Vec<_>, _>>()?;
            (NodeKind::Scope { statements }, Type::Unit)
        }
        // the operator decides the result from the operand types
        NodeKind::BinaryOperation { op, lhs, rhs } => {
            let lhs = Box::new(typecheck(lhs)?);
            let rhs = Box::new(typecheck(rhs)?);
            // TODO: implementation of reverse operations is possible here
            let ty = op.result_type(&lhs.ty, &rhs.ty).ok_or(
                TypeError::UnsupportedBinaryOperation {
                    op: *op,
                    lhs: lhs.ty.clone(),
                    rhs: rhs.ty.clone(),
                    span,
                },
            )?;
            (NodeKind::BinaryOperation { op: *op, lhs, rhs }, ty)
        }
        // the operator decides the result from the subject's type
        NodeKind::UnaryOperation { op, subject } => {
            let subject = Box::new(typecheck(subject)?);
            let ty =
                op.result_type(&subject.ty)
                    .ok_or(TypeError::UnsupportedUnaryOperation {
                        op: *op,
                        subject: subject.ty.clone(),
                        span,
                    })?;
            (NodeKind::UnaryOperation { op: *op, subject }, ty)
        }
        NodeKind::Boolean(val) => (NodeKind::Boolean(*val), Type::Boolean),
        NodeKind::Integer(val) => (
            NodeKind::Integer(*val),
            Type::Integer(IntegerTypeSpec {
                signed: true,
                size_bytes: 8,
            }), // TODO: add actual ints
        ),
        NodeKind::Float(val) => (
            NodeKind::Float(*val),
            Type::Float(FloatTypeSpec { size_bytes: 8 }), // TODO: add actual floats
        ),
        NodeKind::Unit => (NodeKind::Unit, Type::Unit),
        NodeKind::Matrix { elements } => {
            let elements = elements
                .iter()
                .map(typecheck)
                .collect::<Result<Vec<_>, _>>()?;
            let ty = typecheck_matrix(&elements, span)?;
            (NodeKind::Matrix { elements }, ty)
        }
    };

    Ok(TypedNode { kind, span, ty })
}

fn typecheck_matrix(elements: &[TypedNode], span: Span) -> Result<Type, TypeError> {
    // all elements of the same type
    // yes this breaks if the matrix is empty
    let first_element_type = &elements
        .first()
        .unwrap_or_else(|| {
            log::error!("Cannot typecheck empty matrix at {}..{}", span.start, span.end);
            panic!("Empty matrix literal at {}..{}", span.start, span.end);
        })
        .ty;
    if !elements.iter().all(|element| &element.ty == first_element_type) {
        return Err(TypeError::NonuniformMatrixType { span });
    }

    let mut shape = vec![elements.len()];

    /*
        we need to now collapse the matrix as best we can
        because `typecheck` is reverse recursive (the elements are already typechecked),
        we may assume any further inner matrices have already been collapsed into this element.

        eg:
        [
            [ 1, 2, 3],
            [ 40, 50, 60],
        ]
        is interpreted as a matrix of shape=[2,], with element_type=(matrix of shape=[3,] and element_type=i64)
        after collapse: matrix of shape=[2,3] and element_type=i64
    */
    let mut element_type = first_element_type;
    if let Type::Matrix(MatrixTypeSpec {
        shape: inner_shape,
        element_type: inner_element_type,
    }) = first_element_type
    {
        shape.extend(inner_shape);
        element_type = inner_element_type;
    }
    let ty = Type::Matrix(MatrixTypeSpec {
        shape,
        element_type: Box::new(element_type.clone()),
    });
    typed_debug::log_matrix_collapse(span, elements.len(), first_element_type, &ty);
    Ok(ty)
}
