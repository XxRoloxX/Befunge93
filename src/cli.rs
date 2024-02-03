use clap::Parser;
use crate::{io::FileOutput, InterpreterOutput};


#[derive(Parser)]
pub struct Cli {
    pub program_path: std::path::PathBuf,
    // to the output file
    #[arg(short = 'o', long = "output")]
    pub output_path: Option<std::path::PathBuf>,
    //Path to the input file
    #[arg(short = 'i', long = "input")]
    pub input_path: Option<std::path::PathBuf>,
}
impl Cli {
    pub fn output_buffer(&self) -> Option<InterpreterOutput> {
        match &self.output_path {
        Some(path) => Some(FileOutput::boxed_new(path.to_str().unwrap())),
        None => None,
    }
}
}

