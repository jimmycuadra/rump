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

    pub fn run(&self, args: &[String]) -> Result<Option<String>, getopts::Fail> {
        let parsed_options = try!(self.options.parse(args));

        if parsed_options.opt_present("help") {
            return Ok(Some(self.help.clone()));
        } else if parsed_options.opt_present("version") {
            return Ok(Some(commands::version()));
        } else if parsed_options.opt_present("delete") {
            match parsed_options.opt_str("delete") {
                Some(key) => {
                    commands::delete(&key[..]);

                    return Ok(None);
                },
                None => return Ok(Some("Cannot delete without a key.".to_string())),
            }
        }


        match parsed_options.free.len() {
            2 => {
                commands::set(&parsed_options.free[0], &parsed_options.free[1]);

                Ok(None)
            },
            1 => {
                match commands::get(&parsed_options.free[0]) {
                    Some(value) => Ok(Some(value)),
                    None => Ok(None),
                }
            },
            _ => Ok(Some(self.help.clone())),
        }
    }
}

pub fn run() -> Result<(), getopts::Fail> {
    let args: Vec<String> = env::args().skip(1).collect();
    let program = CLI::new();

    match program.run(&args) {
        Ok(option) => {
            if option.is_some() {
                println!("{}", option.unwrap());
            }

            Ok(())
        },
        Err(error) => Err(error),
    }
}
