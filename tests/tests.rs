use crate::mocks::{MockInput, MockOutput};
use befunge93::Interpreter;
use std::fs::read_to_string;
mod mocks;

fn generic_test(input: Vec<u8>, expected_output: &str, file_path: &str) {
    let funge_space = read_to_string(file_path);
    let output = MockOutput::boxed_new();
    let input = MockInput::boxed_new(input);

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
        expected_output,
        output.borrow_mut().get_stringified_output()
    );
}

#[test]
fn big_factorial() {
    generic_test(vec![b'9' as u8], "362880", "./tests/factorial.bf");
}

#[test]
fn small_factorial() {
    generic_test(vec![b'3' as u8], "6", "./tests/factorial.bf");
}

#[test]
fn hello_world_1() {
    generic_test(vec![], "Hello World!", "./tests/hello_world_1.bf");
}

#[test]
fn hello_world_2() {
    generic_test(vec![], "Hello World!", "./tests/hello_world_2.bf");
}

#[test]
fn hello_world_3() {
    generic_test(vec![], "Hello World!", "./tests/hello_world_3.bf");
}
