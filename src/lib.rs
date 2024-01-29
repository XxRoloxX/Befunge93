pub mod funge_space;
pub mod instructions;
pub mod pointer;
pub mod symbol_mapper;

use funge_space::FungeSpace;
use pointer::Pointer;
use instructions::Instruction;

pub struct Interpreter<'a> {
    space: FungeSpace,
    stack: Stack<StackValue>,
    is_running: bool,
    pointer: Pointer<'a>,
}

#[derive(Debug, Clone, Copy)]
enum StackValue {
    Int(i32),
    Char(char),
    Empty,
}

#[derive(Debug)]
struct Stack<T> {
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

impl<'a> Interpreter<'a> {
    pub fn new(plain: Vec<Vec<char>>) -> Interpreter<'a> {
        let space = FungeSpace::new(plain);
        Interpreter {
            space,
            stack: Stack { stack: Vec::new() },
            is_running: true,
            pointer: Pointer::new(&space),
        }
    }

    pub fn get_space(&self) -> &FungeSpace {
        &self.space
    }
    pub fn get_pointer(&self) -> &Pointer {
        &self.pointer
    }
    pub fn get_stack(&mut self) -> &mut Stack<StackValue> {
        &mut self.stack
    }

    fn finish_execution(&mut self) {
        self.is_running = false;
    }

    pub fn run(&mut self) {
        while self.is_running {
            if let Some(instruction) = self.pointer.get_current_instruction() {
                instruction.execute(self);
            }
            print!(
                "Position: ({:?}), , Stack: {:?}\n",
                self.get_pointer(), self.stack.stack
            );
            self.get_pointer().current_move();
        }
    }
}
