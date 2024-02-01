use crate::{FungeSpace, InterpreterInput, InterpreterOutput, Pointer, Stack, StackValue};
use std::cell::RefCell;
use std::io::{stdin, stdout};
use std::io::{BufReader, BufWriter};
use std::rc::Rc;

pub struct Interpreter {
    space: FungeSpace,
    stack: Stack<StackValue>,
    mode: ReadMode,
    is_running: bool,
    pointer: Pointer,
    input: InterpreterInput,
    output: InterpreterOutput,
}
pub enum ReadMode {
    String,
    Normal,
}

impl Interpreter {
    pub fn new(
        plain: &str,
        input: Option<InterpreterInput>,
        output: Option<InterpreterOutput>,
    ) -> Interpreter {
        Interpreter {
            pointer: Pointer::new(),
            space: FungeSpace::new(plain),
            stack: Stack::new(),
            mode: ReadMode::Normal,
            is_running: true,
            input: input.unwrap_or(Rc::from(RefCell::from(BufReader::new(stdin())))),
            output: output.unwrap_or(Rc::from(RefCell::from(BufWriter::new(stdout())))),
        }
    }

    pub fn get_space(&mut self) -> &mut FungeSpace {
        &mut self.space
    }
    pub fn get_immutable_space(&self)->&FungeSpace {
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

    pub fn finish_execution(&mut self) {
        self.is_running = false;
    }
    pub fn execute_current_instruction(&mut self) {
        let instruction = self.pointer.get_current_instruction(self);

        if let Some(instruction) = instruction {
            instruction.execute(self);
        }
    }

    pub fn run(&mut self) {
        while self.is_running {
            self.pointer.wrap_pointer(&self.space);
            self.execute_current_instruction();
            print!(
                "Position: ({:?}), , Stack: {:?}\n",
                self.pointer, self.stack
            );
            self.pointer.current_move();
        }
    }
}
