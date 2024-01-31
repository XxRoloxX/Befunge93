// Purpose: Defines the input trait for the interpreter
use std::cell::RefCell;
use std::io::{BufRead, Write};
use std::rc::Rc;

pub type InterpreterInput = Rc<RefCell<dyn BufRead>>;
pub type InterpreterOutput = Rc<RefCell<dyn Write>>;
