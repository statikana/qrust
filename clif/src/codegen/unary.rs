use ast::nodes::Span;
use ast::operator::UnaryOperator::{self, Negate};
use cranelift_codegen::ir::{self, InstBuilder};
use typed::{typecheck::TypedNode, types::{Type, TypeAlgebra}};

use crate::codegen::{FunctionCodegen, CodegenError, debug};

impl FunctionCodegen<'_> {
    pub(crate) fn try_build_unary(
        &mut self,
        op: &UnaryOperator,
        subject: &TypedNode,
        span: Span,
    ) -> Result<Option<ir::Value>, CodegenError> {
        let subject_value =
            self.require_child_value("clif::codegen::unary", "UnaryOperation", span, "subject", subject)?;

        let result_type = match op {
            Negate => subject.ty.negate(),
        }
        .ok_or_else(|| {
            let err = CodegenError::InvalidTypeAlgebra {
                op: op.symbol(),
                operands: format!("{}{}", op.symbol(), subject.ty),
                span,
            };
            debug::log_error("clif::codegen::unary", &err);
            err
        })?;

        let out = match (op, &result_type) {
            (Negate, Type::Integer(_)) => self.builder.ins().ineg(subject_value),
            (Negate, Type::Float(_)) => self.builder.ins().fneg(subject_value),
            _ => {
                let err = CodegenError::UnsupportedOperationType {
                    op: op.name(),
                    expr: format!("{}{}", op.symbol(), subject.ty),
                    result_ty: result_type,
                    span,
                };
                debug::log_error("clif::codegen::unary", &err);
                return Err(err);
            }
        };

        Ok(Some(out))
    }
}