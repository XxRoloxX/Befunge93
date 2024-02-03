use befunge93::cli::Cli;
use befunge93::Interpreter;
use clap::Parser;
use std::fs::read_to_string;
extern crate pretty_env_logger;
use befunge93::io::FileOutput;

fn main() {
    let args = Cli::parse();
    pretty_env_logger::init();
    let funge_space = read_to_string(&args.path);

    match funge_space {
        Ok(res) => {
            let mut funge_space =
                Interpreter::new(&res, None, Some(FileOutput::boxed_new("./output.txt")));
            funge_space.run();
        }
        Err(err) => {
            print!("{}", err);
        }
    }
}
