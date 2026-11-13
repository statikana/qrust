use ast::nodes::{NodeKind, Span};
use cranelift_codegen::ir::{self, InstBuilder, StackSlotData, immediates::Offset32};
use typed::{typecheck::TypedNode, types::{MatrixTypeSpec, Type}};

use crate::codegen::{Codegen, CodegenError};

use super::{HasLayout, Layout, Target, debug};

impl HasLayout for MatrixTypeSpec {
    fn layout(&self, target: &Target) -> Option<Layout> {
        let element_layout = self.element_type.layout(target)?;

        let lane_count = vector_lane_count(target, &element_layout)?;

        // just the number of elements in the matrix
        let element_count = self.shape.iter().fold(1, |cur, dim_size| cur * dim_size);

        // padded number of elements such that it fits into the number
        // of lanes for this matrix
        let padded_element_count = element_count.next_multiple_of(lane_count);

        // e.g. I8.by(32) => I8X32
        let vector_clif_type = element_layout.clif_type.by(lane_count.try_into().ok()?)?;
        let size_bits = padded_element_count * element_layout.size_bits;
        let align_bits = element_layout.align_bits;

        Some(Layout {
            size_bits,
            align_bits,
            clif_type: vector_clif_type,
        })
    }
}

fn vector_lane_count(target: &Target, element_layout: &Layout) -> Option<usize> {
    let element_bits = element_layout.clif_type.bits();
    target
        .simd_register_bits
        .checked_div(element_bits.try_into().ok()?)
        .filter(|&n| n > 1)
}

/// Explains why `ty.layout(target)` returned `None` for error reporting.
pub(crate) fn diagnose_type_layout(ty: &Type, target: &Target) -> &'static str {
    match ty {
        Type::Unit => "unit type has no value representation",
        Type::Matrix(spec) => {
            let Some(element_layout) = spec.element_type.layout(target) else {
                return diagnose_type_layout(&spec.element_type, target);
            };
            let element_bits = element_layout.clif_type.bits();
            let Ok(element_bits_usize) = usize::try_from(element_bits) else {
                return "element CLIF bit width exceeds usize";
            };
            let Some(lane_count) = target
                .simd_register_bits
                .checked_div(element_bits_usize)
                .filter(|&n| n > 1)
            else {
                return "element type cannot pack multiple lanes into SIMD register";
            };
            let Ok(lane_count_u32) = u32::try_from(lane_count) else {
                return "SIMD lane count exceeds u32";
            };
            if element_layout.clif_type.by(lane_count_u32).is_none() {
                return "Cranelift does not support vector type for computed lane count";
            }
            "unknown matrix layout failure"
        }
        _ => "type layout returned None",
    }
}

impl Codegen<'_> {
    pub(crate) fn try_build_matrix(
        &mut self,
        ty: &Type,
        elements: &[TypedNode],
        span: Span,
    ) -> Result<Option<ir::Value>, CodegenError> {
        // 1. create a stack slot of the total size (slot = create_sized_stack_slot)
        // 2. store elements in row-major order, should fit within the entire stack slot (for each element, stack_store(slot, ...))
        // 3. return pointer to the beginning of the stack slot (stack_addr(pointer_clif_type, slot, offset=0))

        let Type::Matrix(MatrixTypeSpec { shape: _, element_type }) = ty else {
            let err = CodegenError::UnexpectedMatrixType {
                actual_ty: ty.clone(),
                span,
            };
            debug::log_error("clif::codegen::matrix", &err);
            return Err(err);
        };

        let layout = ty.layout(&self.target).ok_or_else(|| {
            let err = CodegenError::MissingLayout {
                context: "matrix",
                ty: ty.clone(),
                span: Some(span),
                reason: diagnose_type_layout(ty, &self.target),
            };
            debug::log_error("clif::codegen::matrix", &err);
            err
        })?;

        let pointer_clif_type = u16::try_from(self.target.pointer_bits)
            .ok()
            .and_then(ir::Type::int)
            .ok_or_else(|| {
                let err = CodegenError::UnsupportedPointerWidth {
                    pointer_bits: self.target.pointer_bits,
                    span: Some(span),
                };
                debug::log_error("clif::codegen::matrix", &err);
                err
            })?;

        let element_layout = element_type.layout(&self.target).ok_or_else(|| {
            let err = CodegenError::MissingLayout {
                context: "matrix element",
                ty: (**element_type).clone(),
                span: Some(span),
                reason: diagnose_type_layout(element_type, &self.target),
            };
            debug::log_error("clif::codegen::matrix", &err);
            err
        })?;
        // we don't actually care about the shape; all the info we need is in layout and element_type
        let element_size_bytes = element_layout.size_bits / 8;

        let slot_size_bytes = u32::try_from(layout.size_bits / 8).map_err(|_| {
            let err = CodegenError::MissingLayout {
                context: "matrix stack slot",
                ty: ty.clone(),
                span: Some(span),
                reason: "matrix byte size exceeds u32 stack slot limit",
            };
            debug::log_error("clif::codegen::matrix", &err);
            err
        })?;
        let align_bytes = (layout.align_bits / 8).max(1);
        let align_shift = align_bytes.trailing_zeros() as u8; // log2 of the alignment in bytes
        let slot_data = StackSlotData::new(ir::StackSlotKind::ExplicitSlot, slot_size_bytes, align_shift);
        let slot = self.builder.create_sized_stack_slot(slot_data);

        let mut flat_elements: Vec<&TypedNode> = vec![];
        flatten_matrix(elements, &mut flat_elements);
        debug::log_matrix_slot(ty, flat_elements.len(), slot_size_bytes, align_shift, slot);
        for (i, element) in flat_elements.iter().enumerate() {
            let value = self.require_child_value(
                "clif::codegen::matrix",
                "Matrix",
                span,
                format!("element[{i}]"),
                element,
            )?;
            // we store it in the same slot, with the offset
            let offset_bytes = i32::try_from(element_size_bytes * i).map_err(|_| {
                let err = CodegenError::MissingLayout {
                    context: "matrix element offset",
                    ty: ty.clone(),
                    span: Some(element.span),
                    reason: "element byte offset exceeds i32 Offset32 limit",
                };
                debug::log_error("clif::codegen::matrix", &err);
                err
            })?;
            self.builder.ins().stack_store(pointer_clif_type, value, slot, Offset32::new(offset_bytes));
        }

        // offset = 0 -> beginning
        Ok(Some(self.builder.ins().stack_addr(pointer_clif_type, slot, 0)))
    }
}

fn flatten_matrix<'a>(elements: &'a [TypedNode], out: &mut Vec<&'a TypedNode>) {
    for element in elements {
        match &element.kind {
            NodeKind::Matrix{ elements } => flatten_matrix(elements, out),
            _ => out.push(element)
        }
    }
}