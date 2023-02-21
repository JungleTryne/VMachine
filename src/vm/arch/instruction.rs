use crate::vm::components::controller::Controller;
use crate::vm::components::state::Register;
use byteorder::{ByteOrder, LittleEndian};

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
    0x0C => LoadAbsoluteInstruction,
    0x0D => InputInstruction,
    0x0E => JumpCompareInstruction,
    0x0F => JumpNotCompareInstruction,
    0x10 => OutFromRegisterInstruction,
    0x11 => SkipInstruction,
    0x12 => OutNumberInstruction,
    0x13 => MoveInstruction
}

/// # Trait *Instruction*
/// Represents instruction of the virtual machine.
/// [execute] method is responsible for executing the instruction.
/// [move_ip] returns true, if after instruction execution the ip
/// register needs to be incremented.
pub trait Instruction {
    fn execute(&mut self, controller: &mut Controller);
    fn move_ip(&self) -> bool {
        true
    }
}

/// # AddInstruction
/// Sums two numbers in given registers
///
/// Structure:
/// - 1st byte: instruction code
/// - 2nd byte: [first_register] address
/// - 3rd byte: [second_register] address
/// - 4th byte: [third_register] address
///
/// Result is stored in the [third_register]
///
pub struct AddInstruction {
    first_register: Register,
    second_register: Register,
    third_register: Register,
}

impl AddInstruction {
    pub fn new(code: &[u8]) -> Self {
        AddInstruction {
            first_register: Register::from_addr(code[1] as u32),
            second_register: Register::from_addr(code[2] as u32),
            third_register: Register::from_addr(code[3] as u32),
        }
    }
}

impl Instruction for AddInstruction {
    fn execute(&mut self, controller: &mut Controller) {
        let state = controller.mut_state();
        let first_value = state.register(self.first_register);
        let second_value = state.register(self.second_register);
        let (result, _overflow_flag) = first_value.overflowing_add(second_value);
        state.set_register(self.third_register, result);
    }
}

/// # SubInstruction
///
/// Subtracts two numbers in given registers
///
/// Structure:
/// - 1st byte: instruction code
/// - 2nd byte: [first_register] address
/// - 3rd byte: [second_register] address
/// - 4th byte: [third_register] address
///
/// Result is stored in the [third_register]
///
pub struct SubInstruction {
    first_register: Register,
    second_register: Register,
    third_register: Register,
}

impl SubInstruction {
    pub fn new(code: &[u8]) -> Self {
        SubInstruction {
            first_register: Register::from_addr(code[1] as u32),
            second_register: Register::from_addr(code[2] as u32),
            third_register: Register::from_addr(code[3] as u32),
        }
    }
}

impl Instruction for SubInstruction {
    fn execute(&mut self, controller: &mut Controller) {
        let state = controller.mut_state();
        let first_value = state.register(self.first_register);
        let second_value = state.register(self.second_register);
        let (result, _overflow_flag) = first_value.overflowing_sub(second_value);
        state.set_register(self.third_register, result);
    }
}

/// # MulInstruction
///
/// Multiplies two numbers in given registers
///
/// Structure:
/// - 1st byte: instruction code
/// - 2nd byte: [first_register] address
/// - 3rd byte: [second_register] address
/// - 4th byte: [third_register] address
///
/// Result is stored in the [third_register]
///
pub struct MulInstruction {
    first_register: Register,
    second_register: Register,
    third_register: Register,
}

impl MulInstruction {
    pub fn new(code: &[u8]) -> Self {
        MulInstruction {
            first_register: Register::from_addr(code[1] as u32),
            second_register: Register::from_addr(code[2] as u32),
            third_register: Register::from_addr(code[3] as u32),
        }
    }
}

impl Instruction for MulInstruction {
    fn execute(&mut self, controller: &mut Controller) {
        let state = controller.mut_state();
        let first_value = state.register(self.first_register);
        let second_value = state.register(self.second_register);
        let (result, _overflow_flag) = first_value.overflowing_mul(second_value);
        state.set_register(self.third_register, result);
    }
}

/// # DivInstruction
///
/// Divides two *integer* numbers in given registers
///
/// Structure:
/// - 1st byte: instruction code
/// - 2nd byte: [first_register] address
/// - 3rd byte: [second_register] address
/// - 4th byte: [third_register] address
///
/// Result is stored in the [third_register]
///
pub struct DivInstruction {
    first_register: Register,
    second_register: Register,
    third_register: Register,
}

impl DivInstruction {
    pub fn new(code: &[u8]) -> Self {
        DivInstruction {
            first_register: Register::from_addr(code[1] as u32),
            second_register: Register::from_addr(code[2] as u32),
            third_register: Register::from_addr(code[3] as u32),
        }
    }
}

impl Instruction for DivInstruction {
    fn execute(&mut self, controller: &mut Controller) {
        let state = controller.mut_state();

        let first_value = state.register(self.first_register);
        let second_value = state.register(self.second_register);

        assert_ne!(
            second_value, 0,
            "DivInstruction failure: second register = 0"
        );
        let (result, _overflow_flag) = first_value.overflowing_div(second_value);
        state.set_register(self.third_register, result);
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
///
pub struct JumpInstruction {
    offset: i16,
}

impl JumpInstruction {
    pub fn new(code: &[u8]) -> Self {
        JumpInstruction {
            offset: LittleEndian::read_i16(&code[1..=2]),
        }
    }
}

impl Instruction for JumpInstruction {
    fn execute(&mut self, controller: &mut Controller) {
        let ip_value = controller.state().register(Register::IP);
        let address = (ip_value as i32 + self.offset as i32) as u32;
        controller.mut_state().set_register(Register::IP, address);
    }

    fn move_ip(&self) -> bool {
        false
    }
}

/// # LoadInstruction
/// Loads value from address [ip + offset],
/// [offset] is parsed as i16 little endian
///
/// Structure:
/// - 1st byte: instruction code
/// - 2nd byte: [register] address
/// - 3rd byte: offset
/// - 4th byte: offset
///
/// Result is stored in [register]
///
pub struct LoadInstruction {
    register: Register,
    offset: i16,
}

impl LoadInstruction {
    pub fn new(code: &[u8]) -> Self {
        LoadInstruction {
            register: Register::from_addr(code[1] as u32),
            offset: LittleEndian::read_i16(&code[2..=3]),
        }
    }
}

impl Instruction for LoadInstruction {
    fn execute(&mut self, controller: &mut Controller) {
        let ip_value = controller.state().register(Register::IP);
        let address = ip_value + self.offset as u32;

        let value = controller
            .mut_state()
            .get_memory_handler()
            .read_byte(address) as u32;

        controller.mut_state().set_register(self.register, value);
    }
}

/// # FinishInstruction
/// Final instruction that stops the execution of the virtual machine.
/// Sets value of the [END] register to 1, thus stops the pipeline.
///
/// Structure
/// - 1st byte: instruction code
/// - 2nd byte: not used
/// - 3rd byte: not used
/// - 4th byte: not used
///
pub struct FinishInstruction;

impl FinishInstruction {
    pub fn new(_code: &[u8]) -> Self {
        FinishInstruction {}
    }
}

impl Instruction for FinishInstruction {
    fn execute(&mut self, controller: &mut Controller) {
        controller.mut_state().set_register(Register::END, 1);
    }

    fn move_ip(&self) -> bool {
        false
    }
}

/// # OutInstruction
/// Prints string from the address, which is stored
/// in a given register.
///
/// Structure
/// - 1st byte: instruction code
/// - 2nd byte: [register] address
/// - 3rd byte: not used
/// - 4th byte: not used
///
pub struct OutInstruction {
    register: Register,
}

impl OutInstruction {
    pub fn new(code: &[u8]) -> Self {
        OutInstruction {
            register: Register::from_addr(code[1] as u32),
        }
    }
}

impl Instruction for OutInstruction {
    fn execute(&mut self, controller: &mut Controller) {
        let mut address = controller.state().register(self.register);
        loop {
            let char = controller.state().get_memory_handler().read_byte(address) as char;

            if char == '\0' {
                break;
            }

            controller.display().print(char);
            address += 1;
        }
    }
}

/// # EqualInstruction
/// Compares values in [left] and [right] registers
/// and stores 1 to CMP if they are equal. Otherwise, stores 0.
///
/// Structure:
/// - 1st byte: instruction code
/// - 2nd byte: [left] register address
/// - 3rd byte: [right] register address
/// - 4th byte: not used
///
pub struct EqualInstruction {
    left: Register,
    right: Register,
}

impl EqualInstruction {
    pub fn new(code: &[u8]) -> Self {
        EqualInstruction {
            left: Register::from_addr(code[1] as u32),
            right: Register::from_addr(code[2] as u32),
        }
    }
}

impl Instruction for EqualInstruction {
    fn execute(&mut self, controller: &mut Controller) {
        let left_value = controller.state().register(self.left);
        let right_value = controller.state().register(self.right);
        controller
            .mut_state()
            .set_register(Register::CMP, (left_value == right_value) as u32);
    }
}

/// # LessInstruction
/// Compares values in [left] and [right] registers
/// and stores 1 to CMP if left < right.
/// Otherwise, stores 0.
///
/// Structure:
/// - 1st byte: instruction code
/// - 2nd byte: [left] register address
/// - 3rd byte: [right] register address
/// - 4th byte: not used
///
pub struct LessInstruction {
    left: Register,
    right: Register,
}

impl LessInstruction {
    pub fn new(code: &[u8]) -> Self {
        LessInstruction {
            left: Register::from_addr(code[1] as u32),
            right: Register::from_addr(code[2] as u32),
        }
    }
}

impl Instruction for LessInstruction {
    fn execute(&mut self, controller: &mut Controller) {
        let left_value = controller.state().register(self.left);
        let right_value = controller.state().register(self.right);
        controller
            .mut_state()
            .set_register(Register::CMP, (left_value < right_value) as u32);
    }
}

/// # LessEqualInstruction
/// Compares values in [left] and [right] registers
/// and stores 1 to CMP if left <= right.
/// Otherwise, stores 0.
///
/// Structure:
/// - 1st byte: instruction code
/// - 2nd byte: [left] register address
/// - 3rd byte: [right] register address
/// - 4th byte: not used
///
pub struct LessEqualInstruction {
    left: Register,
    right: Register,
}

impl LessEqualInstruction {
    pub fn new(code: &[u8]) -> Self {
        LessEqualInstruction {
            left: Register::from_addr(code[1] as u32),
            right: Register::from_addr(code[2] as u32),
        }
    }
}

impl Instruction for LessEqualInstruction {
    fn execute(&mut self, controller: &mut Controller) {
        let left_value = controller.state().register(self.left);
        let right_value = controller.state().register(self.right);
        controller
            .mut_state()
            .set_register(Register::CMP, (left_value <= right_value) as u32);
    }
}

/// # LoadAbsoluteInstruction
/// Loads [value] to [register].
/// [value] is parsed as little-endian u32.
///
/// Structure:
/// - 1st byte: instruction code
/// - 2nd byte: [register] address
/// - 3rd byte: value
/// - 4th byte: value
///
pub struct LoadAbsoluteInstruction {
    register: Register,
    value: u32,
}

impl LoadAbsoluteInstruction {
    pub fn new(code: &[u8]) -> Self {
        LoadAbsoluteInstruction {
            register: Register::from_addr(code[1] as u32),
            value: LittleEndian::read_u16(&code[2..=3]) as u32,
        }
    }
}

impl Instruction for LoadAbsoluteInstruction {
    fn execute(&mut self, controller: &mut Controller) {
        controller
            .mut_state()
            .set_register(self.register, self.value);
    }
}

/// # InputInstruction
/// Gets a character from the user
/// and stores it in [register].
///
/// Structure:
/// - 1st byte: instruction code
/// - 2nd byte: [register] address
/// - 3rd byte: not used
/// - 4th byte: not used
///
pub struct InputInstruction {
    register: Register,
}

impl InputInstruction {
    pub fn new(code: &[u8]) -> Self {
        InputInstruction {
            register: Register::from_addr(code[1] as u32),
        }
    }
}

impl Instruction for InputInstruction {
    fn execute(&mut self, controller: &mut Controller) {
        let c = controller.mut_display().get();
        controller.mut_state().set_register(self.register, c as u32);
    }
}

/// # JumpCompareInstruction
/// Moves [IP] register by the [offset] if [CMP] flag
/// is not zero [offset] is parsed as little endian i16
///
/// Structure:
/// - 1st byte: instruction code
/// - 2nd byte: offset
/// - 3rd byte: offset
/// - 4th byte: not used
///
pub struct JumpCompareInstruction {
    offset: i16,
    success: bool,
}

impl JumpCompareInstruction {
    pub fn new(code: &[u8]) -> Self {
        JumpCompareInstruction {
            offset: LittleEndian::read_i16(&code[1..=2]),
            success: true,
        }
    }
}

impl Instruction for JumpCompareInstruction {
    fn execute(&mut self, controller: &mut Controller) {
        if controller.state().register(Register::CMP) == 0 {
            self.success = false;
            return;
        }
        let ip_value = controller.state().register(Register::IP);
        let address = (ip_value as i32 + self.offset as i32) as u32;
        controller.mut_state().set_register(Register::IP, address);
    }

    fn move_ip(&self) -> bool {
        !self.success
    }
}

/// # JumpNotCompareInstruction
/// Moves [IP] register by the [offset] if [CMP] flag
/// is zero [offset] is parsed as little endian i16
///
/// Structure:
/// - 1st byte: instruction code
/// - 2nd byte: offset
/// - 3rd byte: offset
/// - 4th byte: not used
///
pub struct JumpNotCompareInstruction {
    offset: i16,
    success: bool,
}

impl JumpNotCompareInstruction {
    pub fn new(code: &[u8]) -> Self {
        JumpNotCompareInstruction {
            offset: LittleEndian::read_i16(&code[1..=2]),
            success: true,
        }
    }
}

impl Instruction for JumpNotCompareInstruction {
    fn execute(&mut self, controller: &mut Controller) {
        if controller.state().register(Register::CMP) != 0 {
            self.success = false;
            return;
        }
        let ip_value = controller.state().register(Register::IP);
        let address = (ip_value as i32 + self.offset as i32) as u32;
        controller.mut_state().set_register(Register::IP, address);
    }

    fn move_ip(&self) -> bool {
        !self.success
    }
}

/// # OutFromRegisterInstruction
/// Prints char that is stored in the [register]
///
/// Structure:
/// - 1st byte: instruction code
/// - 2nd byte: [register] address
/// - 3rd byte: not used
/// - 4th byte: not used
///
pub struct OutFromRegisterInstruction {
    register: Register
}

impl OutFromRegisterInstruction {
    pub fn new(code: &[u8]) -> Self {
        OutFromRegisterInstruction {
            register: Register::from_addr(code[1] as u32)
        }
    }
}

impl Instruction for OutFromRegisterInstruction {
    fn execute(&mut self, controller: &mut Controller) {
        let value = char::from_u32(controller.state().register(self.register))
            .expect("Expected char to be in register");
        controller.display().print(value);
    }
}

/// # SkipInstruction
/// Instruction that does nothing
/// Useful for labels implementation
///
/// Structure:
/// - 1st byte: instruction code
/// - 2nd byte: not used
/// - 3rd byte: not used
/// - 4th byte: not used
pub struct SkipInstruction {}

impl SkipInstruction {
    pub fn new(_code: &[u8]) -> Self {
        SkipInstruction{}
    }
}

impl Instruction for SkipInstruction {
    fn execute(&mut self, _controller: &mut Controller) {
        return
    }
}

/// # OutNumberInstruction
/// Outputs a number stored in the [register]
///
/// Structure:
/// - 1st byte: instruction code
/// - 2nd byte: [register] address
/// - 3rd byte: not used
/// - 4th byte: not used
pub struct OutNumberInstruction {
    register: Register
}

impl OutNumberInstruction {
    pub fn new(code: &[u8]) -> Self {
        OutNumberInstruction{
            register: Register::from_addr(code[1] as u32),
        }
    }
}

impl Instruction for OutNumberInstruction {
    fn execute(&mut self, controller: &mut Controller) {
        let number = controller.state().register(self.register);
        let number_str = number.to_string();
        for c in number_str.chars() {
            controller.display().print(c);
        }
    }
}

/// # MoveInstruction
/// Copies value from [first_register] to [second_register]
///
/// Structure:
/// - 1st byte: instruction code
/// - 2nd byte: [first_register] address
/// - 3rd byte: [second_register] address
/// - 4th byte: not used
///
pub struct MoveInstruction {
    first_register: Register,
    second_register: Register,
}

impl MoveInstruction {
    pub fn new(code: &[u8]) -> Self {
        MoveInstruction {
            first_register: Register::from_addr(code[1] as u32),
            second_register: Register::from_addr(code[2] as u32),
        }
    }
}

impl Instruction for MoveInstruction {
    fn execute(&mut self, controller: &mut Controller) {
        let value = controller.state().register(self.first_register);
        controller.mut_state().set_register(self.second_register, value);
    }
}