extern crate rump;

use std::os;

fn main() {
    let args: Vec<String> = os::args();

    match rump::call(args.tail()) {
        Ok(result) => match result {
            Some(output) => println!("{}", output),
            None => {}
        },
        Err(error) => {
            println!("{}", error);

            os::set_exit_status(1);
        }
    }
}
