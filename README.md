# Qadb (Quick and dirty bencher)

Very simple tool for quickly checking how long a command takes to run. Can run the command multiple times and omit the n highest and lowest results before calculating the mean run time.

## Help

USAGE:
    qadb [OPTIONS] [test]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --iterations <iterations>    Number of time to repeat bench mark test
    -o, --outliers <outliers>        Remove the 'n' fastest and slowest results

ARGS:
    <test>...    command to be benchmarked
