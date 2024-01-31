use crate::pointer::Direction;
use crate::Interpreter;
use crate::StackValue;
use crate::Stackable;
use std::io::Read;

use self::stack_operations::convert_empty_stack_value_to_default_int;

pub trait Executable: Sync +Send + 'static {
    fn execute(&self, interpreter: &mut Interpreter);
}

mod stack_operations {
    use crate::{Interpreter, StackValue, Stackable};

    pub fn convert_empty_stack_value_to_default_int(value: StackValue)->StackValue{
        match value{
            StackValue::Empty => StackValue::Int(0),
            StackValue::Int(a) => StackValue::Int(a),
            StackValue::Char(a) => StackValue::Int(a as i32)
        }
    }

    pub fn binary_arithmetic_operation_on_stack<F>(interpreter: &mut Interpreter, operation: F)
    where
        F: Fn(i32, i32) -> i32,
    {
        let (mut a,mut b) = interpreter.get_stack().get_two_items_from_stack();
        a = convert_empty_stack_value_to_default_int(a);
        b = convert_empty_stack_value_to_default_int(b);

        if let (StackValue::Int(a), StackValue::Int(b)) = (a, b) {
            interpreter
                .get_stack()
                .push(StackValue::Int(operation(a, b)));
        };
    }
}

mod pointer_operations {
    use crate::pointer::Direction;
    use crate::Interpreter;

    pub fn change_pointer_direction<'a>(interpreter: &'a mut Interpreter, direction: Direction) {
        interpreter.pointer.change_direction(direction);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AddInstruction {}

#[derive(Debug, Clone, Copy)]
pub struct SubInstruction {}

#[derive(Debug, Clone, Copy)]
pub struct DivInstruction {}

#[derive(Debug, Clone, Copy)]
pub struct MulInstruction {}

#[derive(Debug, Clone, Copy)]
pub struct ModInstruction {}

#[derive(Debug, Clone, Copy)]
pub struct PrintCharInstruction {}

#[derive(Debug, Clone, Copy)]
pub struct PrintIntInstruction {}

#[derive(Debug, Clone, Copy)]
pub struct FinishInstruction {}

#[derive(Debug, Clone, Copy)]
pub struct PutIntInstruction(pub i32);

#[derive(Debug, Clone, Copy)]
pub struct PutCharInstruction(pub char);

#[derive(Debug, Clone, Copy)]
pub struct MoveUpInstruction {}

#[derive(Debug, Clone, Copy)]
pub struct MoveDownInstruction {}

#[derive(Debug, Clone, Copy)]
pub struct MoveLeftInstruction {}

#[derive(Debug, Clone, Copy)]
pub struct MoveRightInstruction {}

#[derive(Debug, Clone, Copy)]
pub struct HorizontalIfInstruction {}

#[derive(Debug, Clone, Copy)]
pub struct VerticalIfInstruction {}

#[derive(Debug, Clone, Copy)]
pub struct BridgeInstruction {}

#[derive(Debug, Clone, Copy)]
pub struct DuplicateInstruction{}

#[derive(Debug, Clone,Copy)]
pub struct InputIntInstruction{}

#[derive(Debug, Clone,Copy)]
pub struct InputCharInstruction{}

#[derive(Debug, Clone,Copy)]
pub struct PopValueInstruction{}

#[derive(Debug, Clone,Copy)]
pub struct SwapInstruction{}


impl Executable for AddInstruction {
    fn execute(&self, interpreter: &mut Interpreter) {
        stack_operations::binary_arithmetic_operation_on_stack(interpreter, |a, b| a + b);
    }
}

impl Executable for ModInstruction {
    fn execute(&self, interpreter: &mut Interpreter) {
        stack_operations::binary_arithmetic_operation_on_stack(interpreter, |a, b| a%b);
    }
}

impl Executable for SubInstruction {
    fn execute(&self, interpreter: &mut Interpreter) {
        stack_operations::binary_arithmetic_operation_on_stack(interpreter, |a, b| b - a);
    }
}
impl Executable for DivInstruction {
    fn execute(&self, interpreter: &mut Interpreter) {
        stack_operations::binary_arithmetic_operation_on_stack(interpreter, |a, b| a / b);
    }
}
impl Executable for MulInstruction {
    fn execute(&self, interpreter: &mut Interpreter) {
        stack_operations::binary_arithmetic_operation_on_stack(interpreter, |a, b| a * b);
    }
}
impl Executable for PrintCharInstruction {
    fn execute(&self, interpreter: &mut Interpreter) {
        match interpreter.get_stack().remove_value_from_stack() {
            StackValue::Int(val) => print!("{}", val as u8),
            StackValue::Char(val) => print!("{}", val),
            StackValue::Empty => print!("Stack is Empty"),
        }
    }
}
impl Executable for PrintIntInstruction {
    fn execute(&self, interpreter: &mut Interpreter) {
        match interpreter.get_stack().remove_value_from_stack() {
            StackValue::Char(c) => print!("{}", c as u8),
            StackValue::Int(i) => print!("{}", i),
            StackValue::Empty => print!("Empty"),
        }
    }
}

impl Executable for FinishInstruction {
    fn execute(&self, interpreter: &mut Interpreter) {
        interpreter.finish_execution();
    }
}

impl Executable for BridgeInstruction {
    fn execute(&self, interpreter: &mut Interpreter) {
        interpreter.pointer.current_move();
    }
}


impl Executable for HorizontalIfInstruction {
    fn execute(&self, interpreter: &mut Interpreter) {
        let top = interpreter.get_stack().remove_value_from_stack();
        match top {
            StackValue::Int(val) if val != 0 => {
                pointer_operations::change_pointer_direction(interpreter, Direction::Left);
            },
            StackValue::Char(val) if val as u8 != 0 => {
                pointer_operations::change_pointer_direction(interpreter, Direction::Left);
            },
            _ => {
                pointer_operations::change_pointer_direction(interpreter, Direction::Right);
            }
        }
    }
}


impl Executable for VerticalIfInstruction {
    fn execute(&self, interpreter: &mut Interpreter) {
        let top = interpreter.get_stack().remove_value_from_stack();
        match top {
            StackValue::Int(val) if val != 0 => {
                pointer_operations::change_pointer_direction(interpreter, Direction::Up);
            },
            StackValue::Char(val) if val as u8 != 0 => {
                pointer_operations::change_pointer_direction(interpreter, Direction::Up);
            },
            _ => {
                pointer_operations::change_pointer_direction(interpreter, Direction::Down);
            }
        }
    }
}


impl Executable for PutIntInstruction {
    fn execute(&self, interpreter: &mut Interpreter) {
        interpreter.get_stack().push(StackValue::Int(self.0))
    }
}

impl Executable for PutCharInstruction {
    fn execute(&self, interpreter: &mut Interpreter) {
        interpreter.get_stack().push(StackValue::Char(self.0))
    }
}

impl Executable for MoveUpInstruction {
    fn execute<'a>(&self, interpreter: &'a mut Interpreter) {
        pointer_operations::change_pointer_direction(interpreter, Direction::Up);
    }
}

impl Executable for MoveDownInstruction {
    fn execute<'a>(&self, interpreter: &'a mut Interpreter) {
        pointer_operations::change_pointer_direction(interpreter, Direction::Down);
    }
}

impl Executable for MoveRightInstruction {
    fn execute<'a>(&self, interpreter: &'a mut Interpreter) {
        pointer_operations::change_pointer_direction(interpreter, Direction::Right);
    }
}

impl Executable for MoveLeftInstruction {
    fn execute<'a>(&self, interpreter: &'a mut Interpreter) {
        pointer_operations::change_pointer_direction(interpreter, Direction::Left);
    }
}

impl Executable for DuplicateInstruction {
    fn execute<'a>(&self, interpreter: &'a mut Interpreter) {
        let top = interpreter.get_stack().remove_value_from_stack();
        interpreter.get_stack().push(top);
        interpreter.get_stack().push(top);
    }
}

impl Executable for InputIntInstruction {
    fn execute<'a>(&self, interpreter: &'a mut Interpreter) {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim().parse::<i32>().unwrap();
        interpreter.get_stack().push(StackValue::Int(input));
    }
}
impl Executable for InputCharInstruction {
    fn execute<'a>(&self, interpreter: &'a mut Interpreter) {
        let input = std::io::stdin()
            .bytes()
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte as char);
        interpreter.get_stack().push(StackValue::Char(input.unwrap()));
    }
}


impl Executable for PopValueInstruction {
    fn execute<'a>(&self, interpreter: &'a mut Interpreter) {
        interpreter.get_stack().remove_value_from_stack();
    }
}
impl Executable for SwapInstruction {
    fn execute<'a>(&self, interpreter: &'a mut Interpreter) {
        let (a, b) = interpreter.get_stack().get_two_items_from_stack();
        interpreter.get_stack().push(convert_empty_stack_value_to_default_int(a));
        interpreter.get_stack().push(convert_empty_stack_value_to_default_int(b));
    }
}


