mod spec;
mod testrunner;

use spec::{parse_command_line, TestSpec};
use testrunner::run_tests;

fn main() {
    let mut spec = parse_command_line();
    print_header(&spec);
    print_results(run_tests(&mut spec.command, spec.iterations, spec.outliers));
}

fn print_header(spec: &TestSpec) {
    println!("Repeat \"{:?}\" {} times ignoring {} outliers",
             spec.command,
             spec.iterations,
             spec.outliers);
}

fn print_results(result: (i64, i64)) {
    let (total, average) = result;
    println!("It took {} milliseconds in total", total);
    println!("It took {} milliseconds on average", average);
}

