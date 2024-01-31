pub mod funge_space;
pub mod instructions;
pub mod pointer;
pub mod symbol_mapper;
pub mod io;


pub use funge_space::FungeSpace;
pub use pointer::Pointer;
pub use io::{InterpreterInput, InterpreterOutput};
use std::io::{stdin, stdout};
use std::io::{BufReader, BufWriter};
use std::rc::Rc;
use std::cell::RefCell;

pub struct Interpreter{
    space: FungeSpace,
    stack: Stack<StackValue>,
    mode: ReadMode,
    is_running: bool,
    pointer: Pointer,
    input: InterpreterInput,
    output: InterpreterOutput,
}

#[derive(Debug, Clone, Copy)]
pub enum StackValue {
    Int(i32),
    Char(char),
    Empty,
}


impl StackValue {
    pub fn is_empty(&self)->bool{
        match self{
            StackValue::Empty=>true,
            _ => false
        }
    }
}

#[derive(Debug)]
pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stackable<T> for Stack<T> {
    fn push(&mut self, val: T) {
        self.stack.push(val)
    }
    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
}

trait Stackable<T> {
    fn push(&mut self, val: T);
    fn pop(&mut self) -> Option<T>;
}

impl Stack<StackValue> {
    pub fn remove_value_from_stack(&mut self) -> StackValue {
        match self.stack.pop() {
            Some(a) => a,
            None => StackValue::Empty,
        }
    }
    pub fn get_two_items_from_stack(&mut self) -> (StackValue, StackValue) {
        let top = self.remove_value_from_stack();
        let next_to_top = self.remove_value_from_stack();
        (top, next_to_top)
    }
}
pub enum ReadMode {
    String,
    Normal
}

impl Interpreter {
    pub fn new(plain: &str, input: Option<InterpreterInput>, output:Option<InterpreterOutput>) -> Interpreter {
        Interpreter {
            pointer: Pointer::new(),
            space: FungeSpace::new(plain),
            stack: Stack { stack: Vec::new() },
            mode: ReadMode::Normal,
            is_running: true,
            input: input.unwrap_or(Rc::from(RefCell::from(BufReader::new(stdin())))),
            output: output.unwrap_or(Rc::from(RefCell::from(BufWriter::new(stdout())))),
        }

    }

    pub fn get_space(&self) -> &FungeSpace {
        &self.space
    }
    pub fn get_pointer(&mut self) -> &mut Pointer {
        &mut self.pointer
    }
    pub fn get_stack(&mut self) -> &mut Stack<StackValue> {
        &mut self.stack
    }
    pub fn get_input(&mut self) -> &mut InterpreterInput {
        &mut self.input
    }
    pub fn get_output(&mut self) -> &mut InterpreterOutput {
        &mut self.output
    }
    pub fn get_mode(&self) -> &ReadMode {
        &self.mode
    }
    pub fn set_mode(&mut self, mode: ReadMode) {
        self.mode = mode;
    }

    fn finish_execution(&mut self) {
        self.is_running = false;
    }
    fn execute_current_instruction(&mut self) {
        let instruction = self.pointer.get_current_instruction(self);
        if let Some(instruction) = instruction {
            instruction.execute(self);
        }
    }

    pub fn run(&mut self) {
        while self.is_running {
            self.execute_current_instruction();
            print!(
                    "Position: ({:?}), , Stack: {:?}\n",
                    self.pointer,
                    self.stack.stack
                );
            self.pointer.current_move();
            self.pointer.wrap_pointer(&self.space);
        }
    }
}




