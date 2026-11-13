use ast::nodes::Span;
use ast::operator::BinaryOperator;
use cranelift_codegen::ir::{self, InstBuilder};
use typed::{typecheck::TypedNode, types::{Type, TypeAlgebra}};

use crate::codegen::{Codegen, CodegenError, debug};

impl Codegen<'_> {
    // TODO: this currently bases the operation based on the output (i.e. if the output is an integer, lhs and rhs must both be intergers).
    // not sure about this validity.

    //TODO: matrix operations
    pub(crate) fn try_build_binary(
        &mut self,
        op: &BinaryOperator,
        lhs: &TypedNode,
        rhs: &TypedNode,
        span: Span,
    ) -> Result<Option<ir::Value>, CodegenError> {
        let lhs_value =
            self.require_child_value("clif::codegen::binary", "BinaryOperation", span, "lhs", lhs)?;
        let rhs_value =
            self.require_child_value("clif::codegen::binary", "BinaryOperation", span, "rhs", rhs)?;

        let return_type = match op {
            BinaryOperator::Add => lhs.ty.add(&rhs.ty),
            BinaryOperator::Sub => lhs.ty.sub(&rhs.ty),
            BinaryOperator::Mul => lhs.ty.mul(&rhs.ty),
            BinaryOperator::Div => lhs.ty.div(&rhs.ty),
        }
        .ok_or_else(|| {
            let err = CodegenError::InvalidTypeAlgebra {
                op: op.symbol(),
                operands: format!("{} {} {}", lhs.ty, op.symbol(), rhs.ty),
                span,
            };
            debug::log_error("clif::codegen::binary", &err);
            err
        })?;

        let out = match (op, &return_type) {
            (BinaryOperator::Add, Type::Integer(_)) => self.builder.ins().iadd(lhs_value, rhs_value),
            (BinaryOperator::Add, Type::Float(_)) => self.builder.ins().fadd(lhs_value, rhs_value),
            (BinaryOperator::Sub, Type::Integer(_)) => self.builder.ins().isub(lhs_value, rhs_value),
            (BinaryOperator::Sub, Type::Float(_)) => self.builder.ins().fsub(lhs_value, rhs_value),
            (BinaryOperator::Mul, Type::Integer(_)) => self.builder.ins().imul(lhs_value, rhs_value),
            (BinaryOperator::Mul, Type::Float(_)) => self.builder.ins().fmul(lhs_value, rhs_value),
            (BinaryOperator::Div, Type::Integer(_)) => self.builder.ins().sdiv(lhs_value, rhs_value),
            (BinaryOperator::Div, Type::Float(_)) => self.builder.ins().fdiv(lhs_value, rhs_value),
            _ => {
                let err = CodegenError::UnsupportedOperationType {
                    op: op.name(),
                    expr: format!("{} {} {}", lhs.ty, op.symbol(), rhs.ty),
                    result_ty: return_type,
                    span,
                };
                debug::log_error("clif::codegen::binary", &err);
                return Err(err);
            }
        };

        Ok(Some(out))
    }
}