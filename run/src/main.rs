mod logger;

use std::fs::File;
use std::io::Read;
use std::process;
use std::time::Instant;

use antlr_rust::InputStream;
use antlr_rust::common_token_stream::CommonTokenStream;
use ast::ast_visitor;
use clif::codegen::FunctionCodegen;
use parse::qrustvisitor::QrustVisitorCompat;
use typed::typecheck;

use cranelift_codegen::{Context, ir::{self, Function, Signature, types::I32}, settings::{self, Configurable}, verify_function};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};

use cranelift_object::{ObjectBuilder, ObjectModule};
use cranelift_module::{Linkage, Module, default_libcall_names};

fn read(path: &str) -> Vec<u8> {
    let mut file = File::open(path).unwrap_or_else(|err| {
        log::error!("Failed to open `{path}`: {err}");
        process::exit(1);
    });

    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(bytes) => {
            log::info!("Read {bytes} bytes from {path}");
            buffer
        }
        Err(err) => {
            log::error!("Failed to read `{path}`: {err}");
            process::exit(1);
        }
    }
}

fn main() {
    logger::init();

    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "./input.txt".to_string());
    let data = read(&path);

    //  (step num). (code format) --> (processor) = ...:

    // 1. raw input     --> lexer =
    // 2. token stream  --> parser =
    // 3. grammar tree  --> AST Visitor =
    // 4. AST Tree      --> typecheck =
    // 5. typed AST     --> build_standalone =
    // 6. clif IR       --> verify/finalize =
    // 7. verified IR   --> 

    let input_stream = InputStream::new(&data[..]);
    let lexer = parse::qrustlexer::QrustLexer::new(input_stream);
    let token_stream = CommonTokenStream::new(lexer);

    let t_parse_start = Instant::now();

    // parse raw text into a walkable grammar tree
    let mut parser = parse::qrustparser::QrustParser::new(token_stream);


    // walk the tree, with the parse rule always being `scope: `
    let start = parser.scope().unwrap_or_else(|err| {
        log::error!("Parse failed: {err:?}");
        process::exit(1);
    });
    log::info!("Parsed in {:?}", t_parse_start.elapsed());
    
    let mut ast_visitor = ast_visitor::ASTVisitor {
        temp_result: Box::new(ast::nodes::UntypedNode::new(
            ast::nodes::NodeKind::Unit,
            ast::nodes::Span { start: 0, end: 1 },
        )),
    };

    let t_ast_start = Instant::now();
    // visit the first rule (this rule should match the first parser visit)
    let result = ast_visitor.visit_scope(&start);
    log::info!("AST built in {:?}", t_ast_start.elapsed());
    log::debug!("Untyped AST:\n{result:#?}");

    let t_typed_start = Instant::now();
    // run typecheck & transform into TypedNode
    let typed_ast = typecheck::typecheck(&result).unwrap_or_else(|err| {
        log::error!("Typecheck failed: {err}");
        process::exit(1);
    });
    log::info!("Typechecked in {:?}", t_typed_start.elapsed());
    log::debug!("Typed AST:\n{typed_ast:#?}");


    // start writing IR
    let t_compile_start = Instant::now();

    let mut flags = settings::builder();
    flags.set("is_pic", "true").unwrap(); // PIE-by-default linkers need this

    // MODULE //
    // instruction-specific flags for the ObjectModule builder
    let isa = cranelift_native::builder()
        .unwrap()
        .finish(settings::Flags::new(flags))
        .unwrap();
    let mut module = ObjectModule::new(ObjectBuilder::new(isa, "supacool", default_libcall_names()).unwrap());
    let mut mod_ctx = module.make_context();

    // module signature for `main`
    let mut sig = module.make_signature();
    sig.returns.push(ir::AbiParam::new(I32));

    // create the `main` function
    let main_id = module.declare_function("main", Linkage::Export, &sig).unwrap();


    // populate the function with `typed_ast`
    let mut fn_ctx = FunctionBuilderContext::new();
    mod_ctx.func.signature = sig;
    {
        let mut fn_codegen = FunctionCodegen::new(&mut mod_ctx.func, &mut fn_ctx);
        // build_standalone creates a block, seals it with the necessary function params andblock params, and then adds a return instruction
        // for `node`, (if it returned anything, otherwise empty return statement)
        fn_codegen.build_standalone(&typed_ast, true);
        fn_codegen.finalize();
    }

    module.define_function(main_id, &mut mod_ctx).unwrap();
    module.clear_context(&mut mod_ctx);

    log::info!(
        "Codegen finished in {:?}",
        t_compile_start.elapsed()
    );
}
