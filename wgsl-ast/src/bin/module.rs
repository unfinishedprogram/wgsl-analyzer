use ariadne::Source;
use wgsl_ast::{
    front::ast::{create_ast, tokenize},
    module::Module,
};

fn main() {
    let source = include_str!("test.wgsl");
    let token_result = tokenize(source);
    let ast_result = create_ast(&token_result);
    let module = Module::from_ast(ast_result);

    match module {
        Ok(_) => println!("Module is valid"),
        Err(err) => {
            err.build_report("test.wgsl")
                .print(("test.wgsl", Source::from(source)))
                .unwrap();
        }
    }
}
