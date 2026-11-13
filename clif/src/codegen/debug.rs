use cranelift_codegen::ir;
use typed::typecheck::TypedNode;
use typed::types::Type;

use super::error::CodegenError;

// Logging helpers for CLIF codegen, kept separate from instruction emission so codegen stays readable.

pub(crate) fn log_error(target: &str, err: &CodegenError) {
    log::error!(target: target, "{err}");
}

pub(crate) fn log_build_node(
    node: &TypedNode,
    result: &Result<Option<ir::Value>, CodegenError>,
) {
    match result {
        Ok(value) => {
            if value.is_none() && node.ty != Type::Unit {
                log::warn!(
                    target: "clif::codegen",
                    "[clif] {:<16} span={}..{} ty={} produced None for non-unit type",
                    node.kind.name(),
                    node.span.start,
                    node.span.end,
                    node.ty
                );
            } else {
                log::trace!(
                    target: "clif::codegen",
                    "[clif] {:<16} span={}..{} ty={} -> {:?}",
                    node.kind.name(),
                    node.span.start,
                    node.span.end,
                    node.ty,
                    value
                );
            }
        }
        Err(err) => {
            log::debug!(
                target: "clif::codegen",
                "[clif] {:<16} span={}..{} ty={} failed: {err}",
                node.kind.name(),
                node.span.start,
                node.span.end,
                node.ty
            );
        }
    }
}

pub(crate) fn log_matrix_slot(
    ty: &Type,
    element_count: usize,
    slot_size_bytes: u32,
    align_shift: u8,
    slot: ir::StackSlot,
) {
    log::debug!(
        target: "clif::codegen::matrix",
        "[matrix] ty={ty} elems={element_count} slot={slot:?} size={slot_size_bytes}B align_shift={align_shift}"
    );
}

pub(crate) fn log_scope_block(block: Option<ir::Block>, statement_count: usize) {
    log::trace!(
        target: "clif::codegen::scope",
        "[scope] block={block:?} ({statement_count} statements)"
    );
}

pub(crate) fn log_scope_statement(index: usize, statement: &TypedNode) {
    log::trace!(
        target: "clif::codegen::scope",
        "[scope] stmt[{index}] {:<16} span={}..{} ty={}",
        statement.kind.name(),
        statement.span.start,
        statement.span.end,
        statement.ty
    );
}
