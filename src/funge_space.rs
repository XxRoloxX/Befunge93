#[derive(Debug)]
pub struct FungeSpace {
    pub plain: Vec<Vec<char>>,
    pub height: usize,
    pub width: usize,
}


fn format_string_to_vec_of_vecs(plain: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for line in plain.lines() {
        let mut line_vec = Vec::new();
        for c in line.chars() {
            line_vec.push(c);
        }
        result.push(line_vec);
    }
    result
}


impl FungeSpace {
    pub fn new(plain: &str) -> FungeSpace {
        let formated_plain = format_string_to_vec_of_vecs(plain);
        let width = formated_plain.first().map(|e| e.len()).unwrap_or(0);
        let height = formated_plain.len();
        FungeSpace {
            plain: formated_plain,
            width,
            height
        }
    }

    pub fn get_symbol_at(&self, x: usize, y: usize) -> char {
        self.plain[y][x]
    }
}
