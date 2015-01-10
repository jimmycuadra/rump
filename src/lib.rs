extern crate getopts;
extern crate "rustc-serialize" as rustc_serialize;

mod commands;
mod data;

pub fn call(args: &[String]) -> Result<Option<String>, String> {
    let flags = &[
        getopts::optflag("h", "help", "output usage information"),
        getopts::optflag("v", "version", "output the version number"),
        getopts::optopt("d", "delete", "delete the specified key", "KEY")
    ];

    let help = format!("{}", getopts::usage("Usage: rump [OPTIONS] [KEY] [VALUE]", flags));

    let options = match getopts::getopts(args, flags) {
        Ok(m) => m,
        Err(failure) => {
            return Err(failure.to_err_msg());
        }
    };

    if options.opt_present("help") {
        return Ok(Some(help));
    } else if options.opt_present("version") {
        return Ok(Some(commands::version()));
    } else if options.opt_present("delete") {
        match options.opt_str("delete") {
            Some(key) => {
                commands::delete(key.as_slice());
                return Ok(None);
            },
            None => return Err("Cannot delete without a key.".to_string())
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
        _ => return Ok(Some(help))
    }
}
