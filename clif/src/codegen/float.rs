use cranelift_codegen::ir::{self, InstBuilder, types::F64};
use typed::types::{FloatTypeSpec, Type};

use crate::codegen::Codegen;

use super::{HasLayout, Layout, Target};

impl HasLayout for FloatTypeSpec {
    fn layout(&self, _target: &Target) -> Option<Layout> {
        Some(Layout {
            size_bits: 64,
            align_bits: 64,
            clif_type: F64,
        })
    }
}

impl Codegen<'_> {
    pub(crate) fn build_float(&mut self, _ty: &Type, value: f64) -> Option<ir::Value> {
        Some(self.builder.ins().f64const(value))
    }
}