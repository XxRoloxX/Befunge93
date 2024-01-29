use crate::funge_space::FungeSpace;
use crate::symbol_mapper::map_symbol_to_instruction as mapper;
use crate::instructions::Instruction;

#[derive(Debug)]
pub struct Pointer<'a> {
    x: i32,
    y: i32,
    direction: Direction,
    space: &'a FungeSpace,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl<'a> Pointer<'a> {
    pub fn new(space: &'a FungeSpace) -> Pointer<'a> {
        Pointer {
            x: 0,
            y: 0,
            direction: Direction::Right,
            space,
        }
    }
    pub fn move_vertically(&mut self, steps: i32) {
        self.y = (self.y + steps) % self.space.height as i32;
    }
    pub fn move_horizontally(&mut self, steps: i32) {
        self.x = (self.x + steps) % self.space.width as i32;
    }

    pub fn change_direction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::MoveUp(_) => self.direction = Direction::Up,
            Instruction::MoveDown(_) => self.direction = Direction::Down,
            Instruction::MoveLeft(_) => self.direction = Direction::Left,
            Instruction::MoveRight(_) => self.direction = Direction::Right,
            _ => {}
        }
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
    pub fn get_current_instruction(&mut self) -> Option<Instruction> {
        return mapper(self.space.get_symbol_at(self.x as usize, self.y as usize));
    }
}
