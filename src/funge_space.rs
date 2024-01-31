#[derive(Debug)]
pub struct FungeSpace {
    pub plain: [[char; 25]; 80],
    pub height: usize,
    pub width: usize,
}

fn format_string_to_matrix(plain: &str) -> [[char; 25]; 80] {
    let mut matrix = [[' '; 25]; 80];
    for (row, line) in plain.lines().enumerate() {
        for (col, character) in line.chars().enumerate() {
            matrix[row][col] = character;
        }
    }
    return matrix;
}

impl FungeSpace {
    pub fn new(plain: &str) -> FungeSpace {
        let matrix = format_string_to_matrix(plain);
        let width = 25;
        let height = 80;
        FungeSpace {
            plain: matrix,
            width,
            height,
        }
    }

    pub fn get_symbol_at(&self, x: usize, y: usize) -> char {
        self.plain[y][x]
    }
}
