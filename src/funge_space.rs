use std::io::Write;
const FUNGE_SPACE_WIDTH: usize = 80; // By definition, the maximum size of a funge space is 80x25
const FUNGE_SPACE_HEIGHT: usize = 25;

#[derive(Debug)]
pub struct FungeSpace {
    pub plain: [[char; FUNGE_SPACE_WIDTH]; FUNGE_SPACE_HEIGHT],
    pub height: usize,
    pub width: usize,
}

fn format_string_to_matrix(plain: &str) -> [[char; FUNGE_SPACE_WIDTH]; FUNGE_SPACE_HEIGHT] {
    let mut matrix = [[' '; FUNGE_SPACE_WIDTH]; FUNGE_SPACE_HEIGHT];
    for (row, line) in plain.lines().enumerate() {
        for (col, character) in line.chars().enumerate() {
            matrix[row][col] = character;
        }
    }
    return matrix;
}
fn format_matrix_to_string(matrix: [[char; FUNGE_SPACE_WIDTH]; FUNGE_SPACE_HEIGHT]) -> String {
    let mut plain = String::new();
    for row in matrix.iter() {
        for character in row.iter() {
            plain.push(*character);
        }
        plain.push('\n');
    }
    return plain;
}

impl FungeSpace {
    pub fn new(plain: &str) -> FungeSpace {
        let matrix = format_string_to_matrix(plain);
        let width = FUNGE_SPACE_WIDTH;
        let height = FUNGE_SPACE_HEIGHT;
        FungeSpace {
            plain: matrix,
            width,
            height,
        }
    }

    pub fn get_symbol_at(&self, x: usize, y: usize) -> char {
        self.plain[y][x]
    }

    pub fn set_symbol_at(&mut self, x:usize, y:usize, symbol:char){
        self.plain[y][x] = symbol;
    }

}
