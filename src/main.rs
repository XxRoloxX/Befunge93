use befunge93::cli::Cli;
use befunge93::Interpreter;
use clap::Parser;
use std::fs::read_to_string;
extern crate pretty_env_logger;

fn main() {
    pretty_env_logger::init();
    let args = Cli::parse();
    let funge_space = read_to_string(&args.program_path);
    Interpreter::new(
        &funge_space.unwrap(), 
        None,
        args.output_buffer()
        ).run();


}
