use super::{HasLayout, Layout, Target};
use cranelift_codegen::ir::types::I8;
use typed::types::PtrTypeSpec;

impl HasLayout for PtrTypeSpec {
    fn layout(&self, target: &Target) -> Option<Layout> {
        Some(Layout {
            size_bits: target.pointer_bits,
            align_bits: target.pointer_bits,
            clif_type: I8, // or I4, but doesn't exist
        })
    }
}
