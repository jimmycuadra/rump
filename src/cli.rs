use std::os;

use getopts;

use commands;

struct CLI {
    flags: Vec<getopts::OptGroup>,
    help: String
}

impl CLI {
    pub fn new() -> CLI {
        let flags = vec![
            getopts::optflag("h", "help", "output usage information"),
            getopts::optflag("v", "version", "output the version number"),
            getopts::optopt("d", "delete", "delete the specified key", "KEY")
        ];

        let help = getopts::usage("Usage: rump [OPTIONS] [KEY] [VALUE]", flags.as_slice());

        CLI {
            flags: flags,
            help: help
        }
    }

    pub fn run(&self, args: &[String]) -> Option<String> {
        let options = match getopts::getopts(args, self.flags.as_slice()) {
            Ok(m) => m,
            Err(failure) => {
                os::set_exit_status(1);
                return Some(failure.to_err_msg());
            }
        };

        if options.opt_present("help") {
            return Some(self.help.clone());
        } else if options.opt_present("version") {
            return Some(commands::version());
        } else if options.opt_present("delete") {
            match options.opt_str("delete") {
                Some(key) => {
                    commands::delete(key.as_slice());
                    return None;
                },
                None => return Some("Cannot delete without a key.".to_string())
            }
        }

        match options.free.as_slice() {
            [ref key, ref value] => {
                commands::set(key.as_slice(), value.as_slice());
                return None;
            }
            [ref key] => {
                let result = commands::get(key.as_slice());

                match result {
                    Some(value) => return Some(value),
                    None => return None
                }
            }
            _ => return Some(self.help.clone())
        }
    }
}

pub fn run() {
    let args = os::args();
    let program = CLI::new();

    match program.run(args.tail()) {
        Some(output) => println!("{}", output),
        None => {}
    }
}
