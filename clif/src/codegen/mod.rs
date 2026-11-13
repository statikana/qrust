pub mod float;
pub mod integer;
pub mod matrix;
pub mod ptr;
pub mod boolean;
pub mod scope;
pub mod binary;
pub mod unary;
pub mod debug;
pub mod error;

use ast::nodes::{NodeKind, Span};
use cranelift_codegen::ir::{self, Function, InstBuilder};
use cranelift_codegen::ir::types::{I8, I32};
use cranelift_codegen::isa::CallConv;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use typed::typecheck::TypedNode;
use typed::types::Type;

pub use error::CodegenError;

/// represents rules for how an object must be treated in memory
#[derive(Debug)]
pub struct Layout {
    /// size of object in bits
    size_bits: usize,

    /// alignment of the memory address in bits
    align_bits: usize,
    pub clif_type: ir::Type,
}

/// info about target system
#[derive(Debug)]
pub struct Target {
    pub pointer_bits: usize,
    pub register_bits: usize,
    pub simd_register_bits: usize,
    pub page_width_bits: usize
}

/// memory rules for a type
pub trait HasLayout {
    /// the layout of the type on `target`. returns `None` if it has no layout or is too large for a stack (use dynamic)
    fn layout(&self, target: &Target) -> Option<Layout>;
}

impl HasLayout for Type {
    fn layout(&self, target: &Target) -> Option<Layout> {
        match self {
            Type::Unit => None,
            Type::Boolean => Some(Layout {
                size_bits: 1,
                align_bits: 1,
                clif_type: I8,
            }),
            Type::Integer(t) => t.layout(target),
            Type::Float(t) => t.layout(target),
            Type::Ptr(t) => t.layout(target),
            Type::Matrix(t) => t.layout(target),
        }
    }
}


pub struct FunctionCodegen<'a> {
    pub builder: FunctionBuilder<'a>,
    pub target: Target,
}

impl<'a> FunctionCodegen<'a> {
    pub fn new(function: &'a mut Function, fn_ctx: &'a mut FunctionBuilderContext) -> Self {
        let builder = FunctionBuilder::new(function, fn_ctx);

        // for my intel 12700H
        let target = Target {
            pointer_bits: 64,
            register_bits: 128,
            simd_register_bits: 256,
            page_width_bits: 4096 * 8
        };
        FunctionCodegen { builder, target }
    }

    /// inserts the `node`'s instructions into its own function, which when called, returns the value of the node.
    /// this function cannot take any parameters, and doesn't return a value if the `layout` of the node is None
    pub fn build_standalone(&mut self, node: &TypedNode, default_return_0: bool) {
        if let Err(err) = self.try_build_standalone(node, default_return_0) {
            panic!("CLIF codegen failed: {err}");
        }
    }

    /// fallible version of [`Codegen::build_standalone`] that logs and returns structured [`CodegenError`]s.
    pub fn try_build_standalone(&mut self, node: &TypedNode, default_return_0: bool) -> Result<(), CodegenError> {
        let block = self.builder.create_block();
            

        // first, determine return parameters
        let node_layout = node.ty.layout(&self.target);
        if node_layout.is_none() && node.ty != Type::Unit {
            let err = CodegenError::MissingLayout {
                context: "standalone return type",
                ty: node.ty.clone(),
                span: Some(node.span),
                reason: matrix::diagnose_type_layout(&node.ty, &self.target),
            };
            debug::log_error("clif::codegen", &err);
            return Err(err);
        }

        let has_return_value = if let Some(layout) = node_layout {
            let param = ir::AbiParam::new(layout.clif_type);
            self.builder.func.signature.returns.push(param);
            self.builder.append_block_params_for_function_returns(block);
            true
        } else {
            false
        };
        // otherwise, shouldn't have a return value

        self.builder.seal_block(block);
        self.builder.switch_to_block(block);
        let value = self.try_build_here(node)?;

        if has_return_value {
            // assume the function's type algebra and their actual instructions agree on the type of
            // the return value, if it exists at all
            let Some(ret_val) = value else {
                let err = CodegenError::MissingReturnValue {
                    node_kind: node.kind.name(),
                    ty: node.ty.clone(),
                    span: node.span,
                };
                debug::log_error("clif::codegen", &err);
                return Err(err);
            };
            self.builder.ins().return_(&[ret_val]);
        } else if default_return_0 {
            let default_return = self.builder.ins().iconst(I32, 0);
            self.builder.ins().return_(&[default_return]);
        } else {
            self.builder.ins().return_(&[]);
        }

        Ok(())
    }

    /// writes the instructions for `node` into the current block. don't use unless this is being called from another node's instructions (use `enter` instead)
    pub fn build_here(&mut self, node: &TypedNode) -> Option<ir::Value> {
        self.try_build_here(node).ok().flatten()
    }

    /// fallible version of [`Codegen::build_here`].
    pub fn try_build_here(&mut self, node: &TypedNode) -> Result<Option<ir::Value>, CodegenError> {
        let result = self.build_node_inner(node);
        debug::log_build_node(node, &result);
        result
    }

    pub(crate) fn require_child_value(
        &mut self,
        target: &'static str,
        parent_kind: &'static str,
        parent_span: Span,
        role: impl Into<String>,
        child: &TypedNode,
    ) -> Result<ir::Value, CodegenError> {
        let role = role.into();
        match self.try_build_here(child)? {
            Some(value) => Ok(value),
            None => {
                let err = CodegenError::MissingChildValue {
                    parent_kind,
                    parent_span,
                    role,
                    child_kind: child.kind.name(),
                    child_ty: child.ty.clone(),
                    child_span: child.span,
                };
                debug::log_error(target, &err);
                Err(err)
            }
        }
    }

    fn build_node_inner(&mut self, node: &TypedNode) -> Result<Option<ir::Value>, CodegenError> {
        // map node => instructions
        match &node.kind {
            NodeKind::Integer(value) => Ok(self.build_integer(&node.ty, *value)),
            NodeKind::Float(value) => Ok(self.build_float(&node.ty, *value)),
            NodeKind::Boolean(value) => Ok(self.build_boolean(&node.ty, *value)),

            NodeKind::BinaryOperation { op, lhs, rhs } => {
                self.try_build_binary(op, lhs, rhs, node.span)
            }
            NodeKind::UnaryOperation { op, subject } => {
                self.try_build_unary(op, subject, node.span)
            }

            NodeKind::Matrix { elements } => self.try_build_matrix(&node.ty, elements, node.span),
            NodeKind::Unit => Ok(None),

            NodeKind::Scope { statements } => self.try_build_scope(&node.ty, statements),
        }
    }

    pub fn finalize(self) {
        if let Err(err) = self.try_finalize() {
            panic!("CLIF finalize failed: {err}");
        }
    }

    /// fallible version of [`Codegen::finalize`].
    pub fn try_finalize(self) -> Result<(), CodegenError> {
        let lexicon_pointer_width = match self.target.pointer_bits {
            16 => target_lexicon::PointerWidth::U16,
            32 => target_lexicon::PointerWidth::U32,
            64 => target_lexicon::PointerWidth::U64,
            other => {
                let err = CodegenError::UnsupportedPointerWidth {
                    pointer_bits: other,
                    span: None,
                };
                debug::log_error("clif::codegen", &err);
                return Err(err);
            }
        };

        let frontend_config = cranelift_codegen::isa::TargetFrontendConfig {
            default_call_conv: CallConv::SystemV,
            pointer_width: lexicon_pointer_width,
            page_size_align_log2: f64::log2((self.target.page_width_bits * 8) as f64) as u8 // yeah we love this
        };

        self.builder.finalize(frontend_config);
        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use std::sync::{Mutex, OnceLock};

    use ast::nodes::{NodeKind, Span};
    use ast::operator::{BinaryOperator, UnaryOperator};
    use cranelift_codegen::ir::Function;
    use cranelift_frontend::FunctionBuilderContext;
    use log::{Level, LevelFilter, Log, Metadata, Record};
    use typed::typecheck::TypedNode;
    use typed::types::{FloatTypeSpec, IntegerTypeSpec, MatrixTypeSpec, Type};

    use super::{FunctionCodegen, CodegenError};

    struct RecordedLog {
        level: Level,
        target: String,
        message: String,
    }

    struct TestLogger {
        records: Mutex<Vec<RecordedLog>>,
    }

    impl Log for TestLogger {
        fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
            true
        }

        fn log(&self, record: &Record<'_>) {
            if let Ok(mut guard) = self.records.lock() {
                guard.push(RecordedLog {
                    level: record.level(),
                    target: record.target().to_string(),
                    message: record.args().to_string(),
                });
            }
        }

        fn flush(&self) {}
    }

    fn init_test_logger() -> &'static TestLogger {
        static LOGGER: OnceLock<&'static TestLogger> = OnceLock::new();
        *LOGGER.get_or_init(|| {
            let leaked: &'static TestLogger = Box::leak(Box::new(TestLogger {
                records: Mutex::new(Vec::new()),
            }));
            let _ = log::set_logger(leaked);
            log::set_max_level(LevelFilter::Trace);
            leaked
        })
    }

    fn with_captured_logs<T>(f: impl FnOnce() -> T) -> (T, Vec<RecordedLog>) {
        static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();
        let _guard = TEST_MUTEX.get_or_init(|| Mutex::new(())).lock().unwrap();
        let log_sink = init_test_logger();
        log_sink.records.lock().unwrap().clear();
        let result = f();
        let logs = log_sink.records.lock().unwrap().drain(..).collect();
        (result, logs)
    }

    fn span(start: u32, end: u32) -> Span {
        Span { start, end }
    }

    fn i64_ty() -> Type {
        Type::Integer(IntegerTypeSpec {
            signed: true,
            size_bytes: 8,
        })
    }

    fn f64_ty() -> Type {
        Type::Float(FloatTypeSpec { size_bytes: 8 })
    }

    fn typed_node(kind: NodeKind<Type>, ty: Type, s: Span) -> TypedNode {
        TypedNode { kind, span: s, ty }
    }

    #[test]
    fn unary_negate_inside_binary_succeeds() {
        let ast = typed_node(
            NodeKind::BinaryOperation {
                op: BinaryOperator::Add,
                lhs: Box::new(typed_node(NodeKind::Integer(10), i64_ty(), span(0, 2))),
                rhs: Box::new(typed_node(
                    NodeKind::UnaryOperation {
                        op: UnaryOperator::Negate,
                        subject: Box::new(typed_node(NodeKind::Integer(3), i64_ty(), span(6, 7))),
                    },
                    i64_ty(),
                    span(5, 7),
                )),
            },
            i64_ty(),
            span(0, 7),
        );

        let (res, logs) = with_captured_logs(|| {
            let mut func = Function::new();
            let mut ctx = FunctionBuilderContext::new();
            let mut codegen = FunctionCodegen::new(&mut func, &mut ctx);
            let r = codegen.try_build_standalone(&ast, false);
            codegen.try_finalize().unwrap();
            r
        });

        assert!(res.is_ok());
        assert!(!logs.iter().any(|l| l.level == Level::Error));
    }

    #[test]
    fn unsupported_binary_matrix_op_logs_error_with_context() {
        let mat_ty = Type::Matrix(MatrixTypeSpec {
            shape: vec![2],
            element_type: Box::new(f64_ty()),
        });
        let ast = typed_node(
            NodeKind::BinaryOperation {
                op: BinaryOperator::Add,
                lhs: Box::new(typed_node(
                    NodeKind::Matrix {
                        elements: vec![
                            typed_node(NodeKind::Float(1.0), f64_ty(), span(1, 4)),
                            typed_node(NodeKind::Float(2.0), f64_ty(), span(6, 9)),
                        ],
                    },
                    mat_ty.clone(),
                    span(0, 10),
                )),
                rhs: Box::new(typed_node(NodeKind::Float(3.0), f64_ty(), span(13, 16))),
            },
            mat_ty,
            span(0, 16),
        );

        let (res, logs) = with_captured_logs(|| {
            let mut func = Function::new();
            let mut ctx = FunctionBuilderContext::new();
            let mut codegen = FunctionCodegen::new(&mut func, &mut ctx);
            codegen.try_build_standalone(&ast, false)
        });

        assert!(matches!(
            res,
            Err(CodegenError::UnsupportedOperationType { op: "Add", .. })
        ));
        let errors: Vec<_> = logs.iter().filter(|l| l.level == Level::Error).collect();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].target, "clif::codegen::binary");
        assert!(errors[0].message.contains("[2]f64 + f64"));
        assert!(errors[0].message.contains("0..16"));
    }

    #[test]
    fn missing_child_value_logs_error_with_roles_and_spans() {
        let ast = typed_node(
            NodeKind::BinaryOperation {
                op: BinaryOperator::Add,
                lhs: Box::new(typed_node(NodeKind::Unit, Type::Unit, span(0, 2))),
                rhs: Box::new(typed_node(NodeKind::Integer(1), i64_ty(), span(5, 6))),
            },
            i64_ty(),
            span(0, 6),
        );

        let (res, logs) = with_captured_logs(|| {
            let mut func = Function::new();
            let mut ctx = FunctionBuilderContext::new();
            let mut codegen = FunctionCodegen::new(&mut func, &mut ctx);
            codegen.try_build_standalone(&ast, false)
        });

        assert!(matches!(
            res,
            Err(CodegenError::MissingChildValue {
                parent_kind: "BinaryOperation",
                ..
            })
        ));
        let errors: Vec<_> = logs.iter().filter(|l| l.level == Level::Error).collect();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].target, "clif::codegen::binary");
        assert!(errors[0].message.contains("lhs"));
        assert!(errors[0].message.contains("0..2"));
        assert!(errors[0].message.contains("0..6"));
    }

    #[test]
    fn invalid_type_algebra_in_unary_logs_error() {
        let ast = typed_node(
            NodeKind::UnaryOperation {
                op: UnaryOperator::Negate,
                subject: Box::new(typed_node(NodeKind::Boolean(true), Type::Boolean, span(1, 5))),
            },
            Type::Boolean,
            span(0, 5),
        );

        let (res, logs) = with_captured_logs(|| {
            let mut func = Function::new();
            let mut ctx = FunctionBuilderContext::new();
            let mut codegen = FunctionCodegen::new(&mut func, &mut ctx);
            codegen.try_build_standalone(&ast, false)
        });

        assert!(matches!(res, Err(CodegenError::InvalidTypeAlgebra { .. })));
        let errors: Vec<_> = logs.iter().filter(|l| l.level == Level::Error).collect();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].target, "clif::codegen::unary");
        assert!(errors[0].message.contains("-bool"));
        assert!(errors[0].message.contains("0..5"));
    }

    #[test]
    fn matrix_with_missing_layout_diagnoses_reason() {
        let bad_mat_ty = Type::Matrix(MatrixTypeSpec {
            shape: vec![1],
            element_type: Box::new(Type::Unit),
        });
        let ast = typed_node(
            NodeKind::Scope {
                statements: vec![typed_node(
                    NodeKind::Matrix {
                        elements: vec![typed_node(NodeKind::Unit, Type::Unit, span(1, 3))],
                    },
                    bad_mat_ty,
                    span(0, 4),
                )],
            },
            Type::Unit,
            span(0, 4),
        );

        let (res, logs) = with_captured_logs(|| {
            let mut func = Function::new();
            let mut ctx = FunctionBuilderContext::new();
            let mut codegen = FunctionCodegen::new(&mut func, &mut ctx);
            codegen.try_build_standalone(&ast, false)
        });

        assert!(matches!(res, Err(CodegenError::MissingLayout { .. })));
        let errors: Vec<_> = logs.iter().filter(|l| l.level == Level::Error).collect();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].target, "clif::codegen::matrix");
        assert!(errors[0].message.contains("unit type has no value representation"));
    }

    #[test]
    fn unsupported_pointer_width_in_finalize_logs_error() {
        let (res, logs) = with_captured_logs(|| {
            let mut func = Function::new();
            let mut ctx = FunctionBuilderContext::new();
            let mut codegen = FunctionCodegen::new(&mut func, &mut ctx);
            codegen.target.pointer_bits = 128;
            codegen.try_finalize()
        });

        assert!(matches!(
            res,
            Err(CodegenError::UnsupportedPointerWidth {
                pointer_bits: 128,
                ..
            })
        ));
        let errors: Vec<_> = logs.iter().filter(|l| l.level == Level::Error).collect();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].target, "clif::codegen");
        assert!(errors[0].message.contains("128"));
    }
}