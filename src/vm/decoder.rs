use crate::vm::instruction::{
    AddInstruction, DivInstruction, EqualInstruction, FinishInstruction, Instruction,
    JumpInstruction, LessEqualInstruction, LessInstruction, LoadAbsoluteInstruction,
    LoadInstruction, MulInstruction, OutInstruction, SubInstruction,
};
use crate::vm::ARCH_BYTES;

pub fn decode(code: &[u8]) -> Box<dyn Instruction> {
    assert_eq!(code.len(), ARCH_BYTES as usize);

    let instruction_code = code[0];

    match instruction_code {
        0x01 => Box::new(AddInstruction::new(code)),
        0x02 => Box::new(SubInstruction::new(code)),
        0x03 => Box::new(MulInstruction::new(code)),
        0x04 => Box::new(DivInstruction::new(code)),
        0x05 => Box::new(JumpInstruction::new(code)),
        0x06 => Box::new(LoadInstruction::new(code)),
        0x07 => Box::new(FinishInstruction::new(code)),
        0x08 => Box::new(OutInstruction::new(code)),
        0x09 => Box::new(EqualInstruction::new(code)),
        0x0A => Box::new(LessInstruction::new(code)),
        0x0B => Box::new(LessEqualInstruction::new(code)),
        0x0C => Box::new(LoadAbsoluteInstruction::new(code)),
        _ => panic!("Invalid instruction"),
    }
}
