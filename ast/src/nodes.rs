use std::fmt;

use crate::operator::{BinaryOperator, UnaryOperator};

#[derive(Clone, Copy, Debug)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

// Node<()>   = UntypedNode, made by the ASTVisitor
// Node<Type> = TypedNode, made by the typechecker (see the `typed` crate)
pub struct Node<Ty = ()> {
    pub kind: NodeKind<Ty>,
    pub span: Span,
    /// the node's type: `()` before typechecking, `typed::types::Type` after
    pub ty: Ty,
}

pub type UntypedNode = Node<()>;

impl UntypedNode {
    pub fn new(kind: NodeKind<()>, span: Span) -> UntypedNode {
        Node {
            kind,
            span,
            ty: (),
        }
    }
}

/// every kind of node. a closed set, so later passes (e.g. the typechecker) can `match` on it exhaustively
pub enum NodeKind<Ty = ()> {
    Scope {
        statements: Vec<Node<Ty>>,
    },
    BinaryOperation {
        op: BinaryOperator,
        lhs: Box<Node<Ty>>,
        rhs: Box<Node<Ty>>,
    },
    UnaryOperation {
        op: UnaryOperator,
        subject: Box<Node<Ty>>,
    },
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Unit,
    Matrix {
        elements: Vec<Node<Ty>>,
    },
}

impl<Ty> NodeKind<Ty> {
    /// used as the node's label when printing
    pub fn name(&self) -> &'static str {
        match self {
            NodeKind::Scope { .. } => "Scope",
            NodeKind::BinaryOperation { .. } => "BinaryOperation",
            NodeKind::UnaryOperation { .. } => "UnaryOperation",
            NodeKind::Boolean(_) => "Boolean",
            NodeKind::Integer(_) => "Integer",
            NodeKind::Float(_) => "Float",
            NodeKind::Unit => "Unit",
            NodeKind::Matrix { .. } => "Matrix",
        }
    }
}

impl<Ty> NodeKind<Ty>
where
    Node<Ty>: fmt::Debug,
{
    /// formats this node under `label`
    pub fn fmt_labeled(&self, label: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeKind::Scope { statements } => f.debug_tuple(label).field(statements).finish(),
            NodeKind::BinaryOperation { op, lhs, rhs } => f
                .debug_struct(label)
                .field("op", op)
                .field("lhs", lhs)
                .field("rhs", rhs)
                .finish(),
            NodeKind::UnaryOperation { op, subject } => f
                .debug_struct(label)
                .field("op", op)
                .field("subject", subject)
                .finish(),
            NodeKind::Boolean(v) => f.debug_tuple(label).field(v).finish(),
            NodeKind::Integer(v) => f.debug_tuple(label).field(v).finish(),
            NodeKind::Float(v) => f.debug_tuple(label).field(v).finish(),
            NodeKind::Unit => f.write_str(label),
            NodeKind::Matrix { elements } => {
                f.debug_struct(label).field("elements", elements).finish()
            }
        }
    }
}
