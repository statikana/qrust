// Generated from Qrust.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use super::qrustlistener::*;
use super::qrustvisitor::*;
use antlr_rust::PredictionContextCache;
use antlr_rust::TokenSource;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::error_strategy::{DefaultErrorStrategy, ErrorStrategy};
use antlr_rust::errors::*;
use antlr_rust::int_stream::EOF;
use antlr_rust::parser::{BaseParser, Parser, ParserNodeType, ParserRecog};
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext, cast, cast_mut};
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::token::{OwningToken, TOKEN_EOF, Token};
use antlr_rust::token_factory::{CommonTokenFactory, TokenAware, TokenFactory};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::*;
use antlr_rust::vocabulary::{Vocabulary, VocabularyImpl};

use antlr_rust::lazy_static;
use antlr_rust::{TidAble, TidExt};

use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub const T__0: isize = 1;
pub const T__1: isize = 2;
pub const T__2: isize = 3;
pub const T__3: isize = 4;
pub const T__4: isize = 5;
pub const T__5: isize = 6;
pub const T__6: isize = 7;
pub const TRUE: isize = 8;
pub const FALSE: isize = 9;
pub const INTEGER: isize = 10;
pub const REAL: isize = 11;
pub const MUL: isize = 12;
pub const DIV: isize = 13;
pub const ADD: isize = 14;
pub const SUB: isize = 15;
pub const PAREN_L: isize = 16;
pub const PAREN_R: isize = 17;
pub const WS: isize = 18;
pub const RULE_scope: usize = 0;
pub const RULE_expr: usize = 1;
pub const RULE_term: usize = 2;
pub const RULE_literal: usize = 3;
pub const RULE_boolean: usize = 4;
pub const RULE_numeric: usize = 5;
pub const RULE_matrix: usize = 6;
pub const ruleNames: [&'static str; 7] = [
    "scope", "expr", "term", "literal", "boolean", "numeric", "matrix",
];

pub const _LITERAL_NAMES: [Option<&'static str>; 18] = [
    None,
    Some("'{'"),
    Some("';'"),
    Some("'}'"),
    Some("'out'"),
    Some("'['"),
    Some("','"),
    Some("']'"),
    Some("'true'"),
    Some("'false'"),
    None,
    None,
    Some("'*'"),
    Some("'/'"),
    Some("'+'"),
    Some("'-'"),
    Some("'('"),
    Some("')'"),
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 19] = [
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some("TRUE"),
    Some("FALSE"),
    Some("INTEGER"),
    Some("REAL"),
    Some("MUL"),
    Some("DIV"),
    Some("ADD"),
    Some("SUB"),
    Some("PAREN_L"),
    Some("PAREN_R"),
    Some("WS"),
];
lazy_static! {
    static ref _shared_context_cache: Arc<PredictionContextCache> =
        Arc::new(PredictionContextCache::new());
    static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(
        _LITERAL_NAMES.iter(),
        _SYMBOLIC_NAMES.iter(),
        None
    ));
}

type BaseParserType<'input, I> = BaseParser<
    'input,
    QrustParserExt<'input>,
    I,
    QrustParserContextType,
    dyn QrustListener<'input> + 'input,
>;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type QrustTreeWalker<'input, 'a> =
    ParseTreeWalker<'input, 'a, QrustParserContextType, dyn QrustListener<'input> + 'a>;

/// Parser for Qrust grammar
pub struct QrustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    base: BaseParserType<'input, I>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> QrustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_serialized_atn() -> &'static str {
        _serializedATN
    }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
        antlr_rust::recognizer::check_version("0", "3");
        let interpreter = Arc::new(ParserATNSimulator::new(
            _ATN.clone(),
            _decision_to_DFA.clone(),
            _shared_context_cache.clone(),
        ));
        Self {
            base: BaseParser::new_base_parser(
                input,
                Arc::clone(&interpreter),
                QrustParserExt {
                    _pd: Default::default(),
                },
            ),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }
}

type DynStrategy<'input, I> = Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>> + 'input>;

impl<'input, I> QrustParser<'input, I, DynStrategy<'input, I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self {
        Self::with_strategy(input, Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> QrustParser<'input, I, DefaultErrorStrategy<'input, QrustParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn new(input: I) -> Self {
        Self::with_strategy(input, DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for QrustParser
pub trait QrustParserContext<'input>:
    for<'x> Listenable<dyn QrustListener<'input> + 'x>
    + for<'x> Visitable<dyn QrustVisitor<'input> + 'x>
    + ParserRuleContext<'input, TF = LocalTokenFactory<'input>, Ctx = QrustParserContextType>
{
}

antlr_rust::coerce_from! { 'input : QrustParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn QrustParserContext<'input> + 'input
where
    T: QrustVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn QrustVisitor<'input> + 'x))
    }
}

impl<'input> QrustParserContext<'input> for TerminalNode<'input, QrustParserContextType> {}
impl<'input> QrustParserContext<'input> for ErrorNode<'input, QrustParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn QrustParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn QrustListener<'input> + 'input }

pub struct QrustParserContextType;
antlr_rust::tid! {QrustParserContextType}

impl<'input> ParserNodeType<'input> for QrustParserContextType {
    type TF = LocalTokenFactory<'input>;
    type Type = dyn QrustParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for QrustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    type Target = BaseParserType<'input, I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for QrustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct QrustParserExt<'input> {
    _pd: PhantomData<&'input str>,
}

impl<'input> QrustParserExt<'input> {}
antlr_rust::tid! { QrustParserExt<'a> }

impl<'input> TokenAware<'input> for QrustParserExt<'input> {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    ParserRecog<'input, BaseParserType<'input, I>> for QrustParserExt<'input>
{
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    Actions<'input, BaseParserType<'input, I>> for QrustParserExt<'input>
{
    fn get_grammar_file_name(&self) -> &str {
        "Qrust.g4"
    }

    fn get_rule_names(&self) -> &[&str] {
        &ruleNames
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &**VOCABULARY
    }
    fn sempred(
        _localctx: Option<&(dyn QrustParserContext<'input> + 'input)>,
        rule_index: isize,
        pred_index: isize,
        recog: &mut BaseParserType<'input, I>,
    ) -> bool {
        match rule_index {
            2 => QrustParser::<'input, I, _>::term_sempred(
                _localctx.and_then(|x| x.downcast_ref()),
                pred_index,
                recog,
            ),
            _ => true,
        }
    }
}

impl<'input, I> QrustParser<'input, I, DefaultErrorStrategy<'input, QrustParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    fn term_sempred(
        _localctx: Option<&TermContext<'input>>,
        pred_index: isize,
        recog: &mut <Self as Deref>::Target,
    ) -> bool {
        match pred_index {
            0 => recog.precpred(None, 4),
            1 => recog.precpred(None, 3),
            _ => true,
        }
    }
}
//------------------- scope ----------------
pub type ScopeContextAll<'input> = ScopeContext<'input>;

pub type ScopeContext<'input> = BaseParserRuleContext<'input, ScopeContextExt<'input>>;

#[derive(Clone)]
pub struct ScopeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> QrustParserContext<'input> for ScopeContext<'input> {}

impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for ScopeContext<'input> {
    fn enter(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_scope(self);
    }
    fn exit(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.exit_scope(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for ScopeContext<'input> {
    fn accept(&self, visitor: &mut (dyn QrustVisitor<'input> + 'a)) {
        visitor.visit_scope(self);
    }
}

impl<'input> CustomRuleContext<'input> for ScopeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = QrustParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_scope
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_scope }
}
antlr_rust::tid! {ScopeContextExt<'a>}

impl<'input> ScopeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn QrustParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ScopeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ScopeContextExt { ph: PhantomData },
        ))
    }
}

pub trait ScopeContextAttrs<'input>:
    QrustParserContext<'input> + BorrowMut<ScopeContextExt<'input>>
{
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> ScopeContextAttrs<'input> for ScopeContext<'input> {}

impl<'input, I, H> QrustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn scope(&mut self) -> Result<Rc<ScopeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ScopeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_scope);
        let mut _localctx: Rc<ScopeContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(14);
                recog.base.match_token(T__0, &mut recog.err_handler)?;

                recog.base.set_state(18);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                loop {
                    {
                        {
                            /*InvokeRule expr*/
                            recog.base.set_state(15);
                            recog.expr()?;

                            recog.base.set_state(16);
                            recog.base.match_token(T__1, &mut recog.err_handler)?;
                        }
                    }
                    recog.base.set_state(20);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    if !(((_la) & !0x3f) == 0
                        && ((1usize << _la)
                            & ((1usize << T__3)
                                | (1usize << T__4)
                                | (1usize << TRUE)
                                | (1usize << FALSE)
                                | (1usize << INTEGER)
                                | (1usize << REAL)
                                | (1usize << SUB)
                                | (1usize << PAREN_L)))
                            != 0)
                    {
                        break;
                    }
                }
                recog.base.set_state(22);
                recog.base.match_token(T__2, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- expr ----------------
#[derive(Debug)]
pub enum ExprContextAll<'input> {
    Expr_outContext(Expr_outContext<'input>),
    Expr_termContext(Expr_termContext<'input>),
    Error(ExprContext<'input>),
}
antlr_rust::tid! {ExprContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ExprContextAll<'input> {}

impl<'input> QrustParserContext<'input> for ExprContextAll<'input> {}

impl<'input> Deref for ExprContextAll<'input> {
    type Target = dyn ExprContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use ExprContextAll::*;
        match self {
            Expr_outContext(inner) => inner,
            Expr_termContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for ExprContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn QrustVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for ExprContextAll<'input> {
    fn enter(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type ExprContext<'input> = BaseParserRuleContext<'input, ExprContextExt<'input>>;

#[derive(Clone)]
pub struct ExprContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> QrustParserContext<'input> for ExprContext<'input> {}

impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for ExprContext<'input> {}

impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for ExprContext<'input> {}

impl<'input> CustomRuleContext<'input> for ExprContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = QrustParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}
antlr_rust::tid! {ExprContextExt<'a>}

impl<'input> ExprContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn QrustParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ExprContextAll<'input>> {
        Rc::new(ExprContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                ExprContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait ExprContextAttrs<'input>:
    QrustParserContext<'input> + BorrowMut<ExprContextExt<'input>>
{
}

impl<'input> ExprContextAttrs<'input> for ExprContext<'input> {}

pub type Expr_outContext<'input> = BaseParserRuleContext<'input, Expr_outContextExt<'input>>;

pub trait Expr_outContextAttrs<'input>: QrustParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token PAREN_L
    /// Returns `None` if there is no child corresponding to token PAREN_L
    fn PAREN_L(&self) -> Option<Rc<TerminalNode<'input, QrustParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PAREN_L, 0)
    }
    fn term(&self) -> Option<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token PAREN_R
    /// Returns `None` if there is no child corresponding to token PAREN_R
    fn PAREN_R(&self) -> Option<Rc<TerminalNode<'input, QrustParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PAREN_R, 0)
    }
}

impl<'input> Expr_outContextAttrs<'input> for Expr_outContext<'input> {}

pub struct Expr_outContextExt<'input> {
    base: ExprContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Expr_outContextExt<'a>}

impl<'input> QrustParserContext<'input> for Expr_outContext<'input> {}

impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for Expr_outContext<'input> {
    fn enter(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_expr_out(self);
    }
    fn exit(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.exit_expr_out(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for Expr_outContext<'input> {
    fn accept(&self, visitor: &mut (dyn QrustVisitor<'input> + 'a)) {
        visitor.visit_expr_out(self);
    }
}

impl<'input> CustomRuleContext<'input> for Expr_outContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = QrustParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for Expr_outContext<'input> {
    fn borrow(&self) -> &ExprContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for Expr_outContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExprContextAttrs<'input> for Expr_outContext<'input> {}

impl<'input> Expr_outContextExt<'input> {
    fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>> {
        Rc::new(ExprContextAll::Expr_outContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Expr_outContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Expr_termContext<'input> = BaseParserRuleContext<'input, Expr_termContextExt<'input>>;

pub trait Expr_termContextAttrs<'input>: QrustParserContext<'input> {
    fn term(&self) -> Option<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Expr_termContextAttrs<'input> for Expr_termContext<'input> {}

pub struct Expr_termContextExt<'input> {
    base: ExprContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Expr_termContextExt<'a>}

impl<'input> QrustParserContext<'input> for Expr_termContext<'input> {}

impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for Expr_termContext<'input> {
    fn enter(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_expr_term(self);
    }
    fn exit(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.exit_expr_term(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for Expr_termContext<'input> {
    fn accept(&self, visitor: &mut (dyn QrustVisitor<'input> + 'a)) {
        visitor.visit_expr_term(self);
    }
}

impl<'input> CustomRuleContext<'input> for Expr_termContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = QrustParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for Expr_termContext<'input> {
    fn borrow(&self) -> &ExprContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for Expr_termContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExprContextAttrs<'input> for Expr_termContext<'input> {}

impl<'input> Expr_termContextExt<'input> {
    fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>> {
        Rc::new(ExprContextAll::Expr_termContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Expr_termContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> QrustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn expr(&mut self) -> Result<Rc<ExprContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_expr);
        let mut _localctx: Rc<ExprContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(30);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                T__3 => {
                    let tmp = Expr_outContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        recog.base.set_state(24);
                        recog.base.match_token(T__3, &mut recog.err_handler)?;

                        recog.base.set_state(25);
                        recog.base.match_token(PAREN_L, &mut recog.err_handler)?;

                        /*InvokeRule term*/
                        recog.base.set_state(26);
                        recog.term_rec(0)?;

                        recog.base.set_state(27);
                        recog.base.match_token(PAREN_R, &mut recog.err_handler)?;
                    }
                }

                T__4 | TRUE | FALSE | INTEGER | REAL | SUB | PAREN_L => {
                    let tmp = Expr_termContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        /*InvokeRule term*/
                        recog.base.set_state(29);
                        recog.term_rec(0)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- term ----------------
#[derive(Debug)]
pub enum TermContextAll<'input> {
    Term_add_subContext(Term_add_subContext<'input>),
    Term_literalContext(Term_literalContext<'input>),
    Term_negContext(Term_negContext<'input>),
    Term_mul_divContext(Term_mul_divContext<'input>),
    Term_parenContext(Term_parenContext<'input>),
    Error(TermContext<'input>),
}
antlr_rust::tid! {TermContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for TermContextAll<'input> {}

impl<'input> QrustParserContext<'input> for TermContextAll<'input> {}

impl<'input> Deref for TermContextAll<'input> {
    type Target = dyn TermContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use TermContextAll::*;
        match self {
            Term_add_subContext(inner) => inner,
            Term_literalContext(inner) => inner,
            Term_negContext(inner) => inner,
            Term_mul_divContext(inner) => inner,
            Term_parenContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for TermContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn QrustVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for TermContextAll<'input> {
    fn enter(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type TermContext<'input> = BaseParserRuleContext<'input, TermContextExt<'input>>;

#[derive(Clone)]
pub struct TermContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> QrustParserContext<'input> for TermContext<'input> {}

impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for TermContext<'input> {}

impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for TermContext<'input> {}

impl<'input> CustomRuleContext<'input> for TermContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = QrustParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_term
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_term }
}
antlr_rust::tid! {TermContextExt<'a>}

impl<'input> TermContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn QrustParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<TermContextAll<'input>> {
        Rc::new(TermContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                TermContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait TermContextAttrs<'input>:
    QrustParserContext<'input> + BorrowMut<TermContextExt<'input>>
{
}

impl<'input> TermContextAttrs<'input> for TermContext<'input> {}

pub type Term_add_subContext<'input> =
    BaseParserRuleContext<'input, Term_add_subContextExt<'input>>;

pub trait Term_add_subContextAttrs<'input>: QrustParserContext<'input> {
    fn term_all(&self) -> Vec<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn term(&self, i: usize) -> Option<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token ADD
    /// Returns `None` if there is no child corresponding to token ADD
    fn ADD(&self) -> Option<Rc<TerminalNode<'input, QrustParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ADD, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SUB
    /// Returns `None` if there is no child corresponding to token SUB
    fn SUB(&self) -> Option<Rc<TerminalNode<'input, QrustParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SUB, 0)
    }
}

impl<'input> Term_add_subContextAttrs<'input> for Term_add_subContext<'input> {}

pub struct Term_add_subContextExt<'input> {
    base: TermContextExt<'input>,
    pub lhs: Option<Rc<TermContextAll<'input>>>,
    pub rhs: Option<Rc<TermContextAll<'input>>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Term_add_subContextExt<'a>}

impl<'input> QrustParserContext<'input> for Term_add_subContext<'input> {}

impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for Term_add_subContext<'input> {
    fn enter(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_term_add_sub(self);
    }
    fn exit(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.exit_term_add_sub(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for Term_add_subContext<'input> {
    fn accept(&self, visitor: &mut (dyn QrustVisitor<'input> + 'a)) {
        visitor.visit_term_add_sub(self);
    }
}

impl<'input> CustomRuleContext<'input> for Term_add_subContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = QrustParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_term
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_term }
}

impl<'input> Borrow<TermContextExt<'input>> for Term_add_subContext<'input> {
    fn borrow(&self) -> &TermContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<TermContextExt<'input>> for Term_add_subContext<'input> {
    fn borrow_mut(&mut self) -> &mut TermContextExt<'input> {
        &mut self.base
    }
}

impl<'input> TermContextAttrs<'input> for Term_add_subContext<'input> {}

impl<'input> Term_add_subContextExt<'input> {
    fn new(ctx: &dyn TermContextAttrs<'input>) -> Rc<TermContextAll<'input>> {
        Rc::new(TermContextAll::Term_add_subContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Term_add_subContextExt {
                    lhs: None,
                    rhs: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Term_literalContext<'input> =
    BaseParserRuleContext<'input, Term_literalContextExt<'input>>;

pub trait Term_literalContextAttrs<'input>: QrustParserContext<'input> {
    fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Term_literalContextAttrs<'input> for Term_literalContext<'input> {}

pub struct Term_literalContextExt<'input> {
    base: TermContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Term_literalContextExt<'a>}

impl<'input> QrustParserContext<'input> for Term_literalContext<'input> {}

impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for Term_literalContext<'input> {
    fn enter(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_term_literal(self);
    }
    fn exit(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.exit_term_literal(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for Term_literalContext<'input> {
    fn accept(&self, visitor: &mut (dyn QrustVisitor<'input> + 'a)) {
        visitor.visit_term_literal(self);
    }
}

impl<'input> CustomRuleContext<'input> for Term_literalContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = QrustParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_term
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_term }
}

impl<'input> Borrow<TermContextExt<'input>> for Term_literalContext<'input> {
    fn borrow(&self) -> &TermContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<TermContextExt<'input>> for Term_literalContext<'input> {
    fn borrow_mut(&mut self) -> &mut TermContextExt<'input> {
        &mut self.base
    }
}

impl<'input> TermContextAttrs<'input> for Term_literalContext<'input> {}

impl<'input> Term_literalContextExt<'input> {
    fn new(ctx: &dyn TermContextAttrs<'input>) -> Rc<TermContextAll<'input>> {
        Rc::new(TermContextAll::Term_literalContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Term_literalContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Term_negContext<'input> = BaseParserRuleContext<'input, Term_negContextExt<'input>>;

pub trait Term_negContextAttrs<'input>: QrustParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token SUB
    /// Returns `None` if there is no child corresponding to token SUB
    fn SUB(&self) -> Option<Rc<TerminalNode<'input, QrustParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SUB, 0)
    }
    fn term(&self) -> Option<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Term_negContextAttrs<'input> for Term_negContext<'input> {}

pub struct Term_negContextExt<'input> {
    base: TermContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Term_negContextExt<'a>}

impl<'input> QrustParserContext<'input> for Term_negContext<'input> {}

impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for Term_negContext<'input> {
    fn enter(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_term_neg(self);
    }
    fn exit(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.exit_term_neg(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for Term_negContext<'input> {
    fn accept(&self, visitor: &mut (dyn QrustVisitor<'input> + 'a)) {
        visitor.visit_term_neg(self);
    }
}

impl<'input> CustomRuleContext<'input> for Term_negContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = QrustParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_term
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_term }
}

impl<'input> Borrow<TermContextExt<'input>> for Term_negContext<'input> {
    fn borrow(&self) -> &TermContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<TermContextExt<'input>> for Term_negContext<'input> {
    fn borrow_mut(&mut self) -> &mut TermContextExt<'input> {
        &mut self.base
    }
}

impl<'input> TermContextAttrs<'input> for Term_negContext<'input> {}

impl<'input> Term_negContextExt<'input> {
    fn new(ctx: &dyn TermContextAttrs<'input>) -> Rc<TermContextAll<'input>> {
        Rc::new(TermContextAll::Term_negContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Term_negContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Term_mul_divContext<'input> =
    BaseParserRuleContext<'input, Term_mul_divContextExt<'input>>;

pub trait Term_mul_divContextAttrs<'input>: QrustParserContext<'input> {
    fn term_all(&self) -> Vec<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn term(&self, i: usize) -> Option<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token MUL
    /// Returns `None` if there is no child corresponding to token MUL
    fn MUL(&self) -> Option<Rc<TerminalNode<'input, QrustParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MUL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DIV
    /// Returns `None` if there is no child corresponding to token DIV
    fn DIV(&self) -> Option<Rc<TerminalNode<'input, QrustParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DIV, 0)
    }
}

impl<'input> Term_mul_divContextAttrs<'input> for Term_mul_divContext<'input> {}

pub struct Term_mul_divContextExt<'input> {
    base: TermContextExt<'input>,
    pub lhs: Option<Rc<TermContextAll<'input>>>,
    pub rhs: Option<Rc<TermContextAll<'input>>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Term_mul_divContextExt<'a>}

impl<'input> QrustParserContext<'input> for Term_mul_divContext<'input> {}

impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for Term_mul_divContext<'input> {
    fn enter(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_term_mul_div(self);
    }
    fn exit(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.exit_term_mul_div(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for Term_mul_divContext<'input> {
    fn accept(&self, visitor: &mut (dyn QrustVisitor<'input> + 'a)) {
        visitor.visit_term_mul_div(self);
    }
}

impl<'input> CustomRuleContext<'input> for Term_mul_divContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = QrustParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_term
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_term }
}

impl<'input> Borrow<TermContextExt<'input>> for Term_mul_divContext<'input> {
    fn borrow(&self) -> &TermContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<TermContextExt<'input>> for Term_mul_divContext<'input> {
    fn borrow_mut(&mut self) -> &mut TermContextExt<'input> {
        &mut self.base
    }
}

impl<'input> TermContextAttrs<'input> for Term_mul_divContext<'input> {}

impl<'input> Term_mul_divContextExt<'input> {
    fn new(ctx: &dyn TermContextAttrs<'input>) -> Rc<TermContextAll<'input>> {
        Rc::new(TermContextAll::Term_mul_divContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Term_mul_divContextExt {
                    lhs: None,
                    rhs: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Term_parenContext<'input> = BaseParserRuleContext<'input, Term_parenContextExt<'input>>;

pub trait Term_parenContextAttrs<'input>: QrustParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token PAREN_L
    /// Returns `None` if there is no child corresponding to token PAREN_L
    fn PAREN_L(&self) -> Option<Rc<TerminalNode<'input, QrustParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PAREN_L, 0)
    }
    fn term(&self) -> Option<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token PAREN_R
    /// Returns `None` if there is no child corresponding to token PAREN_R
    fn PAREN_R(&self) -> Option<Rc<TerminalNode<'input, QrustParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PAREN_R, 0)
    }
}

impl<'input> Term_parenContextAttrs<'input> for Term_parenContext<'input> {}

pub struct Term_parenContextExt<'input> {
    base: TermContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Term_parenContextExt<'a>}

impl<'input> QrustParserContext<'input> for Term_parenContext<'input> {}

impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for Term_parenContext<'input> {
    fn enter(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_term_paren(self);
    }
    fn exit(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.exit_term_paren(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for Term_parenContext<'input> {
    fn accept(&self, visitor: &mut (dyn QrustVisitor<'input> + 'a)) {
        visitor.visit_term_paren(self);
    }
}

impl<'input> CustomRuleContext<'input> for Term_parenContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = QrustParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_term
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_term }
}

impl<'input> Borrow<TermContextExt<'input>> for Term_parenContext<'input> {
    fn borrow(&self) -> &TermContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<TermContextExt<'input>> for Term_parenContext<'input> {
    fn borrow_mut(&mut self) -> &mut TermContextExt<'input> {
        &mut self.base
    }
}

impl<'input> TermContextAttrs<'input> for Term_parenContext<'input> {}

impl<'input> Term_parenContextExt<'input> {
    fn new(ctx: &dyn TermContextAttrs<'input>) -> Rc<TermContextAll<'input>> {
        Rc::new(TermContextAll::Term_parenContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Term_parenContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> QrustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn term(&mut self) -> Result<Rc<TermContextAll<'input>>, ANTLRError> {
        self.term_rec(0)
    }

    fn term_rec(&mut self, _p: isize) -> Result<Rc<TermContextAll<'input>>, ANTLRError> {
        let recog = self;
        let _parentctx = recog.ctx.take();
        let _parentState = recog.base.get_state();
        let mut _localctx = TermContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_recursion_rule(_localctx.clone(), 4, RULE_term, _p);
        let mut _localctx: Rc<TermContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
        let _startState = 4;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(40);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    T__4 | TRUE | FALSE | INTEGER | REAL => {
                        {
                            let mut tmp = Term_literalContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();

                            /*InvokeRule literal*/
                            recog.base.set_state(33);
                            recog.literal()?;
                        }
                    }

                    PAREN_L => {
                        {
                            let mut tmp = Term_parenContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();
                            recog.base.set_state(34);
                            recog.base.match_token(PAREN_L, &mut recog.err_handler)?;

                            /*InvokeRule term*/
                            recog.base.set_state(35);
                            recog.term_rec(0)?;

                            recog.base.set_state(36);
                            recog.base.match_token(PAREN_R, &mut recog.err_handler)?;
                        }
                    }

                    SUB => {
                        {
                            let mut tmp = Term_negContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();
                            recog.base.set_state(38);
                            recog.base.match_token(SUB, &mut recog.err_handler)?;

                            /*InvokeRule term*/
                            recog.base.set_state(39);
                            recog.term_rec(1)?;
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }

                let tmp = recog.input.lt(-1).cloned();
                recog.ctx.as_ref().unwrap().set_stop(tmp);
                recog.base.set_state(50);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(4, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        recog.trigger_exit_rule_event();
                        _prevctx = _localctx.clone();
                        {
                            recog.base.set_state(48);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(3, &mut recog.base)? {
                                1 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp =
                                            Term_mul_divContextExt::new(&**TermContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ));
                                        if let TermContextAll::Term_mul_divContext(ctx) =
                                            cast_mut::<_, TermContextAll>(&mut tmp)
                                        {
                                            ctx.lhs = Some(_prevctx.clone());
                                        } else {
                                            unreachable!("cant cast");
                                        }
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_term,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(42);
                                        if !({ recog.precpred(None, 4) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 4)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(43);
                                        _la = recog.base.input.la(1);
                                        if { !(_la == MUL || _la == DIV) } {
                                            recog.err_handler.recover_inline(&mut recog.base)?;
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule term*/
                                        recog.base.set_state(44);
                                        let tmp = recog.term_rec(5)?;
                                        if let TermContextAll::Term_mul_divContext(ctx) =
                                            cast_mut::<_, TermContextAll>(&mut _localctx)
                                        {
                                            ctx.rhs = Some(tmp.clone());
                                        } else {
                                            unreachable!("cant cast");
                                        }
                                    }
                                }
                                2 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp =
                                            Term_add_subContextExt::new(&**TermContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ));
                                        if let TermContextAll::Term_add_subContext(ctx) =
                                            cast_mut::<_, TermContextAll>(&mut tmp)
                                        {
                                            ctx.lhs = Some(_prevctx.clone());
                                        } else {
                                            unreachable!("cant cast");
                                        }
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_term,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(45);
                                        if !({ recog.precpred(None, 3) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 3)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(46);
                                        _la = recog.base.input.la(1);
                                        if { !(_la == ADD || _la == SUB) } {
                                            recog.err_handler.recover_inline(&mut recog.base)?;
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule term*/
                                        recog.base.set_state(47);
                                        let tmp = recog.term_rec(4)?;
                                        if let TermContextAll::Term_add_subContext(ctx) =
                                            cast_mut::<_, TermContextAll>(&mut _localctx)
                                        {
                                            ctx.rhs = Some(tmp.clone());
                                        } else {
                                            unreachable!("cant cast");
                                        }
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                    recog.base.set_state(52);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(4, &mut recog.base)?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.unroll_recursion_context(_parentctx);

        Ok(_localctx)
    }
}
//------------------- literal ----------------
#[derive(Debug)]
pub enum LiteralContextAll<'input> {
    Value_booleanContext(Value_booleanContext<'input>),
    Value_numericContext(Value_numericContext<'input>),
    Value_matrixContext(Value_matrixContext<'input>),
    Error(LiteralContext<'input>),
}
antlr_rust::tid! {LiteralContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for LiteralContextAll<'input> {}

impl<'input> QrustParserContext<'input> for LiteralContextAll<'input> {}

impl<'input> Deref for LiteralContextAll<'input> {
    type Target = dyn LiteralContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use LiteralContextAll::*;
        match self {
            Value_booleanContext(inner) => inner,
            Value_numericContext(inner) => inner,
            Value_matrixContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for LiteralContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn QrustVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for LiteralContextAll<'input> {
    fn enter(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type LiteralContext<'input> = BaseParserRuleContext<'input, LiteralContextExt<'input>>;

#[derive(Clone)]
pub struct LiteralContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> QrustParserContext<'input> for LiteralContext<'input> {}

impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for LiteralContext<'input> {}

impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for LiteralContext<'input> {}

impl<'input> CustomRuleContext<'input> for LiteralContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = QrustParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_literal
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_literal }
}
antlr_rust::tid! {LiteralContextExt<'a>}

impl<'input> LiteralContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn QrustParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<LiteralContextAll<'input>> {
        Rc::new(LiteralContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                LiteralContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait LiteralContextAttrs<'input>:
    QrustParserContext<'input> + BorrowMut<LiteralContextExt<'input>>
{
}

impl<'input> LiteralContextAttrs<'input> for LiteralContext<'input> {}

pub type Value_booleanContext<'input> =
    BaseParserRuleContext<'input, Value_booleanContextExt<'input>>;

pub trait Value_booleanContextAttrs<'input>: QrustParserContext<'input> {
    fn boolean(&self) -> Option<Rc<BooleanContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Value_booleanContextAttrs<'input> for Value_booleanContext<'input> {}

pub struct Value_booleanContextExt<'input> {
    base: LiteralContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Value_booleanContextExt<'a>}

impl<'input> QrustParserContext<'input> for Value_booleanContext<'input> {}

impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for Value_booleanContext<'input> {
    fn enter(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_value_boolean(self);
    }
    fn exit(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.exit_value_boolean(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for Value_booleanContext<'input> {
    fn accept(&self, visitor: &mut (dyn QrustVisitor<'input> + 'a)) {
        visitor.visit_value_boolean(self);
    }
}

impl<'input> CustomRuleContext<'input> for Value_booleanContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = QrustParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_literal
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_literal }
}

impl<'input> Borrow<LiteralContextExt<'input>> for Value_booleanContext<'input> {
    fn borrow(&self) -> &LiteralContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<LiteralContextExt<'input>> for Value_booleanContext<'input> {
    fn borrow_mut(&mut self) -> &mut LiteralContextExt<'input> {
        &mut self.base
    }
}

impl<'input> LiteralContextAttrs<'input> for Value_booleanContext<'input> {}

impl<'input> Value_booleanContextExt<'input> {
    fn new(ctx: &dyn LiteralContextAttrs<'input>) -> Rc<LiteralContextAll<'input>> {
        Rc::new(LiteralContextAll::Value_booleanContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Value_booleanContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Value_numericContext<'input> =
    BaseParserRuleContext<'input, Value_numericContextExt<'input>>;

pub trait Value_numericContextAttrs<'input>: QrustParserContext<'input> {
    fn numeric(&self) -> Option<Rc<NumericContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Value_numericContextAttrs<'input> for Value_numericContext<'input> {}

pub struct Value_numericContextExt<'input> {
    base: LiteralContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Value_numericContextExt<'a>}

impl<'input> QrustParserContext<'input> for Value_numericContext<'input> {}

impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for Value_numericContext<'input> {
    fn enter(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_value_numeric(self);
    }
    fn exit(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.exit_value_numeric(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for Value_numericContext<'input> {
    fn accept(&self, visitor: &mut (dyn QrustVisitor<'input> + 'a)) {
        visitor.visit_value_numeric(self);
    }
}

impl<'input> CustomRuleContext<'input> for Value_numericContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = QrustParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_literal
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_literal }
}

impl<'input> Borrow<LiteralContextExt<'input>> for Value_numericContext<'input> {
    fn borrow(&self) -> &LiteralContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<LiteralContextExt<'input>> for Value_numericContext<'input> {
    fn borrow_mut(&mut self) -> &mut LiteralContextExt<'input> {
        &mut self.base
    }
}

impl<'input> LiteralContextAttrs<'input> for Value_numericContext<'input> {}

impl<'input> Value_numericContextExt<'input> {
    fn new(ctx: &dyn LiteralContextAttrs<'input>) -> Rc<LiteralContextAll<'input>> {
        Rc::new(LiteralContextAll::Value_numericContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Value_numericContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Value_matrixContext<'input> =
    BaseParserRuleContext<'input, Value_matrixContextExt<'input>>;

pub trait Value_matrixContextAttrs<'input>: QrustParserContext<'input> {
    fn matrix(&self) -> Option<Rc<MatrixContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Value_matrixContextAttrs<'input> for Value_matrixContext<'input> {}

pub struct Value_matrixContextExt<'input> {
    base: LiteralContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Value_matrixContextExt<'a>}

impl<'input> QrustParserContext<'input> for Value_matrixContext<'input> {}

impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for Value_matrixContext<'input> {
    fn enter(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_value_matrix(self);
    }
    fn exit(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.exit_value_matrix(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for Value_matrixContext<'input> {
    fn accept(&self, visitor: &mut (dyn QrustVisitor<'input> + 'a)) {
        visitor.visit_value_matrix(self);
    }
}

impl<'input> CustomRuleContext<'input> for Value_matrixContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = QrustParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_literal
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_literal }
}

impl<'input> Borrow<LiteralContextExt<'input>> for Value_matrixContext<'input> {
    fn borrow(&self) -> &LiteralContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<LiteralContextExt<'input>> for Value_matrixContext<'input> {
    fn borrow_mut(&mut self) -> &mut LiteralContextExt<'input> {
        &mut self.base
    }
}

impl<'input> LiteralContextAttrs<'input> for Value_matrixContext<'input> {}

impl<'input> Value_matrixContextExt<'input> {
    fn new(ctx: &dyn LiteralContextAttrs<'input>) -> Rc<LiteralContextAll<'input>> {
        Rc::new(LiteralContextAll::Value_matrixContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Value_matrixContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> QrustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn literal(&mut self) -> Result<Rc<LiteralContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = LiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_literal);
        let mut _localctx: Rc<LiteralContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(56);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                TRUE | FALSE => {
                    let tmp = Value_booleanContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        /*InvokeRule boolean*/
                        recog.base.set_state(53);
                        recog.boolean()?;
                    }
                }

                INTEGER | REAL => {
                    let tmp = Value_numericContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        /*InvokeRule numeric*/
                        recog.base.set_state(54);
                        recog.numeric()?;
                    }
                }

                T__4 => {
                    let tmp = Value_matrixContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 3);
                    _localctx = tmp;
                    {
                        /*InvokeRule matrix*/
                        recog.base.set_state(55);
                        recog.matrix()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- boolean ----------------
#[derive(Debug)]
pub enum BooleanContextAll<'input> {
    Boolean_falseContext(Boolean_falseContext<'input>),
    Boolean_trueContext(Boolean_trueContext<'input>),
    Error(BooleanContext<'input>),
}
antlr_rust::tid! {BooleanContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for BooleanContextAll<'input> {}

impl<'input> QrustParserContext<'input> for BooleanContextAll<'input> {}

impl<'input> Deref for BooleanContextAll<'input> {
    type Target = dyn BooleanContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use BooleanContextAll::*;
        match self {
            Boolean_falseContext(inner) => inner,
            Boolean_trueContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for BooleanContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn QrustVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for BooleanContextAll<'input> {
    fn enter(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type BooleanContext<'input> = BaseParserRuleContext<'input, BooleanContextExt<'input>>;

#[derive(Clone)]
pub struct BooleanContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> QrustParserContext<'input> for BooleanContext<'input> {}

impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for BooleanContext<'input> {}

impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for BooleanContext<'input> {}

impl<'input> CustomRuleContext<'input> for BooleanContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = QrustParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_boolean
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_boolean }
}
antlr_rust::tid! {BooleanContextExt<'a>}

impl<'input> BooleanContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn QrustParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<BooleanContextAll<'input>> {
        Rc::new(BooleanContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                BooleanContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait BooleanContextAttrs<'input>:
    QrustParserContext<'input> + BorrowMut<BooleanContextExt<'input>>
{
}

impl<'input> BooleanContextAttrs<'input> for BooleanContext<'input> {}

pub type Boolean_falseContext<'input> =
    BaseParserRuleContext<'input, Boolean_falseContextExt<'input>>;

pub trait Boolean_falseContextAttrs<'input>: QrustParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token FALSE
    /// Returns `None` if there is no child corresponding to token FALSE
    fn FALSE(&self) -> Option<Rc<TerminalNode<'input, QrustParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FALSE, 0)
    }
}

impl<'input> Boolean_falseContextAttrs<'input> for Boolean_falseContext<'input> {}

pub struct Boolean_falseContextExt<'input> {
    base: BooleanContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Boolean_falseContextExt<'a>}

impl<'input> QrustParserContext<'input> for Boolean_falseContext<'input> {}

impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for Boolean_falseContext<'input> {
    fn enter(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_boolean_false(self);
    }
    fn exit(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.exit_boolean_false(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for Boolean_falseContext<'input> {
    fn accept(&self, visitor: &mut (dyn QrustVisitor<'input> + 'a)) {
        visitor.visit_boolean_false(self);
    }
}

impl<'input> CustomRuleContext<'input> for Boolean_falseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = QrustParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_boolean
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_boolean }
}

impl<'input> Borrow<BooleanContextExt<'input>> for Boolean_falseContext<'input> {
    fn borrow(&self) -> &BooleanContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<BooleanContextExt<'input>> for Boolean_falseContext<'input> {
    fn borrow_mut(&mut self) -> &mut BooleanContextExt<'input> {
        &mut self.base
    }
}

impl<'input> BooleanContextAttrs<'input> for Boolean_falseContext<'input> {}

impl<'input> Boolean_falseContextExt<'input> {
    fn new(ctx: &dyn BooleanContextAttrs<'input>) -> Rc<BooleanContextAll<'input>> {
        Rc::new(BooleanContextAll::Boolean_falseContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Boolean_falseContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Boolean_trueContext<'input> =
    BaseParserRuleContext<'input, Boolean_trueContextExt<'input>>;

pub trait Boolean_trueContextAttrs<'input>: QrustParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token TRUE
    /// Returns `None` if there is no child corresponding to token TRUE
    fn TRUE(&self) -> Option<Rc<TerminalNode<'input, QrustParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(TRUE, 0)
    }
}

impl<'input> Boolean_trueContextAttrs<'input> for Boolean_trueContext<'input> {}

pub struct Boolean_trueContextExt<'input> {
    base: BooleanContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Boolean_trueContextExt<'a>}

impl<'input> QrustParserContext<'input> for Boolean_trueContext<'input> {}

impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for Boolean_trueContext<'input> {
    fn enter(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_boolean_true(self);
    }
    fn exit(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.exit_boolean_true(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for Boolean_trueContext<'input> {
    fn accept(&self, visitor: &mut (dyn QrustVisitor<'input> + 'a)) {
        visitor.visit_boolean_true(self);
    }
}

impl<'input> CustomRuleContext<'input> for Boolean_trueContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = QrustParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_boolean
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_boolean }
}

impl<'input> Borrow<BooleanContextExt<'input>> for Boolean_trueContext<'input> {
    fn borrow(&self) -> &BooleanContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<BooleanContextExt<'input>> for Boolean_trueContext<'input> {
    fn borrow_mut(&mut self) -> &mut BooleanContextExt<'input> {
        &mut self.base
    }
}

impl<'input> BooleanContextAttrs<'input> for Boolean_trueContext<'input> {}

impl<'input> Boolean_trueContextExt<'input> {
    fn new(ctx: &dyn BooleanContextAttrs<'input>) -> Rc<BooleanContextAll<'input>> {
        Rc::new(BooleanContextAll::Boolean_trueContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Boolean_trueContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> QrustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn boolean(&mut self) -> Result<Rc<BooleanContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = BooleanContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_boolean);
        let mut _localctx: Rc<BooleanContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(60);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                TRUE => {
                    let tmp = Boolean_trueContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        recog.base.set_state(58);
                        recog.base.match_token(TRUE, &mut recog.err_handler)?;
                    }
                }

                FALSE => {
                    let tmp = Boolean_falseContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        recog.base.set_state(59);
                        recog.base.match_token(FALSE, &mut recog.err_handler)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- numeric ----------------
#[derive(Debug)]
pub enum NumericContextAll<'input> {
    Numeric_realContext(Numeric_realContext<'input>),
    Numeric_integerContext(Numeric_integerContext<'input>),
    Error(NumericContext<'input>),
}
antlr_rust::tid! {NumericContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for NumericContextAll<'input> {}

impl<'input> QrustParserContext<'input> for NumericContextAll<'input> {}

impl<'input> Deref for NumericContextAll<'input> {
    type Target = dyn NumericContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use NumericContextAll::*;
        match self {
            Numeric_realContext(inner) => inner,
            Numeric_integerContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for NumericContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn QrustVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for NumericContextAll<'input> {
    fn enter(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type NumericContext<'input> = BaseParserRuleContext<'input, NumericContextExt<'input>>;

#[derive(Clone)]
pub struct NumericContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> QrustParserContext<'input> for NumericContext<'input> {}

impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for NumericContext<'input> {}

impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for NumericContext<'input> {}

impl<'input> CustomRuleContext<'input> for NumericContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = QrustParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_numeric
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_numeric }
}
antlr_rust::tid! {NumericContextExt<'a>}

impl<'input> NumericContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn QrustParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<NumericContextAll<'input>> {
        Rc::new(NumericContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                NumericContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait NumericContextAttrs<'input>:
    QrustParserContext<'input> + BorrowMut<NumericContextExt<'input>>
{
}

impl<'input> NumericContextAttrs<'input> for NumericContext<'input> {}

pub type Numeric_realContext<'input> =
    BaseParserRuleContext<'input, Numeric_realContextExt<'input>>;

pub trait Numeric_realContextAttrs<'input>: QrustParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token REAL
    /// Returns `None` if there is no child corresponding to token REAL
    fn REAL(&self) -> Option<Rc<TerminalNode<'input, QrustParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(REAL, 0)
    }
}

impl<'input> Numeric_realContextAttrs<'input> for Numeric_realContext<'input> {}

pub struct Numeric_realContextExt<'input> {
    base: NumericContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Numeric_realContextExt<'a>}

impl<'input> QrustParserContext<'input> for Numeric_realContext<'input> {}

impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for Numeric_realContext<'input> {
    fn enter(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_numeric_real(self);
    }
    fn exit(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.exit_numeric_real(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for Numeric_realContext<'input> {
    fn accept(&self, visitor: &mut (dyn QrustVisitor<'input> + 'a)) {
        visitor.visit_numeric_real(self);
    }
}

impl<'input> CustomRuleContext<'input> for Numeric_realContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = QrustParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_numeric
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_numeric }
}

impl<'input> Borrow<NumericContextExt<'input>> for Numeric_realContext<'input> {
    fn borrow(&self) -> &NumericContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<NumericContextExt<'input>> for Numeric_realContext<'input> {
    fn borrow_mut(&mut self) -> &mut NumericContextExt<'input> {
        &mut self.base
    }
}

impl<'input> NumericContextAttrs<'input> for Numeric_realContext<'input> {}

impl<'input> Numeric_realContextExt<'input> {
    fn new(ctx: &dyn NumericContextAttrs<'input>) -> Rc<NumericContextAll<'input>> {
        Rc::new(NumericContextAll::Numeric_realContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Numeric_realContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Numeric_integerContext<'input> =
    BaseParserRuleContext<'input, Numeric_integerContextExt<'input>>;

pub trait Numeric_integerContextAttrs<'input>: QrustParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token INTEGER
    /// Returns `None` if there is no child corresponding to token INTEGER
    fn INTEGER(&self) -> Option<Rc<TerminalNode<'input, QrustParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(INTEGER, 0)
    }
}

impl<'input> Numeric_integerContextAttrs<'input> for Numeric_integerContext<'input> {}

pub struct Numeric_integerContextExt<'input> {
    base: NumericContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Numeric_integerContextExt<'a>}

impl<'input> QrustParserContext<'input> for Numeric_integerContext<'input> {}

impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for Numeric_integerContext<'input> {
    fn enter(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_numeric_integer(self);
    }
    fn exit(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.exit_numeric_integer(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for Numeric_integerContext<'input> {
    fn accept(&self, visitor: &mut (dyn QrustVisitor<'input> + 'a)) {
        visitor.visit_numeric_integer(self);
    }
}

impl<'input> CustomRuleContext<'input> for Numeric_integerContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = QrustParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_numeric
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_numeric }
}

impl<'input> Borrow<NumericContextExt<'input>> for Numeric_integerContext<'input> {
    fn borrow(&self) -> &NumericContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<NumericContextExt<'input>> for Numeric_integerContext<'input> {
    fn borrow_mut(&mut self) -> &mut NumericContextExt<'input> {
        &mut self.base
    }
}

impl<'input> NumericContextAttrs<'input> for Numeric_integerContext<'input> {}

impl<'input> Numeric_integerContextExt<'input> {
    fn new(ctx: &dyn NumericContextAttrs<'input>) -> Rc<NumericContextAll<'input>> {
        Rc::new(NumericContextAll::Numeric_integerContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Numeric_integerContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> QrustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn numeric(&mut self) -> Result<Rc<NumericContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = NumericContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_numeric);
        let mut _localctx: Rc<NumericContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(64);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                INTEGER => {
                    let tmp = Numeric_integerContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        recog.base.set_state(62);
                        recog.base.match_token(INTEGER, &mut recog.err_handler)?;
                    }
                }

                REAL => {
                    let tmp = Numeric_realContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        recog.base.set_state(63);
                        recog.base.match_token(REAL, &mut recog.err_handler)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- matrix ----------------
pub type MatrixContextAll<'input> = MatrixContext<'input>;

pub type MatrixContext<'input> = BaseParserRuleContext<'input, MatrixContextExt<'input>>;

#[derive(Clone)]
pub struct MatrixContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> QrustParserContext<'input> for MatrixContext<'input> {}

impl<'input, 'a> Listenable<dyn QrustListener<'input> + 'a> for MatrixContext<'input> {
    fn enter(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_matrix(self);
    }
    fn exit(&self, listener: &mut (dyn QrustListener<'input> + 'a)) {
        listener.exit_matrix(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn QrustVisitor<'input> + 'a> for MatrixContext<'input> {
    fn accept(&self, visitor: &mut (dyn QrustVisitor<'input> + 'a)) {
        visitor.visit_matrix(self);
    }
}

impl<'input> CustomRuleContext<'input> for MatrixContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = QrustParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_matrix
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_matrix }
}
antlr_rust::tid! {MatrixContextExt<'a>}

impl<'input> MatrixContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn QrustParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<MatrixContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            MatrixContextExt { ph: PhantomData },
        ))
    }
}

pub trait MatrixContextAttrs<'input>:
    QrustParserContext<'input> + BorrowMut<MatrixContextExt<'input>>
{
    fn term_all(&self) -> Vec<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn term(&self, i: usize) -> Option<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> MatrixContextAttrs<'input> for MatrixContext<'input> {}

impl<'input, I, H> QrustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn matrix(&mut self) -> Result<Rc<MatrixContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = MatrixContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_matrix);
        let mut _localctx: Rc<MatrixContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(66);
                recog.base.match_token(T__4, &mut recog.err_handler)?;

                recog.base.set_state(72);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(8, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                /*InvokeRule term*/
                                recog.base.set_state(67);
                                recog.term_rec(0)?;

                                recog.base.set_state(68);
                                recog.base.match_token(T__5, &mut recog.err_handler)?;
                            }
                        }
                    }
                    recog.base.set_state(74);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(8, &mut recog.base)?;
                }
                recog.base.set_state(76);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if ((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << T__4)
                            | (1usize << TRUE)
                            | (1usize << FALSE)
                            | (1usize << INTEGER)
                            | (1usize << REAL)
                            | (1usize << SUB)
                            | (1usize << PAREN_L)))
                        != 0
                {
                    {
                        /*InvokeRule term*/
                        recog.base.set_state(75);
                        recog.term_rec(0)?;
                    }
                }

                recog.base.set_state(78);
                recog.base.match_token(T__6, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(_ATN.clone(), _ATN.get_decision_state(i), i as isize).into())
        }
        Arc::new(dfa)
    };
}

const _serializedATN: &'static str = "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x14\x53\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x03\x02\x03\x02\x03\x02\
	\x03\x02\x06\x02\x15\x0a\x02\x0d\x02\x0e\x02\x16\x03\x02\x03\x02\x03\x03\
	\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x05\x03\x21\x0a\x03\x03\x04\x03\
	\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x05\x04\x2b\x0a\x04\
	\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x07\x04\x33\x0a\x04\x0c\
	\x04\x0e\x04\x36\x0b\x04\x03\x05\x03\x05\x03\x05\x05\x05\x3b\x0a\x05\x03\
	\x06\x03\x06\x05\x06\x3f\x0a\x06\x03\x07\x03\x07\x05\x07\x43\x0a\x07\x03\
	\x08\x03\x08\x03\x08\x03\x08\x07\x08\x49\x0a\x08\x0c\x08\x0e\x08\x4c\x0b\
	\x08\x03\x08\x05\x08\x4f\x0a\x08\x03\x08\x03\x08\x03\x08\x02\x03\x06\x09\
	\x02\x04\x06\x08\x0a\x0c\x0e\x02\x04\x03\x02\x0e\x0f\x03\x02\x10\x11\x02\
	\x57\x02\x10\x03\x02\x02\x02\x04\x20\x03\x02\x02\x02\x06\x2a\x03\x02\x02\
	\x02\x08\x3a\x03\x02\x02\x02\x0a\x3e\x03\x02\x02\x02\x0c\x42\x03\x02\x02\
	\x02\x0e\x44\x03\x02\x02\x02\x10\x14\x07\x03\x02\x02\x11\x12\x05\x04\x03\
	\x02\x12\x13\x07\x04\x02\x02\x13\x15\x03\x02\x02\x02\x14\x11\x03\x02\x02\
	\x02\x15\x16\x03\x02\x02\x02\x16\x14\x03\x02\x02\x02\x16\x17\x03\x02\x02\
	\x02\x17\x18\x03\x02\x02\x02\x18\x19\x07\x05\x02\x02\x19\x03\x03\x02\x02\
	\x02\x1a\x1b\x07\x06\x02\x02\x1b\x1c\x07\x12\x02\x02\x1c\x1d\x05\x06\x04\
	\x02\x1d\x1e\x07\x13\x02\x02\x1e\x21\x03\x02\x02\x02\x1f\x21\x05\x06\x04\
	\x02\x20\x1a\x03\x02\x02\x02\x20\x1f\x03\x02\x02\x02\x21\x05\x03\x02\x02\
	\x02\x22\x23\x08\x04\x01\x02\x23\x2b\x05\x08\x05\x02\x24\x25\x07\x12\x02\
	\x02\x25\x26\x05\x06\x04\x02\x26\x27\x07\x13\x02\x02\x27\x2b\x03\x02\x02\
	\x02\x28\x29\x07\x11\x02\x02\x29\x2b\x05\x06\x04\x03\x2a\x22\x03\x02\x02\
	\x02\x2a\x24\x03\x02\x02\x02\x2a\x28\x03\x02\x02\x02\x2b\x34\x03\x02\x02\
	\x02\x2c\x2d\x0c\x06\x02\x02\x2d\x2e\x09\x02\x02\x02\x2e\x33\x05\x06\x04\
	\x07\x2f\x30\x0c\x05\x02\x02\x30\x31\x09\x03\x02\x02\x31\x33\x05\x06\x04\
	\x06\x32\x2c\x03\x02\x02\x02\x32\x2f\x03\x02\x02\x02\x33\x36\x03\x02\x02\
	\x02\x34\x32\x03\x02\x02\x02\x34\x35\x03\x02\x02\x02\x35\x07\x03\x02\x02\
	\x02\x36\x34\x03\x02\x02\x02\x37\x3b\x05\x0a\x06\x02\x38\x3b\x05\x0c\x07\
	\x02\x39\x3b\x05\x0e\x08\x02\x3a\x37\x03\x02\x02\x02\x3a\x38\x03\x02\x02\
	\x02\x3a\x39\x03\x02\x02\x02\x3b\x09\x03\x02\x02\x02\x3c\x3f\x07\x0a\x02\
	\x02\x3d\x3f\x07\x0b\x02\x02\x3e\x3c\x03\x02\x02\x02\x3e\x3d\x03\x02\x02\
	\x02\x3f\x0b\x03\x02\x02\x02\x40\x43\x07\x0c\x02\x02\x41\x43\x07\x0d\x02\
	\x02\x42\x40\x03\x02\x02\x02\x42\x41\x03\x02\x02\x02\x43\x0d\x03\x02\x02\
	\x02\x44\x4a\x07\x07\x02\x02\x45\x46\x05\x06\x04\x02\x46\x47\x07\x08\x02\
	\x02\x47\x49\x03\x02\x02\x02\x48\x45\x03\x02\x02\x02\x49\x4c\x03\x02\x02\
	\x02\x4a\x48\x03\x02\x02\x02\x4a\x4b\x03\x02\x02\x02\x4b\x4e\x03\x02\x02\
	\x02\x4c\x4a\x03\x02\x02\x02\x4d\x4f\x05\x06\x04\x02\x4e\x4d\x03\x02\x02\
	\x02\x4e\x4f\x03\x02\x02\x02\x4f\x50\x03\x02\x02\x02\x50\x51\x07\x09\x02\
	\x02\x51\x0f\x03\x02\x02\x02\x0c\x16\x20\x2a\x32\x34\x3a\x3e\x42\x4a\x4e";
