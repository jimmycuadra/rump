extern crate rump;

use std::process::exit;

use rump::cli;

fn main() {
    if cli::run().is_err() {
        exit(1);
    }
}
