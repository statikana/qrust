use cranelift_codegen::ir;
use typed::{typecheck::TypedNode, types::Type};

use crate::codegen::{CodegenError, FunctionCodegen, debug};

impl FunctionCodegen<'_> {
    pub(crate) fn try_build_scope(
        &mut self,
        _ty: &Type,
        statements: &[TypedNode],
    ) -> Result<Option<ir::Value>, CodegenError> {
        debug::log_scope_block(self.builder.current_block(), statements.len());
        let mut last: Option<ir::Value> = None;
        for (index, statement) in statements.iter().enumerate() {
            debug::log_scope_statement(index, statement);
            last = self.try_build_here(statement)?;
        }
        Ok(last)
    }
}