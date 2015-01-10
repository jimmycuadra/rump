#![allow(unstable)]

extern crate rump;

use std::os;
use rump::cli::CLI;

fn main() {
    let args = os::args();
    let program = CLI::new();

    match program.run(args.tail()) {
        Ok(result) => match result {
            Some(output) => println!("{}", output),
            None => {}
        },
        Err(error) => {
            match error {
                Some(error_message) => println!("{}", error_message),
                None => {}
            }

            os::set_exit_status(1);
        }
    }
}
