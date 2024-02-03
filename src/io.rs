// Purpose: Defines the input trait for the interpreter
use std::cell::RefCell;
use std::io::{BufRead, Write};
use std::rc::Rc;

pub type InterpreterInput = Rc<RefCell<dyn BufRead>>;
pub type InterpreterOutput = Rc<RefCell<dyn Write>>;


pub struct FileOutput {
    file: std::fs::File,
}

impl FileOutput {
    pub fn new(path: &str) -> FileOutput {
        FileOutput {
            file: std::fs::File::create(path).unwrap(),
        }
    }
    pub fn boxed_new(path: &str) -> Rc<RefCell<FileOutput>>{
        Rc::from(RefCell::from(FileOutput {
            file: std::fs::File::create(path).unwrap(),
        }))
    }
}
impl Write for FileOutput {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.file.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}
