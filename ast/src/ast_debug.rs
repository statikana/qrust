use std::fmt;

use crate::nodes::{Node, Span};

// Debug impls for the AST, kept out of nodes/mod.rs so the type definitions stay readable.
// each node kind formats itself through `NodeKind::fmt_labeled`; these just build the label.

pub trait DebugTypeAnnotation {
    fn format_label(&self, name: &str, span: Span) -> String;
}

impl DebugTypeAnnotation for () {
    // the span is folded into the node's label, e.g. `Integer@3..4(5)`
    fn format_label(&self, name: &str, span: Span) -> String {
        format!("{}@{}..{}", name, span.start, span.end)
    }
}

impl<Ty: DebugTypeAnnotation> fmt::Debug for Node<Ty> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = self.ty.format_label(self.kind.name(), self.span);
        self.kind.fmt_labeled(&label, f)
    }
}

/// Logs a single ANTLR visitor step with compact, single-line source text.
pub fn log_visit(target: &str, rule: &str, span: Span, text: &str) {
    const MAX_TEXT_LEN: usize = 60;
    let collapsed: String = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let preview = if collapsed.chars().count() > MAX_TEXT_LEN {
        let truncated: String = collapsed.chars().take(MAX_TEXT_LEN - 3).collect();
        format!("{truncated}...")
    } else {
        collapsed
    };
    log::trace!(
        target: target,
        "[visit] {rule:<20} span={}..{} text={preview:?}",
        span.start,
        span.end
    );
}
