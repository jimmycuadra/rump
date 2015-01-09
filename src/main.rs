extern crate getopts;
extern crate "rustc-serialize" as rustc_serialize;

use std::os;

mod commands;
mod data;

fn main() {
    let args: Vec<String> = os::args();

    let flags = &[
        getopts::optflag("h", "help", "output usage information"),
        getopts::optflag("v", "version", "output the version number"),
        getopts::optopt("d", "delete", "delete the specified key", "KEY")
    ];

    let help = format!("{}", getopts::usage("Usage: rump [OPTIONS] [KEY] [VALUE]", flags));

    let options = match getopts::getopts(args.tail(), flags) {
        Ok(m) => m,
        Err(failure) => {
            println!("{}\n\n{}", failure, help);
            os::set_exit_status(1);
            return;
        }
    };

    if options.opt_present("help") {
        println!("{}", help);
        return;
    } else if options.opt_present("version") {
        println!("{}", commands::version());
        return;
    } else if options.opt_present("delete") {
        match options.opt_str("delete") {
            Some(key) => commands::delete(key.as_slice()),
            None => panic!("Cannot delete without a key.")
        }
    }

    match options.free.as_slice() {
        [ref key, ref value] => commands::set(key.as_slice(), value.as_slice()),
        [ref key] => {
            let result = commands::get(key.as_slice());

            match result {
                Some(value) => println!("{}", value),
                None => {
                    os::set_exit_status(1);
                    return;
                }
            }
        }
        _ => println!("{}", help)
    }
}
