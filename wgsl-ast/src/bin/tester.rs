use ariadne::Source;
use wgsl_ast::front::ast::{create_ast, tokenize};

fn main() {
    let source = include_str!("test.wgsl");
    let token_result = tokenize(source);
    let ast_result = create_ast(&token_result);

    println!("{:#?}", ast_result);

    for err in ast_result.errors {
        err.build_report("test.wgsl")
            .print(("test.wgsl", Source::from(source)))
            .unwrap();
    }
}
