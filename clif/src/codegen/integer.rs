use cranelift_codegen::ir::{self, InstBuilder, types::I64};
use typed::types::{IntegerTypeSpec, Type};

use crate::codegen::FunctionCodegen;

use super::{HasLayout, Layout, Target};

impl HasLayout for IntegerTypeSpec {
    fn layout(&self, _target: &Target) -> Option<Layout> {
        Some(Layout {
            size_bits: 64,
            align_bits: 64,
            clif_type: I64,
        })
    }
}

impl FunctionCodegen<'_> {
    pub(crate) fn build_integer(&mut self, _ty: &Type, value: i64) -> Option<ir::Value> {
        Some(self.builder.ins().iconst(I64, value))
    }
}