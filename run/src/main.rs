mod logger;

use std::fs::File;
use std::io::Read;
use std::process;
use std::time::Instant;

use antlr_rust::InputStream;
use antlr_rust::common_token_stream::CommonTokenStream;
use ast::ast_visitor;
use clif::codegen::Codegen;
use parse::qrustvisitor::QrustVisitorCompat;
use typed::typecheck;

use cranelift_codegen::{ir::Function, settings, verify_function};
use cranelift_frontend::FunctionBuilderContext;

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
    let mut parser = parse::qrustparser::QrustParser::new(token_stream);

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
    let result = ast_visitor.visit_scope(&start);
    log::info!("AST built in {:?}", t_ast_start.elapsed());
    log::debug!("Untyped AST:\n{result:#?}");

    let t_typed_start = Instant::now();
    let typed_ast = typecheck::typecheck(&result).unwrap_or_else(|err| {
        log::error!("Typecheck failed: {err}");
        process::exit(1);
    });
    log::info!("Typechecked in {:?}", t_typed_start.elapsed());
    log::debug!("Typed AST:\n{typed_ast:#?}");

    let t_compile_start = Instant::now();
    let mut function = Function::new();
    let mut ctx = FunctionBuilderContext::new();
    let mut codegen = Codegen::new(&mut function, &mut ctx);

    // this usually writes to `function` and the return value doesn't have any meaningful information
    codegen.build_standalone(&typed_ast);

    log::info!(
        "Codegen finished in {:?}",
        t_compile_start.elapsed()
    );

    let finalize_result = codegen.try_finalize();
    if let Err(error) = finalize_result {
        log::error!("Failed to finalize generated code: {:?}", error);
        process::exit(1)
    }

    log::debug!("CLIF IR:\n{}", function.display());

    let flags = settings::Flags::new(settings::builder());
    let verify_result = verify_function(&function, &flags);
    if !verify_result.as_ref().ok().is_some() {
        log::error!("Verify error: {:?}", verify_result.err());
    } else {
        log::info!("IR verified OK");
    }

}