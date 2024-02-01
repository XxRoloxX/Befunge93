use crate::mocks::{MockInput, MockOutput};
use befunge93::Interpreter;
use std::fs::read_to_string;
mod mocks;

fn generic_test(input: Vec<u8>, expected_output: &str, file_name: &str) {
    let full_path = format!("tests/befunge_programs/{}", file_name);
    let funge_space = read_to_string(full_path);
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
    generic_test(vec![b'9' as u8], "362880", "factorial.bf");
}

#[test]
fn small_factorial() {
    generic_test(vec![b'3' as u8], "6", "factorial.bf");
}

#[test]
fn hello_world_1() {
    generic_test(vec![], "Hello World!", "hello_world_1.bf");
}

#[test]
fn hello_world_2() {
    generic_test(vec![], "Hello World!", "hello_world_2.bf");
}

#[test]
fn hello_world_3() {
    generic_test(vec![], "Hello World!", "hello_world_3.bf");
}

#[test]
fn comparison_1() {
    generic_test(vec![], "1", "comparison_1.bf");
}

#[test]
fn comparison_2() {
    generic_test(vec![], "0", "comparison_2.bf");
}
#[test]
fn self_modification_1(){
    generic_test(vec![], "x", "self_modification_1.bf")
}

#[test]
fn self_modification_2(){
    generic_test(vec![], "", "self_modification_2.bf")
}
// #[test]
// fn calc_1(){
//     generic_test(vec![2, 5, b'+',b'.'], "7", "./tests/calc.bf")
// }

