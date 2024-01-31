// Purpose: Defines the input trait for the interpreter
use std::io::{BufRead, Write};
use std::rc::Rc;
use std::cell::RefCell;
//
// pub trait InterpreterInput: Read{
//
// }
pub type InterpreterInput = Rc<RefCell<dyn BufRead>>;
pub type InterpreterOutput = Rc<RefCell<dyn Write>>;

// pub type InterpreterInput = Arc<dyn BufRead>;
// pub type InterpreterOutput = Arc<dyn Write>;
