use befunge93::Interpreter;
use std::fs::read_to_string;
use befunge93::cli::Cli;
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let funge_space = read_to_string(&args.path);

    match funge_space {
        Ok(res) => {
            let mut funge_space = Interpreter::new(&res, None, None);
            funge_space.run();
        }
        Err(err) => {
            print!("{}", err);
        }
    }
}
