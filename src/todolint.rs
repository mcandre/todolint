//! CLI kirill tool

extern crate die;
extern crate getopts;
extern crate todolint;

use die::{Die, die};
use std::env;
use std::path;

/// CLI entrypoint
fn main() {
    let brief: String = format!(
        "usage: {} [OPTIONS] <path> [<path> [<path> ...]]",
        env!("CARGO_PKG_NAME")
    );

    let mut opts: getopts::Options = getopts::Options::new();
    opts.optflag("d", "debug", "enable additional logging");
    opts.optflag("h", "help", "print usage info");
    opts.optflag("v", "version", "print version info");

    let usage: String = opts.usage(&brief);
    let arguments: Vec<String> = env::args().collect();
    let optmatches: getopts::Matches = opts.parse(&arguments[1..]).die(&usage);

    if optmatches.opt_present("h") {
        die!(0; usage);
    }

    if optmatches.opt_present("v") {
        die!(0; format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
    }

    let mut linter = todolint::Linter::default();

    if optmatches.opt_present("d") {
        linter.debug = true;
    }

    let rest_args = optmatches.free;

    if rest_args.is_empty() {
        die!(usage);
    }

    let roots: Vec<&path::Path> = rest_args.iter().map(path::Path::new).collect();
    let warnings_result = linter.scan(roots);

    if let Err(e) = warnings_result {
        die!(1; format!("error: {e}"));
    }

    let warnings = warnings_result.unwrap();

    if warnings.is_empty() {
        die!(0);
    }

    for warning in warnings {
        println!("{}", warning);
    }

    die!(1);
}
