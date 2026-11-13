use cranelift_codegen::ir::{self, InstBuilder, types::I8};
use typed::types::Type;

use crate::codegen::FunctionCodegen;

// boolean layout is constant, so we don't need to implement HasLayout (there is no defined BooleanType)
impl FunctionCodegen<'_> {
    pub(crate) fn build_boolean(&mut self, _ty: &Type, value: bool) -> Option<ir::Value> {
        Some(self.builder.ins().iconst(I8, value as i64))
    }
}