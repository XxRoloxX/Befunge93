use crate::instructions::{
    AddInstruction, DivInstruction, FinishInstruction, Instruction, MoveDownInstruction,
    MoveLeftInstruction, MoveRightInstruction, MoveUpInstruction, MulInstruction,
    PrintCharInstruction, PrintIntInstruction, PutCharInstruction, PutIntInstruction,
    SubInstruction, ModInstruction, HorizontalIfInstruction, VerticalIfInstruction,
    BridgeInstruction, DuplicateInstruction , InputIntInstruction, InputCharInstruction
};

pub fn map_symbol_to_instruction(symbol: char) -> Option<Instruction> {

    let mapping = vec![
        InstructionSymbolMapping {
            instruction: Instruction::MoveUp(MoveUpInstruction {}),
            symbol: '^',
        },
        InstructionSymbolMapping {
            instruction: Instruction::MoveDown(MoveDownInstruction {}),
            symbol: 'v',
        },
        InstructionSymbolMapping {
            instruction: Instruction::MoveLeft(MoveLeftInstruction {}),
            symbol: '<',
        },
        InstructionSymbolMapping {
            instruction: Instruction::MoveRight(MoveRightInstruction {}),
            symbol: '>',
        },
        InstructionSymbolMapping {
            instruction: Instruction::Add(AddInstruction {}),
            symbol: '+',
        },
        InstructionSymbolMapping {
            instruction: Instruction::Sub(SubInstruction {}),
            symbol: '-',
        },
        InstructionSymbolMapping {
            instruction: Instruction::Mul(MulInstruction {}),
            symbol: '*',
        },
        InstructionSymbolMapping {
            instruction: Instruction::Div(DivInstruction {}),
            symbol: '/',
        },
        InstructionSymbolMapping {
            instruction: Instruction::PrintInt(PrintIntInstruction {}),
            symbol: '.',
        },
        InstructionSymbolMapping {
            instruction: Instruction::PrintChar(PrintCharInstruction {}),
            symbol: ',',
        },
        InstructionSymbolMapping {
            instruction: Instruction::Finish(FinishInstruction {}),
            symbol: '@',
        },
        InstructionSymbolMapping{
            instruction: Instruction::Mod(ModInstruction{}),
            symbol: '%'
        },
        InstructionSymbolMapping{
            instruction: Instruction::HorizontalIf(HorizontalIfInstruction{}),
            symbol: '_'
        },
        InstructionSymbolMapping{
            instruction: Instruction::VerticalIf(VerticalIfInstruction{}),
            symbol: '|'
        },
        InstructionSymbolMapping{
            instruction: Instruction::Bridge(BridgeInstruction{}),
            symbol: '#'
        },
        InstructionSymbolMapping{
            instruction: Instruction::Duplicate(DuplicateInstruction{}),
            symbol: ':'
        },
        InstructionSymbolMapping{
            instruction: Instruction::InputInt(InputIntInstruction{}),
            symbol: '&'
        },
        InstructionSymbolMapping{
            instruction: Instruction::InputChar(InputCharInstruction{}),
            symbol: '~'
        },
        
    ];

    let mapped_value = mapping
        .iter()
        .find(move |mapping| mapping.symbol == symbol)
        .map(|mapping| mapping.instruction);

    if mapped_value.is_some(){
        return mapped_value;
    }else if let Some(a) = symbol.to_digit(10) {
        return Some(Instruction::PutInt(PutIntInstruction(a as i32)));
    }
    else if symbol.is_alphabetic(){
        return Some(Instruction::PutChar(PutCharInstruction(symbol)));
    }else{
        return None
    }

}

struct InstructionSymbolMapping {
    instruction: Instruction,
    symbol: char,
}
