use getopts;

use commands;

pub struct CLI {
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

    pub fn run(&self, args: &[String]) -> Result<Option<String>, Option<String>> {
        let options = match getopts::getopts(args, self.flags.as_slice()) {
            Ok(m) => m,
            Err(failure) => {
                return Err(Some(failure.to_err_msg()));
            }
        };

        if options.opt_present("help") {
            return Ok(Some(self.help.clone()));
        } else if options.opt_present("version") {
            return Ok(Some(commands::version()));
        } else if options.opt_present("delete") {
            match options.opt_str("delete") {
                Some(key) => {
                    commands::delete(key.as_slice());
                    return Ok(None);
                },
                None => return Err(Some("Cannot delete without a key.".to_string()))
            }
        }

        match options.free.as_slice() {
            [ref key, ref value] => {
                commands::set(key.as_slice(), value.as_slice());
                return Ok(None);
            }
            [ref key] => {
                let result = commands::get(key.as_slice());

                match result {
                    Some(value) => return Ok(Some(value)),
                    None => return Ok(None)
                }
            }
            _ => return Ok(Some(self.help.clone()))
        }
    }
}
