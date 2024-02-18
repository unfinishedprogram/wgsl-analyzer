use wgsl_ast::bench::parser::bench_all;

const REPEAT_TIMES: usize = 1;

pub fn main() {
    println!("Starting!");

    println!(
        "{:?}",
        bench_all(
            include_str!("../bench/source_examples/top.wgsl"),
            "Stress",
            REPEAT_TIMES
        )
    );

    println!(
        "{:?}",
        bench_all(include_str!("./test.wgsl"), "Normal Use", REPEAT_TIMES)
    );
}
