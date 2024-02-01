use crate::instructions::{
    AddInstruction, BridgeInstruction, ComparisonInstruction, DivInstruction, DuplicateInstruction, Executable, FinishInstruction, GetSymbolFromSpaceInstruction, HorizontalIfInstruction, InputCharInstruction, InputIntInstruction, ModInstruction, MoveDownInstruction, MoveLeftInstruction, MoveRightInstruction, MoveUpInstruction, MulInstruction, PopValueInstruction, PrintCharInstruction, PrintIntInstruction, PutCharInstruction, PutIntInstruction, PutSymbolInSpaceInstruction, SubInstruction, SwapInstruction, SwitchStringModeInstruction, VerticalIfInstruction
};
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    static ref MAPPING: Arc<[InstructionSymbolMapping]> = get_instruction_mapping();
}

fn get_instruction_mapping() -> Arc<[InstructionSymbolMapping]> {
    Arc::from([
        InstructionSymbolMapping {
            instruction: Arc::from(MoveUpInstruction {}),
            symbol: '^',
        },
        InstructionSymbolMapping {
            instruction: Arc::from(MoveDownInstruction {}),
            symbol: 'v',
        },
        InstructionSymbolMapping {
            instruction: Arc::from(MoveLeftInstruction {}),
            symbol: '<',
        },
        InstructionSymbolMapping {
            instruction: Arc::from(MoveRightInstruction {}),
            symbol: '>',
        },
        InstructionSymbolMapping {
            instruction: Arc::from(AddInstruction {}),
            symbol: '+',
        },
        InstructionSymbolMapping {
            instruction: Arc::from(SubInstruction {}),
            symbol: '-',
        },
        InstructionSymbolMapping {
            instruction: Arc::from(MulInstruction {}),
            symbol: '*',
        },
        InstructionSymbolMapping {
            instruction: Arc::from(DivInstruction {}),
            symbol: '/',
        },
        InstructionSymbolMapping {
            instruction: Arc::from(PrintIntInstruction {}),
            symbol: '.',
        },
        InstructionSymbolMapping {
            instruction: Arc::from(PrintCharInstruction {}),
            symbol: ',',
        },
        InstructionSymbolMapping {
            instruction: Arc::from(FinishInstruction {}),
            symbol: '@',
        },
        InstructionSymbolMapping {
            instruction: Arc::from(ModInstruction {}),
            symbol: '%',
        },
        InstructionSymbolMapping {
            instruction: Arc::from(HorizontalIfInstruction {}),
            symbol: '_',
        },
        InstructionSymbolMapping {
            instruction: Arc::from(VerticalIfInstruction {}),
            symbol: '|',
        },
        InstructionSymbolMapping {
            instruction: Arc::from(BridgeInstruction {}),
            symbol: '#',
        },
        InstructionSymbolMapping {
            instruction: Arc::from(DuplicateInstruction {}),
            symbol: ':',
        },
        InstructionSymbolMapping {
            instruction: Arc::from(InputIntInstruction {}),
            symbol: '&',
        },
        InstructionSymbolMapping {
            instruction: Arc::from(InputCharInstruction {}),
            symbol: '~',
        },
        InstructionSymbolMapping {
            instruction: Arc::from(PopValueInstruction {}),
            symbol: '$',
        },
        InstructionSymbolMapping {
            instruction: Arc::from(SwapInstruction {}),
            symbol: '\\',
        },
        InstructionSymbolMapping {
            instruction: Arc::from(SwitchStringModeInstruction {}),
            symbol: '"',
        },
        InstructionSymbolMapping {
            instruction: Arc::from(ComparisonInstruction {}),
            symbol: '`',
        },
        InstructionSymbolMapping {
            instruction: Arc::from(PutSymbolInSpaceInstruction {}),
            symbol: 'p',
        },
        InstructionSymbolMapping {
            instruction: Arc::from(GetSymbolFromSpaceInstruction {}),
            symbol: 'g',
        },
    ])
}

pub fn map_symbol_to_instruction(symbol: char) -> Option<Arc<dyn Executable>> {
    let mapped_value = MAPPING
        .iter()
        .find(|mapping| mapping.symbol == symbol)
        .map(|mapping| mapping.instruction.clone());

    if mapped_value.is_some() {
        return mapped_value;
    } else if let Some(a) = symbol.to_digit(10) {
        return Some(Arc::from(PutIntInstruction(a as i32)));
    } else if symbol.is_alphabetic() {
        return Some(Arc::from(PutCharInstruction(symbol)));
    } else {
        return None;
    }
}
struct InstructionSymbolMapping {
    instruction: Arc<dyn Executable>,
    symbol: char,
}
