use std::cell::RefCell;
use std::io::{BufRead, Read, Write};
use std::rc::Rc;

pub struct MockInput {
    input: Vec<u8>,
}
pub struct MockOutput {
    output: Vec<u8>,
}

impl MockInput {
    pub fn new(input: Vec<u8>) -> Self {
        MockInput { input }
    }
    pub fn boxed_new(input: Vec<u8>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(MockInput::new(input)))
    }
}
impl MockOutput {
    pub fn new() -> Self {
        MockOutput { output: Vec::new() }
    }
    pub fn boxed_new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(MockOutput::new()))
    }
    pub fn get_stringified_output(&self) -> String {
        String::from_utf8(self.output.clone()).unwrap()
    }
}
impl Read for MockInput {
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
impl BufRead for MockInput {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        Ok(&self.input)
    }
    fn consume(&mut self, amt: usize) {
        self.input = self.input[amt..].to_vec();
    }
}

impl Write for MockOutput {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut i = 0;
        for byte in buf.iter() {
            self.output.push(*byte);
            i += 1;
        }
        Ok(i)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
