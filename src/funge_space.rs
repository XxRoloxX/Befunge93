#[derive(Debug)]
pub struct FungeSpace {
    pub plain: Vec<Vec<char>>,
    pub height: usize,
    pub width: usize,
}

impl FungeSpace {
    pub fn new(plain: Vec<Vec<char>>) -> FungeSpace {
        FungeSpace {
            plain,
            width: plain[0].len(),
            height: plain.len(),
        }
    }

    pub fn get_symbol_at(&self, x: usize, y: usize) -> char {
        self.plain[x][y]
    }
}
