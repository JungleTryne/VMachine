use crate::vm::memory::VirtualMemory;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

#[derive(Copy, Clone, Debug)]
pub enum Register {
    IP,
    R0,
    R1,
    R2,
    R3,
    END,
}

impl Register {
    pub fn get_addr(&self) -> u32 {
        match self {
            Register::IP => 0,
            Register::R0 => 4,
            Register::R1 => 8,
            Register::R2 => 12,
            Register::R3 => 16,
            Register::END => 24,
        }
    }

    pub fn from_addr(addr: u32) -> Self {
        match addr {
            0 => Register::IP,
            4 => Register::R0,
            8 => Register::R1,
            12 => Register::R2,
            16 => Register::R3,
            24 => Register::END,
            _ => panic!("Invalid register address"),
        }
    }
}

pub struct State {
    memory: VirtualMemory,
}

impl State {
    pub fn new(memory: VirtualMemory) -> Self {
        State { memory }
    }

    pub fn get_register(&self, register: Register) -> u32 {
        self.get_register_impl(register.get_addr())
    }

    pub fn set_register(&mut self, register: Register, value: u32) {
        self.set_register_impl(register.get_addr(), value);
    }

    fn get_register_impl(&self, register_addr: u32) -> u32 {
        let mut buf = self.memory.read_code(register_addr);
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
            self.memory.write_addr(register_addr + index as u32, *byte);
        }
    }

    pub fn get_memory_handler(&mut self) -> &mut VirtualMemory {
        &mut self.memory
    }
}
