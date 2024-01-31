use std::fs::read_to_string;

use befunge93::Interpreter;

fn main() {
    // let funge_space = vec![vec!['>', '1', '2', '+', '.', '@']];
    // let funge_space = "v12>+.@\n>)4^".to_owned();
    let funge_space = read_to_string("./src/test.bf");

    match funge_space {
        Ok(res) => {
            let mut funge_space = Interpreter::new(&res);
            funge_space.run();
        }
        Err(err) => {
            print!("{}", err);
        }
    }
}
