use crate::vm::arch::instruction::{
    AddInstruction, DivInstruction, EqualInstruction, FinishInstruction, Instruction,
    JumpInstruction, LessEqualInstruction, LessInstruction, LoadAbsoluteInstruction,
    LoadInstruction, MulInstruction, OutInstruction, SubInstruction,
};

use crate::vm::utils::instruction_macro::register_instructions;

register_instructions! {
    0x01 => AddInstruction,
    0x02 => SubInstruction,
    0x03 => MulInstruction,
    0x04 => DivInstruction,
    0x05 => JumpInstruction,
    0x06 => LoadInstruction,
    0x07 => FinishInstruction,
    0x08 => OutInstruction,
    0x09 => EqualInstruction,
    0x0A => LessInstruction,
    0x0B => LessEqualInstruction,
    0x0C => LoadAbsoluteInstruction
}
