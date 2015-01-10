#![allow(unstable)]

extern crate rump;

use std::os;
use rump::cli::CLI;

fn main() {
    let args = os::args();
    let program = CLI::new();

    match program.run(args.tail()) {
        Some(output) => println!("{}", output),
        None => {}
    }
}
