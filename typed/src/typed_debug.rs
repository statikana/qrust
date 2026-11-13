use ast::ast_debug::DebugTypeAnnotation;
use ast::nodes::{Span, UntypedNode};

use crate::typecheck::{TypeError, TypedNode};
use crate::types::Type;

// Debug label formatting for the typed AST, the counterpart of `ast::ast_debug`.
// each node kind formats itself through `NodeKind::fmt_labeled`; this just builds the label.

impl DebugTypeAnnotation for Type {
    // like `UntypedNode`, with the type appended to the label, e.g. `Integer@3..4: i64(5)`
    fn format_label(&self, name: &str, span: Span) -> String {
        format!("{}@{}..{}: {}", name, span.start, span.end, self)
    }
}

/// Logs the resolved type (or error) for an AST node at the `typecheck` boundary.
pub(crate) fn log_typecheck(node: &UntypedNode, result: &Result<TypedNode, TypeError>) {
    match result {
        Ok(typed) => log::trace!(
            target: "typed::typecheck",
            "[type] {:<16} span={}..{} -> {}",
            node.kind.name(),
            node.span.start,
            node.span.end,
            typed.ty
        ),
        Err(err) => log::error!(
            target: "typed::typecheck",
            "[type] {:<16} span={}..{} failed: {err}",
            node.kind.name(),
            node.span.start,
            node.span.end
        ),
    }
}

/// Logs matrix element validation and nested shape collapsing.
pub(crate) fn log_matrix_collapse(
    span: Span,
    element_count: usize,
    first_element_type: &Type,
    collapsed: &Type,
) {
    log::trace!(
        target: "typed::typecheck",
        "[matrix] span={}..{} len={} elem={} -> {}",
        span.start,
        span.end,
        element_count,
        first_element_type,
        collapsed
    );
}
