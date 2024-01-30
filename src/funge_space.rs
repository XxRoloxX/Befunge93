#[derive(Debug)]
pub struct FungeSpace<'a> {
    pub plain: &'a Vec<Vec<char>>,
    pub height: usize,
    pub width: usize,
}

impl<'a> FungeSpace<'a> {
    pub fn new(plain: &'a Vec<Vec<char>>) -> FungeSpace<'a> {
        FungeSpace {
            plain,
            width: plain[0].len(),
            height: plain.len(),
        }
    }

    pub fn get_symbol_at(&self, x: usize, y: usize) -> char {
        self.plain[y][x]
    }
}
