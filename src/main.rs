extern crate jba;

use std::env;
use std::process;

use jba::analyze_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Bad arguments! Exactly one argument (a class file name) is expected.");
        process::exit(1);
    }

    let file_name = &args[1];

    let result = analyze_file(file_name);

    if result.is_err() {
        eprintln!("{}", result.unwrap_err());
        process::exit(2)
    }
}
