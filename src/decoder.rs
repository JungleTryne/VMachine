use crate::command::{AddCommand, Command, FinishCommand, Instruction};

pub fn decode(code: &[u8]) -> Box<dyn Command> {
    assert_eq!(code.len(), 4);

    let coded_command = Instruction::from(code[0]);

    match coded_command {
        Instruction::ADD => Box::new(AddCommand::new(code)),
        Instruction::FIN => Box::new(FinishCommand::new()),
        _ => panic!("Invalid instruction"),
    }
}
