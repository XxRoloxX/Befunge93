pub struct FungeSpace {
    plain: Vec<Vec<char>>,
    width: usize,
    height: usize,
    direction: Direction,
    stack: Stack<StackValue>,
    is_running: bool,
    x: i32,
    y: i32,
}
#[derive(Debug, Clone, Copy)]
enum StackValue {
    Int(i32),
    Char(char),
    Empty,
}
#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn map_symbol_to_instruction(symbol: char) -> Option<Instruction> {
    if let Some(a) = symbol.to_digit(10) {
        return Some(Instruction::PutInt(a as i32));
    }
    if symbol.is_alphabetic() {
        return Some(Instruction::PutChar(symbol));
    }

    let mapping = vec![
        InstructionSymbolMapping {
            instruction: Instruction::MoveUp,
            symbol: '^',
        },
        InstructionSymbolMapping {
            instruction: Instruction::MoveDown,
            symbol: 'v',
        },
        InstructionSymbolMapping {
            instruction: Instruction::MoveLeft,
            symbol: '<',
        },
        InstructionSymbolMapping {
            instruction: Instruction::MoveRight,
            symbol: '>',
        },
        InstructionSymbolMapping {
            instruction: Instruction::Add,
            symbol: '+',
        },
        InstructionSymbolMapping {
            instruction: Instruction::Sub,
            symbol: '-',
        },
        InstructionSymbolMapping {
            instruction: Instruction::Mul,
            symbol: '*',
        },
        InstructionSymbolMapping {
            instruction: Instruction::Div,
            symbol: '/',
        },
        InstructionSymbolMapping {
            instruction: Instruction::PrintInt,
            symbol: '.',
        },
        InstructionSymbolMapping {
            instruction: Instruction::PrintChar,
            symbol: ',',
        },
        InstructionSymbolMapping {
            instruction: Instruction::Finish,
            symbol: '@',
        },
    ];
    mapping
        .iter()
        .find(move |mapping| mapping.symbol == symbol)
        .map(|mapping| mapping.instruction)
}

#[derive(Debug)]
struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stackable<T> for Stack<T> {
    fn push(&mut self, val: T) {
        self.stack.push(val)
    }
    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
}

trait Stackable<T> {
    fn push(&mut self, val: T);
    fn pop(&mut self) -> Option<T>;
}

struct InstructionSymbolMapping {
    instruction: Instruction,
    symbol: char,
}

impl FungeSpace {
    pub fn new(plain: Vec<Vec<char>>) -> FungeSpace {
        let width = plain[0].len();
        let height = plain.len();
        FungeSpace {
            plain,
            width,
            height,
            stack: Stack { stack: Vec::new() },
            x: 0,
            y: 0,
            is_running: true,
            direction: Direction::Right,
        }
    }
    fn move_vertically(&mut self, steps: i32) {
        self.y = (self.y + steps) % self.height as i32;
    }
    fn move_horizontally(&mut self, steps: i32) {
        self.x = (self.x + steps) % self.width as i32;
    }

    fn change_direction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::MoveUp => self.direction = Direction::Up,
            Instruction::MoveDown => self.direction = Direction::Down,
            Instruction::MoveLeft => self.direction = Direction::Left,
            Instruction::MoveRight => self.direction = Direction::Right,
            _ => {}
        }
    }

    fn current_move(&mut self) {
        match self.direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }
    }

    fn push_to_stack(&mut self, value: StackValue) {
        self.stack.push(value);
    }

    fn finish_execution(&mut self) {
        self.is_running = false;
    }

    fn get_current_instruction(&mut self) -> Option<Instruction> {
        return map_symbol_to_instruction(self.plain[self.y as usize][self.x as usize]);
    }

    fn move_up(&mut self) {
        self.move_vertically(-1);
    }

    fn move_down(&mut self) {
        self.move_vertically(1);
    }

    fn move_left(&mut self) {
        self.move_horizontally(-1);
    }

    fn move_right(&mut self) {
        self.move_horizontally(1);
    }

    fn remove_value_from_stack(&mut self) -> StackValue {
        match self.stack.pop() {
            Some(a) => a,
            None => StackValue::Empty,
        }
    }

    pub fn run(&mut self) {
        while self.is_running {
            if let Some(instruction) = self.get_current_instruction() {
                instruction.execute(self);
            }
            print!(
                "Position: ({}, {}), Direction: {:?}, Stack: {:?}\n",
                self.x, self.y, self.direction, self.stack.stack
            );
            self.current_move();
        }
    }

    fn remove_two_elements_from_stack(&mut self) -> (StackValue, StackValue) {
        let top = self.remove_value_from_stack();
        let next_to_top = self.remove_value_from_stack();
        (top, next_to_top)
    }

    fn binary_arithmetic_operation_on_stack<F>(&mut self, operation: F)
    where
        F: Fn(i32, i32) -> i32,
    {
        let (a, b) = self.remove_two_elements_from_stack();
        if let (StackValue::Int(a), StackValue::Int(b)) = (a, b) {
            self.stack.push(StackValue::Int(operation(a, b)));
        };
    }

    fn add(&mut self) {
        let adding = |a, b| a + b;
        self.binary_arithmetic_operation_on_stack(adding)
    }

    fn sub(&mut self) {
        let subtracting = |a, b| a - b;
        self.binary_arithmetic_operation_on_stack(subtracting)
    }

    fn mul(&mut self) {
        let multiplying = |a, b| a * b;
        self.binary_arithmetic_operation_on_stack(multiplying)
    }

    fn div(&mut self) {
        let dividing = |a, b| a / b;
        self.binary_arithmetic_operation_on_stack(dividing)
    }

    fn print_char(&mut self) {
        let a = self.stack.pop();
        if let Some(a) = a {
            match a {
                StackValue::Char(c) => print!("{}", c),
                StackValue::Int(i) => print!("{}", i as u8 as char),
                StackValue::Empty => print!("Empty"),
            }
        }
    }
    fn print_int(&mut self) {
        let a = self.stack.pop();
        if let Some(a) = a {
            match a {
                StackValue::Char(c) => print!("{}", c as u8),
                StackValue::Int(i) => print!("{}", i),
                StackValue::Empty => print!("Empty"),
            }
        }
    }
}

impl Instruction {
    fn execute(&self, space: &mut FungeSpace) {
        match self {
            Instruction::MoveUp => space.change_direction(Instruction::MoveUp),
            Instruction::MoveDown => space.change_direction(Instruction::MoveDown),
            Instruction::MoveLeft => space.change_direction(Instruction::MoveLeft),
            Instruction::MoveRight => space.change_direction(Instruction::MoveRight),
            Instruction::Add => space.add(),
            Instruction::Sub => space.sub(),
            Instruction::Mul => space.mul(),
            Instruction::Div => space.div(),
            Instruction::PrintChar => space.print_char(),
            Instruction::PrintInt => space.print_int(),
            Instruction::Finish => space.finish_execution(),
            Instruction::PutInt(a) => space.push_to_stack(StackValue::Int(*a)),
            Instruction::PutChar(a) => space.push_to_stack(StackValue::Char(*a)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    MoveUp,
    PutInt(i32),
    PutChar(char),
    MoveDown,
    MoveLeft,
    MoveRight,
    Add,
    Sub,
    Mul,
    Div,
    PrintChar,
    PrintInt,
    Finish,
}
