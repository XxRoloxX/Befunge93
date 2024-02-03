// Purpose: Defines the input trait for the interpreter
use std::cell::RefCell;
use std::io::{BufRead, Write, Read};
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
pub struct FileInput {
    input: Vec<u8>,
}


impl FileInput {
    pub fn new(path: &str) -> FileInput {
        FileInput {
            input: std::fs::File::create(path).unwrap().bytes().map(|x| x.unwrap()).collect(),
        }
    }
    pub fn boxed_new(path: &str) -> Rc<RefCell<FileInput>>{
        Rc::from(RefCell::from(FileInput::new(path)))
    }
}

impl BufRead for FileInput {
    fn consume(&mut self, amt: usize) {
        self.input = self.input[amt..].to_vec();
    }

    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        Ok(&self.input)
    }
}
impl Read for FileInput {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut i = 0;
        for byte in buf.iter_mut() {
            if self.input.len() == 0 {
                break;
            }
            *byte = self.input.remove(0);
            i += 1;
        }
        Ok(i)
    }
}
