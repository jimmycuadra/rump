use std::env;

use getopts;

use commands;

struct CLI {
    options: getopts::Options,
    help: String
}

impl CLI {
    pub fn new() -> CLI {
        let mut options = getopts::Options::new();

        options.optflag("h", "help", "output usage information");
        options.optflag("v", "version", "output the version number");
        options.optopt("d", "delete", "delete the specified key", "KEY");

        let help = options.usage("Usage: rump [OPTIONS] [KEY] [VALUE]");

        CLI {
            options: options,
            help: help
        }
    }

    pub fn run(&self, args: &[String]) -> Option<String> {
        let parsed_options = match self.options.parse(args) {
            Ok(m) => m,
            Err(failure) => {
                env::set_exit_status(1);
                return Some(failure.to_err_msg());
            }
        };

        if parsed_options.opt_present("help") {
            return Some(self.help.clone());
        } else if parsed_options.opt_present("version") {
            return Some(commands::version());
        } else if parsed_options.opt_present("delete") {
            match parsed_options.opt_str("delete") {
                Some(key) => {
                    commands::delete(&key[]);
                    return None;
                },
                None => return Some("Cannot delete without a key.".to_string())
            }
        }

        match &parsed_options.free[] {
            [ref key, ref value] => {
                commands::set(&key[], &value[]);
                return None;
            }
            [ref key] => {
                let result = commands::get(&key[]);

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
    let args: Vec<String> = env::args().collect();
    let program = CLI::new();

    match program.run(args.tail()) {
        Some(output) => println!("{}", output),
        None => {}
    }
}
