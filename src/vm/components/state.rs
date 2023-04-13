use crate::vm::arch::ARCH_BYTES;
use crate::vm::components::memory::VirtualMemory;
use crate::vm::utils::register_macro::make_registers;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

make_registers! {
    IP => 0 * ARCH_BYTES,
    R0 => 1 * ARCH_BYTES,
    R1 => 2 * ARCH_BYTES,
    R2 => 3 * ARCH_BYTES,
    R3 => 4 * ARCH_BYTES,
    CMP => 5 * ARCH_BYTES,
    END => 6 * ARCH_BYTES,
    SP => 7 * ARCH_BYTES
}

/// # Machine State
pub struct State {
    memory: VirtualMemory,
}

impl State {
    pub fn new(memory: VirtualMemory) -> Self {
        State { memory }
    }

    pub fn register_value(&self, register: Register) -> u32 {
        self.read_word(register.as_addr())
    }

    pub fn set_register_value(&mut self, register: Register, value: u32) {
        self.write_word(register.as_addr(), value);
    }

    pub fn pop_from_stack(&mut self, register: Register) {
        let sp_value = self.register_value(Register::SP) - ARCH_BYTES;
        self.set_register_value(Register::SP, sp_value);

        let mut stack_value = self.memory.read_word(sp_value);
        let stack_value = stack_value.read_u32::<LittleEndian>().unwrap();

        self.set_register_value(register, stack_value);
    }

    pub fn push_to_stack(&mut self, register: Register) {
        let sp_value = self.register_value(Register::SP);
        let register_value = self.register_value(register);
        self.write_word(sp_value, register_value);
        self.set_register_value(Register::SP, sp_value + ARCH_BYTES);
    }

    fn read_word(&self, addr: u32) -> u32 {
        let mut buf = self.memory.read_word(addr);
        buf.read_u32::<LittleEndian>()
            .unwrap_or_else(|_| panic!("Couldn't read from address: {}", addr))
    }

    fn write_word(&mut self, addr: u32, value: u32) {
        let mut wrt = vec![];
        wrt.write_u32::<LittleEndian>(value)
            .unwrap_or_else(|_| panic!("Couldn't write the register with address: {}", addr));
        for (index, byte) in wrt.iter().enumerate() {
            self.memory.write_byte(addr + index as u32, *byte);
        }
    }

    pub fn get_memory_handler(&self) -> &VirtualMemory {
        &self.memory
    }

    pub fn get_mut_memory_handler(&mut self) -> &mut VirtualMemory {
        &mut self.memory
    }
}
