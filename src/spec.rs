extern crate clap;

use self::clap::{Arg, ArgMatches, App, AppSettings};
use std::process::{Command, Stdio};

pub struct TestSpec {
    pub iterations: u32,
    pub outliers: u32,
    pub command: Command,
}

pub fn parse_command_line() -> TestSpec {
    let matches = App::new("qadb")
        .version("0.1")
        .author("Nuchs. <sjbrown@live.co.uk>")
        .about("Quick and dirty benchmarking tool")
        .setting(AppSettings::TrailingVarArg)
        .arg(
            Arg::with_name("iterations")
                .short("i")
                .long("iterations")
                .help("Number of time to repeat bench mark test")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("outliers")
                .short("o")
                .long("outliers")
                .help("Remove the 'n' fastest and slowest results")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("test")
                .help("command to be benchmarked")
                .multiple(true),
        )
        .get_matches();

    TestSpec::new(&matches)
}

impl TestSpec {
    fn new(matches: &ArgMatches) -> TestSpec {
        let iterations = TestSpec::get_iterations(matches);
        let mut outliers = TestSpec::get_outliers(matches);

        if (2 * outliers) >= iterations {
            outliers = (iterations - 1) / 2;
        }

        TestSpec {
            iterations: iterations,
            outliers: outliers,
            command: TestSpec::get_command(matches),
        }
    }

    fn get_iterations(matches: &ArgMatches) -> u32 {
        matches
            .value_of("iterations")
            .unwrap_or("1")
            .trim()
            .parse()
            .expect("Invalid number of iterations")
    }

    fn get_outliers(matches: &ArgMatches) -> u32 {
        matches
            .value_of("outliers")
            .unwrap_or("0")
            .trim()
            .parse()
            .expect("Invalid number of outliers")
    }

    fn get_command(matches: &ArgMatches) -> Command {
        let test: Vec<&str> = matches.values_of("test").unwrap().collect();
        let args = &test[1..];
        let mut command = Command::new(test[0]);
        command
            .args(args)
            .stderr(Stdio::null())
            .stdout(Stdio::null());

        command
    }
}
