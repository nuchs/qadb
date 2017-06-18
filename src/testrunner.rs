extern crate time;

use std::process::Command;
use self::time::PreciseTime;

pub fn run_tests(command: &mut Command, iterations: u32, outliers: u32) -> (i64, i64) {
    let mut times = Vec::with_capacity(iterations as usize);

    for _ in 0..iterations {
        times.push(time_test(command));
    }

    let total = calculate_total_time(&mut times, iterations, outliers);
    let average = calculate_average_time(total, iterations, outliers);

    (total, average)
}

fn time_test(command: &mut Command) -> i64 {
    let start = PreciseTime::now();
    command.spawn().expect("Failed to run test");
    let end = PreciseTime::now();

    start.to(end).num_milliseconds()
}

fn calculate_total_time(times: &mut Vec<i64>, iterations: u32, outliers: u32) -> i64 {
    times.sort();
    times
        .iter()
        .skip(outliers as usize)
        .take((iterations - outliers) as usize)
        .sum()
}

fn calculate_average_time(total: i64, iterations: u32, outliers: u32) -> i64 {
    total / (iterations - (2 * outliers)) as i64
}
