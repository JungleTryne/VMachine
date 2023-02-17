use crate::vm::controller::Controller;
use crate::vm::state::Register;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InstructionType {
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    OUT,
    LD,
    FIN,
    UNKNOWN,
}

impl InstructionType {
    pub fn from_byte(num: u8) -> Self {
        match num {
            0x1 => InstructionType::ADD,
            0x2 => InstructionType::SUB,
            0x3 => InstructionType::MUL,
            0x4 => InstructionType::DIV,
            0x5 => InstructionType::JMP,
            0x6 => InstructionType::LD,
            0x7 => InstructionType::FIN,
            0x8 => InstructionType::OUT,
            _ => InstructionType::UNKNOWN,
        }
    }
}

/// # Trait *Instruction*
/// Represents instruction of the architecture
/// [execute] method is responsible for changing the state
/// of the machine, that is why it is given the mutable
/// reference to it
pub trait Instruction {
    fn execute(&self, controller: &mut Controller);
}

/// # FinishInstruction
/// Final instruction that stops the execution of the virtual machine
/// Sets value of the [END] register to 1, thus stops the pipeline
/// Structure
/// - 1st byte: instruction code
/// - 2nd byte: not used
/// - 3rd byte: not used
/// - 4th byte: not used
pub struct FinishInstruction;

impl FinishInstruction {
    pub fn new(code: &[u8]) -> Self {
        assert_eq!(
            InstructionType::from_byte(code[0]),
            InstructionType::FIN,
            "Invalid code of instruction"
        );
        FinishInstruction {}
    }
}

impl Instruction for FinishInstruction {
    fn execute(&self, controller: &mut Controller) {
        controller.get_mut_state().set_register(Register::END, 1);
    }
}

/// # AddInstruction
///
/// Sums two numbers in given registers
///
/// Structure:
/// - 1st byte: instruction code
/// - 2nd byte: address of the [first_register]
/// - 3rd byte: address of the [second_register]
/// - 4th byte: not used
///
/// Result is stored in the [first_register]
pub struct AddInstruction {
    first_register: Register,
    second_register: Register,
}

impl AddInstruction {
    pub fn new(code: &[u8]) -> Self {
        assert_eq!(
            InstructionType::from_byte(code[0]),
            InstructionType::ADD,
            "Invalid code of instruction"
        );
        AddInstruction {
            first_register: Register::from_addr(code[1] as u32),
            second_register: Register::from_addr(code[2] as u32),
        }
    }
}

impl Instruction for AddInstruction {
    fn execute(&self, controller: &mut Controller) {
        let state = controller.get_mut_state();
        let first_value = state.get_register(self.first_register);
        let second_value = state.get_register(self.second_register);
        let result = first_value + second_value;
        state.set_register(self.first_register, result);
    }
}

/// # SubInstruction
///
/// Subtracts two numbers in given registers
///
/// Structure:
/// - 1st byte: instruction code
/// - 2nd byte: address of the [first_register]
/// - 3rd byte: address of the [second_register]
/// - 4th byte: not used
///
/// Result is stored in the [first_register]
pub struct SubInstruction {
    first_register: Register,
    second_register: Register,
}

impl SubInstruction {
    pub fn new(code: &[u8]) -> Self {
        assert_eq!(
            InstructionType::from_byte(code[0]),
            InstructionType::SUB,
            "Invalid code of instruction"
        );
        SubInstruction {
            first_register: Register::from_addr(code[1] as u32),
            second_register: Register::from_addr(code[2] as u32),
        }
    }
}

impl Instruction for SubInstruction {
    fn execute(&self, controller: &mut Controller) {
        let state = controller.get_mut_state();
        let first_value = state.get_register(self.first_register);
        let second_value = state.get_register(self.second_register);
        let result = first_value - second_value;
        state.set_register(self.first_register, result);
    }
}

/// # MulInstruction
///
/// Multiplies two numbers in given registers
///
/// Structure:
/// - 1st byte: instruction code
/// - 2nd byte: address of the [first_register]
/// - 3rd byte: address of the [second_register]
/// - 4th byte: not used
///
/// Result is stored in the [first_register]
pub struct MulInstruction {
    first_register: Register,
    second_register: Register,
}

impl MulInstruction {
    pub fn new(code: &[u8]) -> Self {
        assert_eq!(
            InstructionType::from_byte(code[0]),
            InstructionType::MUL,
            "Invalid code of instruction"
        );
        MulInstruction {
            first_register: Register::from_addr(code[1] as u32),
            second_register: Register::from_addr(code[2] as u32),
        }
    }
}

impl Instruction for MulInstruction {
    fn execute(&self, controller: &mut Controller) {
        let state = controller.get_mut_state();
        let first_value = state.get_register(self.first_register);
        let second_value = state.get_register(self.second_register);
        let result = first_value * second_value;
        state.set_register(self.first_register, result);
    }
}

/// # DivInstruction
///
/// Divides two *integer* numbers in given registers
///
/// Structure:
/// - 1st byte: instruction code
/// - 2nd byte: address of the [first_register]
/// - 3rd byte: address of the [second_register]
/// - 4th byte: not used
///
/// Result of division is stored in the [first_register],
/// remainder is stored in the [second_register]
pub struct DivInstruction {
    first_register: Register,
    second_register: Register,
}

impl DivInstruction {
    pub fn new(code: &[u8]) -> Self {
        assert_eq!(
            InstructionType::from_byte(code[0]),
            InstructionType::DIV,
            "Invalid code of instruction"
        );
        DivInstruction {
            first_register: Register::from_addr(code[1] as u32),
            second_register: Register::from_addr(code[2] as u32),
        }
    }
}

impl Instruction for DivInstruction {
    fn execute(&self, controller: &mut Controller) {
        let state = controller.get_mut_state();

        let first_value = state.get_register(self.first_register);
        let second_value = state.get_register(self.second_register);

        assert_ne!(
            second_value, 0,
            "DivInstruction failure: second register = 0"
        );
        let result = first_value / second_value;
        let reminder = first_value % second_value;

        state.set_register(self.first_register, result);
        state.set_register(self.second_register, reminder);
    }
}

/// # OutInstruction
/// Prints character that is stored in a given register
pub struct OutInstruction {
    register: Register,
}

impl OutInstruction {
    pub fn new(code: &[u8]) -> Self {
        assert_eq!(
            InstructionType::from_byte(code[0]),
            InstructionType::OUT,
            "Invalid code of instruction"
        );
        OutInstruction {
            register: Register::from_addr(code[1] as u32),
        }
    }
}

impl Instruction for OutInstruction {
    fn execute(&self, controller: &mut Controller) {
        let state = controller.get_mut_state();
        let char_value =
            char::from_u32(state.get_register(self.register)).expect("Invalid char in register");

        let display = controller.get_mut_display();
        display.print(char_value);
    }
}
