use befunge93::Interpreter;
use std::fs::read_to_string;

fn main() {
    let funge_space = read_to_string("./src/factorial.bf");

    match funge_space {
        Ok(res) => {
            let mut funge_space = Interpreter::new(&res, None, None);
            funge_space.run();
        }
        Err(err) => {
            print!("{}", err);
        }
    }
}
