use crate::vm::instruction::{
    AddInstruction, FinishInstruction, Instruction, InstructionType, JumpInstruction,
    LoadInstruction, MulInstruction, OutInstruction, SubInstruction,
};
use crate::vm::ARCH_BYTES;

pub fn decode(code: &[u8]) -> Box<dyn Instruction> {
    assert_eq!(code.len(), ARCH_BYTES as usize);

    let coded_command = InstructionType::from_byte(code[0]);

    match coded_command {
        InstructionType::ADD => Box::new(AddInstruction::new(code)),
        InstructionType::SUB => Box::new(SubInstruction::new(code)),
        InstructionType::MUL => Box::new(MulInstruction::new(code)),
        InstructionType::DIV => Box::new(SubInstruction::new(code)),
        InstructionType::FIN => Box::new(FinishInstruction::new(code)),
        InstructionType::OUT => Box::new(OutInstruction::new(code)),
        InstructionType::LD => Box::new(LoadInstruction::new(code)),
        InstructionType::JMP => Box::new(JumpInstruction::new(code)),
        _ => panic!("Invalid instruction"),
    }
}
