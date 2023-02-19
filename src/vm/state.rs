use crate::vm::memory::VirtualMemory;
use crate::vm::ARCH_BYTES;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

#[derive(Copy, Clone, Debug)]
pub enum Register {
    IP,
    R0,
    R1,
    R2,
    R3,
    CMP,
    END,
}

#[allow(clippy::erasing_op)]
#[allow(clippy::identity_op)]
impl Register {
    pub fn as_addr(&self) -> u32 {
        match self {
            Register::IP => 0 * ARCH_BYTES,
            Register::R0 => 1 * ARCH_BYTES,
            Register::R1 => 2 * ARCH_BYTES,
            Register::R2 => 3 * ARCH_BYTES,
            Register::R3 => 4 * ARCH_BYTES,
            Register::END => 5 * ARCH_BYTES,
            Register::CMP => 6 * ARCH_BYTES,
        }
    }

    pub fn from_addr(addr: u32) -> Self {
        match addr {
            addr if addr == 0 * ARCH_BYTES => Register::IP,
            addr if addr == 1 * ARCH_BYTES => Register::R0,
            addr if addr == 2 * ARCH_BYTES => Register::R1,
            addr if addr == 3 * ARCH_BYTES => Register::R2,
            addr if addr == 4 * ARCH_BYTES => Register::R3,
            addr if addr == 5 * ARCH_BYTES => Register::END,
            addr if addr == 6 * ARCH_BYTES => Register::CMP,
            _ => panic!("Invalid register address"),
        }
    }
}

/// # Machine State
pub struct State {
    memory: VirtualMemory,
}

impl State {
    pub fn new(memory: VirtualMemory) -> Self {
        State { memory }
    }

    pub fn register(&self, register: Register) -> u32 {
        self.get_register_impl(register.as_addr())
    }

    pub fn set_register(&mut self, register: Register, value: u32) {
        self.set_register_impl(register.as_addr(), value);
    }

    fn get_register_impl(&self, register_addr: u32) -> u32 {
        let mut buf = self.memory.read_word(register_addr);
        buf.read_u32::<LittleEndian>().unwrap_or_else(|_| {
            panic!("Couldn't read the register with address: {}", register_addr)
        })
    }

    fn set_register_impl(&mut self, register_addr: u32, value: u32) {
        let mut wrt = vec![];
        wrt.write_u32::<LittleEndian>(value).unwrap_or_else(|_| {
            panic!(
                "Couldn't write the register with address: {}",
                register_addr
            )
        });
        for (index, byte) in wrt.iter().enumerate() {
            self.memory.write_byte(register_addr + index as u32, *byte);
        }
    }

    pub fn get_memory_handler(&self) -> &VirtualMemory {
        &self.memory
    }
}
