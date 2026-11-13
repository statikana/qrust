use std::rc::Rc;

use antlr_rust::token::Token;
use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::tree::ParseTree;
use parse::qrustparser::*;
use parse::qrustvisitor::*;

use crate::nodes::{UntypedNode, NodeKind, Span};
use crate::operator::{BinaryOperator, UnaryOperator};

type R = Box<UntypedNode>;

fn get_span<'input>(ctx: &impl QrustParserContext<'input>) -> Span {
    Span {
        start: ctx.start().get_start() as u32,
        end: ctx.stop().get_stop() as u32 + 1,
    }
}
// emits a trace log for the rule being visited, its span, and its source text
macro_rules! trace_visit {
    ($name:expr, $ctx:expr) => {
        if ::log::log_enabled!(::log::Level::Trace) {
            $crate::ast_debug::log_visit(module_path!(), $name, get_span($ctx), &$ctx.get_text());
        }
    };
}

impl Default for R {
    fn default() -> Self {
        Box::new(UntypedNode::new(
            NodeKind::Integer(69),
            Span { start: 0, end: 1 },
        ))
    }
}

pub struct ASTVisitor {
    pub temp_result: R, // stores the result of the most recent visit, instead of having functions do regular returns. used by antlr4rust, not me.
}
impl ASTVisitor {
    fn get<'input>(&mut self, x: &Option<Rc<impl QrustParserContext<'input>>>) -> R {
        self.visit(x.as_ref().unwrap().as_ref())
    }

    fn binary_op_node<'input, C: QrustParserContext<'input>>(
        &mut self,
        ctx: &C,
        lhs: R,
        rhs: R,
        parse_op: impl FnOnce(&str) -> Option<BinaryOperator>,
    ) -> R {
        let span = get_span(ctx);
        let op_text = ctx
            .get_child(1)
            .map(|c| c.get_text())
            .unwrap_or_else(|| {
                log::error!("Missing operator token at span {}..{}", span.start, span.end);
                panic!("Missing operator token");
            });
        let op = parse_op(op_text.as_str()).unwrap_or_else(|| {
            log::error!(
                "Unexpected binary operator {:?} at span {}..{}",
                op_text,
                span.start,
                span.end
            );
            panic!("Unexpected operator {op_text:?}");
        });
        R::new(UntypedNode::new(
            NodeKind::BinaryOperation { op, lhs, rhs },
            span,
        ))
    }
}

impl<'input> ParseTreeVisitorCompat<'input> for ASTVisitor {
    type Return = R; // each function returns this value
    type Node = QrustParserContextType; // each node is represented as this
    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.temp_result
    }
}

impl<'input> QrustVisitorCompat<'input> for ASTVisitor {
    fn visit_scope(&mut self, ctx: &ScopeContext<'input>) -> Self::Return {
        trace_visit!("visit_scope", ctx);
        let mut results = vec![];
        for expr in ctx.expr_all() {
            let result = *self.visit(expr.as_ref());
            results.push(result);
        }
        R::new(UntypedNode::new(
            NodeKind::Scope {
                statements: results,
            },
            get_span(ctx),
        ))
    }

    fn visit_expr_out(&mut self, ctx: &Expr_outContext<'input>) -> Self::Return {
        trace_visit!("visit_expr_out", ctx);
        let node = self.get(&ctx.term());
        log::info!("out -> {:?}", *node);
        R::new(UntypedNode::new(NodeKind::Unit, get_span(ctx)))
    }

    fn visit_expr_term(&mut self, ctx: &Expr_termContext<'input>) -> Self::Return {
        trace_visit!("visit_expr_term", ctx);
        self.get(&ctx.term()) // relinquish
    }

    fn visit_term_literal(&mut self, ctx: &Term_literalContext<'input>) -> Self::Return {
        trace_visit!("visit_term_literal", ctx);
        self.get(&ctx.literal()) // relinquish
    }

    fn visit_term_mul_div(&mut self, ctx: &Term_mul_divContext<'input>) -> Self::Return {
        trace_visit!("visit_term_mul_div", ctx);
        let lhs = self.get(&ctx.lhs);
        let rhs = self.get(&ctx.rhs);
        self.binary_op_node(ctx, lhs, rhs, |s| match s {
            "*" => Some(BinaryOperator::Mul),
            "/" => Some(BinaryOperator::Div),
            _ => None,
        })
    }

    fn visit_term_add_sub(&mut self, ctx: &Term_add_subContext<'input>) -> Self::Return {
        trace_visit!("visit_term_add_sub", ctx);
        let lhs = self.get(&ctx.lhs);
        let rhs = self.get(&ctx.rhs);
        self.binary_op_node(ctx, lhs, rhs, |s| match s {
            "+" => Some(BinaryOperator::Add),
            "-" => Some(BinaryOperator::Sub),
            _ => None,
        })
    }

    fn visit_term_paren(&mut self, ctx: &Term_parenContext<'input>) -> Self::Return {
        trace_visit!("visit_term_paren", ctx);
        self.get(&ctx.term()) // relinquish
    }

    fn visit_term_neg(&mut self, ctx: &Term_negContext<'input>) -> Self::Return {
        trace_visit!("visit_term_neg", ctx);
        let subject = self.get(&ctx.term());
        R::new(UntypedNode::new(
            NodeKind::UnaryOperation {
                op: UnaryOperator::Negate,
                subject,
            },
            get_span(ctx),
        ))
    }

    fn visit_value_boolean(&mut self, ctx: &Value_booleanContext<'input>) -> Self::Return {
        trace_visit!("visit_value_boolean", ctx);
        self.get(&ctx.boolean()) // relinqish
    }

    fn visit_value_numeric(&mut self, ctx: &Value_numericContext<'input>) -> Self::Return {
        trace_visit!("visit_value_numeric", ctx);
        self.get(&ctx.numeric()) // relinqish
    }

    fn visit_boolean_true(&mut self, ctx: &Boolean_trueContext<'input>) -> Self::Return {
        trace_visit!("visit_boolean_true", ctx);
        R::new(UntypedNode::new(NodeKind::Boolean(true), get_span(ctx)))
    }

    fn visit_boolean_false(&mut self, ctx: &Boolean_falseContext<'input>) -> Self::Return {
        trace_visit!("visit_boolean_false", ctx);
        R::new(UntypedNode::new(NodeKind::Boolean(false), get_span(ctx)))
    }

    fn visit_numeric_integer(&mut self, ctx: &Numeric_integerContext<'input>) -> Self::Return {
        trace_visit!("visit_numeric_integer", ctx);
        type IntT = i64; // uhh seems odd
        let span = get_span(ctx);
        let text = ctx.get_text();
        let value = text.parse::<IntT>().unwrap_or_else(|err| {
            log::error!("Invalid integer literal {text:?} at {}..{}: {err}", span.start, span.end);
            panic!("Invalid integer literal {text:?}: {err}");
        });
        R::new(UntypedNode::new(NodeKind::Integer(value), span))
    }

    fn visit_numeric_real(&mut self, ctx: &Numeric_realContext<'input>) -> Self::Return {
        trace_visit!("visit_numeric_real", ctx);
        type FloatT = f64; // ...
        let span = get_span(ctx);
        let text = ctx.get_text();
        let value = text.parse::<FloatT>().unwrap_or_else(|err| {
            log::error!("Invalid float literal {text:?} at {}..{}: {err}", span.start, span.end);
            panic!("Invalid float literal {text:?}: {err}");
        });
        R::new(UntypedNode::new(NodeKind::Float(value), span))
    }

    fn visit_value_matrix(&mut self, ctx: &Value_matrixContext<'input>) -> Self::Return {
        trace_visit!("visit_value_matrix", ctx);
        self.get(&ctx.matrix())
    }

    fn visit_matrix(&mut self, ctx: &MatrixContext<'input>) -> Self::Return {
        trace_visit!("visit_matrix", ctx);
        // the typechecker for matrix handles collapsing matrices
        let elements: Vec<UntypedNode> = ctx
            .term_all()
            .iter()
            .map(|elem| *self.visit(elem.as_ref()))
            .collect();
        R::new(UntypedNode::new(NodeKind::Matrix { elements }, get_span(ctx)))
    }
}
