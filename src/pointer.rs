use crate::funge_space::FungeSpace;
use crate::instructions::Instruction;
use crate::symbol_mapper::map_symbol_to_instruction as mapper;

#[derive(Debug)]
pub struct Pointer {
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Pointer {
    pub fn new() -> Pointer {
        Pointer {
            x: 0,
            y: 0,
            direction: Direction::Right,
        }
    }

    pub fn move_vertically(&mut self, steps: i32) {
        self.y = (self.y as i32 + steps) as usize;
    }
    pub fn move_horizontally(&mut self, steps: i32) {
        self.x = (self.x as i32 + steps) as usize;
    } 
    pub fn wrap_pointer(&mut self, space: &FungeSpace){
       self.y = self.y % space.height as usize;
       self.x = self.x % space.width as usize;
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn current_move(&mut self) {
        match self.direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }
    }

    pub fn move_up(&mut self) {
        self.move_vertically(-1);
    }

    pub fn move_down(&mut self) {
        self.move_vertically(1);
    }

    pub fn move_left(&mut self) {
        self.move_horizontally(-1);
    }

    pub fn move_right(&mut self) {
        self.move_horizontally(1);
    }
    pub fn get_current_instruction(&self, space: &FungeSpace) -> Option<Instruction> {
        return mapper(space.get_symbol_at(self.x as usize, self.y as usize));
    }
}
