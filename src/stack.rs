#[derive(Debug, Clone, Copy)]
pub enum StackValue {
    Int(i32),
    Char(char),
    Empty,
}

impl StackValue {
    pub fn is_empty(&self) -> bool {
        match self {
            StackValue::Empty => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Stack<T> {
    stack: Vec<T>,
}

pub trait Stackable<T> {
    fn push(&mut self, val: T);
    fn pop(&mut self) -> Option<T>;
}

impl<T> Stackable<T> for Stack<T> {
    fn push(&mut self, val: T) {
        self.stack.push(val)
    }
    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
}

impl Stack<StackValue> {
    pub fn new() -> Stack<StackValue> {
        Stack { stack: Vec::new() }
    }
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
