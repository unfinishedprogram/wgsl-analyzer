use std::time::Duration;

use crate::front::{self};

use super::utils::time_call;

#[derive(Debug)]
pub struct BenchResult {
    pub tokenization: Duration,
    pub ast_parsing: Duration,
    pub error_count: usize,
    pub name: &'static str,
}

pub fn bench_all(source: &str, name: &'static str, repeat_times: usize) -> BenchResult {
    let source = source.repeat(repeat_times);

    let (token_result, tokenization) = time_call(|| front::ast::tokenize(&source));
    let (ast_result, ast_parsing) = time_call(|| front::ast::create_ast(&token_result));

    let error_count = ast_result.errors.len() + token_result.errors.len();

    BenchResult {
        tokenization,
        ast_parsing,
        error_count,
        name,
    }
}
