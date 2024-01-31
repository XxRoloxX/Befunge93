use befunge93::Interpreter;
use std::cell::RefCell;
use std::fs::read_to_string;
use std::io::{BufRead, Read, Write};
use std::rc::Rc;

struct MockInput {
    input: Vec<u8>,
}
struct MockOutput {
    output: Vec<u8>,
}

impl MockInput {
    fn new(input: Vec<u8>) -> Self {
        MockInput { input }
    }
    fn boxed_new(input: Vec<u8>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(MockInput::new(input)))
    }
}
impl MockOutput {
    fn new() -> Self {
        MockOutput { output: Vec::new() }
    }
    fn boxed_new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(MockOutput::new()))
    }
}
impl Read for MockInput {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut i = 0;
        for byte in self.input.iter() {
            buf[i] = *byte;
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

#[test]
fn big_factorial() {
    let funge_space = read_to_string("./tests/factorial.bf");
    let output = MockOutput::boxed_new();
    let input = MockInput::boxed_new(vec![b'9' as u8]);

    match funge_space {
        Ok(res) => {
            let mut interpreter = Interpreter::new(&res, Some(input), Some(output.clone()));
            interpreter.run();
        }
        Err(err) => {
            print!("{}", err);
        }
    }
    assert_eq!(
        "362880",
        String::from_utf8(output.borrow_mut().output.clone()).unwrap()
    );
}

#[test]
fn small_factorial() {
    let funge_space = read_to_string("./tests/factorial.bf");
    let output = MockOutput::boxed_new();
    let input = MockInput::boxed_new(vec![b'3' as u8]);

    match funge_space {
        Ok(res) => {
            let mut interpreter = Interpreter::new(&res, Some(input), Some(output.clone()));
            interpreter.run();
        }
        Err(err) => {
            print!("{}", err);
        }
    }
    assert_eq!(
        "6",
        String::from_utf8(output.borrow_mut().output.clone()).unwrap()
    );
}

#[test]
fn hello_world_1() {
    let funge_space = read_to_string("./tests/hello_world_1.bf");
    let output = MockOutput::boxed_new();
    let input = MockInput::boxed_new(vec![]);

    match funge_space {
        Ok(res) => {
            let mut interpreter = Interpreter::new(&res, Some(input), Some(output.clone()));
            interpreter.run();
        }
        Err(err) => {
            print!("{}", err);
        }
    }
    assert_eq!(
        "Hello World!",
        String::from_utf8(output.borrow_mut().output.clone()).unwrap()
    );
}
