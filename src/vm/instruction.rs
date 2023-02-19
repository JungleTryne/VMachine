use crate::vm::controller::Controller;
use crate::vm::state::Register;
use byteorder::{ByteOrder, LittleEndian};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InstructionType {
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    LD,
    FIN,
    OUT,
    EQ,
    LE,
    LEQ,
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
            0x9 => InstructionType::EQ,
            0xA => InstructionType::LE,
            0xB => InstructionType::LEQ,
            _ => InstructionType::UNKNOWN,
        }
    }
}

/// # Trait *Instruction*
/// Represents instruction of the virtual machine
/// [execute] method is responsible for executing the instruction
pub trait Instruction {
    fn execute(&self, controller: &mut Controller);
}

/// # FinishInstruction
/// Final instruction that stops the execution of the virtual machine
/// Sets value of the [END] register to 1, thus stops the pipeline.
///
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
        controller.mut_state().set_register(Register::END, 1);
    }
}

/// # AddInstruction
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
        let state = controller.mut_state();
        let first_value = state.register(self.first_register);
        let second_value = state.register(self.second_register);
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
        let state = controller.mut_state();
        let first_value = state.register(self.first_register);
        let second_value = state.register(self.second_register);
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
        let state = controller.mut_state();
        let first_value = state.register(self.first_register);
        let second_value = state.register(self.second_register);
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
        let state = controller.mut_state();

        let first_value = state.register(self.first_register);
        let second_value = state.register(self.second_register);

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
/// Prints string the address of which is stored in
/// a given register
///
/// Structure
/// - 1st byte: instruction code
/// - 2nd byte: register with address to the string
/// - 3rd byte: not used
/// - 4th byte: not used
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
        let mut address = controller.state().register(self.register);
        loop {
            let char = controller
                .state()
                .get_memory_handler()
                .read_byte(address) as char;
            if char == '\0' {
                break;
            }
            controller.display().print(char);
            address += 1;
        }
    }
}

/// # LoadInstruction
/// Loads value from address [ip + offset],
/// [offset] is parsed as i16 little endian
///
/// Structure:
/// - 1st byte: instruction code
/// - 2nd byte: destination [register]
/// - 3rd byte: offset
/// - 4th byte: offset
///
/// Result is stored in [register]
pub struct LoadInstruction {
    register: Register,
    offset: i16,
}

impl LoadInstruction {
    pub fn new(code: &[u8]) -> Self {
        assert_eq!(
            InstructionType::from_byte(code[0]),
            InstructionType::LD,
            "Invalid code of instruction"
        );

        LoadInstruction {
            register: Register::from_addr(code[1] as u32),
            offset: LittleEndian::read_i16(&code[2..=3]),
        }
    }
}

impl Instruction for LoadInstruction {
    fn execute(&self, controller: &mut Controller) {
        let ip_value = controller.state().register(Register::IP);
        let address = ip_value + self.offset as u32;

        let value = controller
            .mut_state()
            .get_memory_handler()
            .read_byte(address) as u32;

        controller
            .mut_state()
            .set_register(self.register, value);
    }
}

/// # JumpInstruction
/// Moves [IP] register by the [offset]
/// [offset] is parsed as little endian i16
///
/// Structure:
/// - 1st byte: instruction code
/// - 2nd byte: offset
/// - 3rd byte: offset
/// - 4th byte: not used
pub struct JumpInstruction {
    offset: i16,
}

impl JumpInstruction {
    pub fn new(code: &[u8]) -> Self {
        assert_eq!(
            InstructionType::from_byte(code[0]),
            InstructionType::JMP,
            "Invalid code of instruction"
        );

        JumpInstruction {
            offset: LittleEndian::read_i16(&code[1..=2]),
        }
    }
}

impl Instruction for JumpInstruction {
    fn execute(&self, controller: &mut Controller) {
        let ip_value = controller.state().register(Register::IP);
        let address = ip_value + self.offset as u32;
        controller
            .mut_state()
            .set_register(Register::IP, address);
    }
}

pub struct EqualInstruction {
    left: Register,
    right: Register,
}

impl EqualInstruction {
    pub fn new(code: &[u8]) -> Self {
        assert_eq!(
            InstructionType::from_byte(code[0]),
            InstructionType::EQ,
            "Invalid code of instruction"
        );

        EqualInstruction {
            left: Register::from_addr(code[1] as u32),
            right: Register::from_addr(code[2] as u32),
        }
    }
}

impl Instruction for EqualInstruction {
    fn execute(&self, controller: &mut Controller) {
        let left_value = controller.state().register(self.left);
        let right_value = controller.state().register(self.right);
        controller
            .mut_state()
            .set_register(Register::CMP, (left_value == right_value) as u32);
    }
}

pub struct LessInstruction {
    left: Register,
    right: Register,
}

impl LessInstruction {
    pub fn new(code: &[u8]) -> Self {
        assert_eq!(
            InstructionType::from_byte(code[0]),
            InstructionType::LE,
            "Invalid code of instruction"
        );

        LessInstruction {
            left: Register::from_addr(code[1] as u32),
            right: Register::from_addr(code[2] as u32),
        }
    }
}

impl Instruction for LessInstruction {
    fn execute(&self, controller: &mut Controller) {
        let left_value = controller.state().register(self.left);
        let right_value = controller.state().register(self.right);
        controller
            .mut_state()
            .set_register(Register::CMP, (left_value < right_value) as u32);
    }
}

pub struct LessEqualInstruction {
    left: Register,
    right: Register,
}

impl LessEqualInstruction {
    pub fn new(code: &[u8]) -> Self {
        assert_eq!(
            InstructionType::from_byte(code[0]),
            InstructionType::LE,
            "Invalid code of instruction"
        );

        LessEqualInstruction {
            left: Register::from_addr(code[1] as u32),
            right: Register::from_addr(code[2] as u32),
        }
    }
}

impl Instruction for LessEqualInstruction {
    fn execute(&self, controller: &mut Controller) {
        let left_value = controller.state().register(self.left);
        let right_value = controller.state().register(self.right);
        controller
            .mut_state()
            .set_register(Register::CMP, (left_value <= right_value) as u32);
    }
}
