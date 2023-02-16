use crate::state::{Register, State};

pub enum Instruction {
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    LD,
    FIN,
    UNKNOWN,
}

impl From<u8> for Instruction {
    fn from(num: u8) -> Self {
        match num {
            0x1 => Instruction::ADD,
            0x2 => Instruction::SUB,
            0x3 => Instruction::MUL,
            0x4 => Instruction::DIV,
            0x5 => Instruction::JMP,
            0x6 => Instruction::LD,
            0x7 => Instruction::FIN,
            _ => Instruction::UNKNOWN,
        }
    }
}

pub trait Command {
    fn execute(&self, state: &mut State);
}

pub struct FinishCommand;

impl FinishCommand {
    pub fn new() -> Self {
        FinishCommand {}
    }
}

impl Command for FinishCommand {
    fn execute(&self, state: &mut State) {
        state.set_register(Register::END, 1);
    }
}

pub struct AddCommand;

impl AddCommand {
    pub fn new(_code: &[u8]) -> Self {
        AddCommand {}
    }
}

impl Command for AddCommand {
    fn execute(&self, _state: &mut State) {
        todo!()
    }
}
