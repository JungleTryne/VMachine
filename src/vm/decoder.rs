use crate::vm::command::{
    AddInstruction, FinishInstruction, Instruction, InstructionType, MulInstruction,
    OutInstruction, SubInstruction,
};

pub fn decode(code: &[u8]) -> Box<dyn Instruction> {
    assert_eq!(code.len(), 4);

    let coded_command = InstructionType::from_byte(code[0]);

    match coded_command {
        InstructionType::ADD => Box::new(AddInstruction::new(code)),
        InstructionType::SUB => Box::new(SubInstruction::new(code)),
        InstructionType::MUL => Box::new(MulInstruction::new(code)),
        InstructionType::DIV => Box::new(SubInstruction::new(code)),
        InstructionType::FIN => Box::new(FinishInstruction::new(code)),
        InstructionType::OUT => Box::new(OutInstruction::new(code)),
        _ => panic!("Invalid instruction"),
    }
}
