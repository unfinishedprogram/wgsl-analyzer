use std::fs;

use ariadne::{ColorGenerator, Label, Report, Source};
use wgsl_ast::front::ast::{create_ast, tokenize};
fn main() {
    let paths = fs::read_dir("src/test_files")
        .unwrap()
        .flat_map(|f| f.ok())
        .flat_map(|entry| {
            if entry.file_type().unwrap().is_file() {
                Some(entry.path())
            } else {
                None
            }
        })
        .map(|path| {
            (
                path.to_str().unwrap().to_owned(),
                fs::read_to_string(path).unwrap(),
            )
        });

    for (path, source) in paths {
        let token_result = tokenize(&source);
        let ast_result = create_ast(&token_result);
        println!("{:#?}", ast_result);

        let mut colors = ColorGenerator::new();

        for err in ast_result.errors {
            Report::build(ariadne::ReportKind::Error, &path, err.span().start)
                .with_label(
                    Label::new((&path, err.span().into_range()))
                        .with_message(err.message())
                        .with_color(colors.next()),
                )
                .finish()
                .print((&path, Source::from(&source)))
                .unwrap();
        }
    }
}
