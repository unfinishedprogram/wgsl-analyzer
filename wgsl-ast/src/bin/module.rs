use wgsl_ast::module::Module;

fn main() {
    let source = include_str!("test.wgsl");
    let module = Module::from_source(source);

    match module {
        Ok(_) => println!("Module is valid"),
        Err(_) => {
            println!("Failed");
        }
    }
}
