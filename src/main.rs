use befunge93::Interpreter;

fn main() {
    println!("Hello, world!");
    let funge_space = vec![vec!['>', '1', '2', '+', '.', '@']];

    let mut funge_space = Interpreter::new(&funge_space);
    funge_space.run();
}
