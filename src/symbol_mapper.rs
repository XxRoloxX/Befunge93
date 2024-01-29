use crate::Instruction;

pub fn map_symbol_to_instruction(symbol: char) -> Option<Instruction> {
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

struct InstructionSymbolMapping {
    instruction: Instruction,
    symbol: char,
}
