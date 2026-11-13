use std::fmt;

use ast::nodes::Span;
use typed::types::Type;

/// Errors produced while lowering a typed AST into Cranelift IR.
#[derive(Debug, Clone)]
pub enum CodegenError {
    MissingChildValue {
        parent_kind: &'static str,
        parent_span: Span,
        role: String,
        child_kind: &'static str,
        child_ty: Type,
        child_span: Span,
    },
    InvalidTypeAlgebra {
        op: &'static str,
        operands: String,
        span: Span,
    },
    UnsupportedOperationType {
        op: &'static str,
        expr: String,
        result_ty: Type,
        span: Span,
    },
    MissingLayout {
        context: &'static str,
        ty: Type,
        span: Option<Span>,
        reason: &'static str,
    },
    UnexpectedMatrixType {
        actual_ty: Type,
        span: Span,
    },
    UnsupportedPointerWidth {
        pointer_bits: usize,
        span: Option<Span>,
    },
    MissingReturnValue {
        node_kind: &'static str,
        ty: Type,
        span: Span,
    },
}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodegenError::MissingChildValue {
                parent_kind,
                parent_span,
                role,
                child_kind,
                child_ty,
                child_span,
            } => write!(
                f,
                "{parent_kind} at {}..{} missing {role} value from {child_kind} (ty={child_ty}) at {}..{}",
                parent_span.start, parent_span.end, child_span.start, child_span.end
            ),
            CodegenError::InvalidTypeAlgebra { op, operands, span } => write!(
                f,
                "type algebra rejected `{op}` for `{operands}` at {}..{}",
                span.start, span.end
            ),
            CodegenError::UnsupportedOperationType {
                op,
                expr,
                result_ty,
                span,
            } => write!(
                f,
                "unsupported {op} CLIF codegen for `{expr}` -> `{result_ty}` at {}..{} (expected scalar integer or float)",
                span.start, span.end
            ),
            CodegenError::MissingLayout {
                context,
                ty,
                span,
                reason,
            } => match span {
                Some(span) => write!(
                    f,
                    "no CLIF layout for {context} type `{ty}` at {}..{}: {reason}",
                    span.start, span.end
                ),
                None => write!(f, "no CLIF layout for {context} type `{ty}`: {reason}"),
            },
            CodegenError::UnexpectedMatrixType { actual_ty, span } => write!(
                f,
                "expected matrix type for Matrix node at {}..{}, got `{actual_ty}`",
                span.start, span.end
            ),
            CodegenError::UnsupportedPointerWidth { pointer_bits, span } => match span {
                Some(span) => write!(
                    f,
                    "unsupported target pointer width `{pointer_bits}` bits at {}..{}",
                    span.start, span.end
                ),
                None => write!(f, "unsupported target pointer width `{pointer_bits}` bits"),
            },
            CodegenError::MissingReturnValue {
                node_kind,
                ty,
                span,
            } => write!(
                f,
                "node `{node_kind}` (ty={ty}) at {}..{} produced no CLIF return value",
                span.start, span.end
            ),
        }
    }
}

impl std::error::Error for CodegenError {}
