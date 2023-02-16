use super::memory::VirtualMemory;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub enum Register {
    IP = 0,
    R0 = 4,
    R1 = 8,
    R2 = 12,
    R3 = 16,
    END = 24,
}

pub struct State {
    memory: VirtualMemory,
}

impl State {
    pub fn new(memory: VirtualMemory) -> Self {
        State { memory }
    }

    pub fn get_register(&self, register: Register) -> u32 {
        self.get_register_impl(register as u32)
    }

    pub fn set_register(&mut self, register: Register, value: u32) {
        self.set_register_impl(register as u32, value);
    }

    fn get_register_impl(&self, register_addr: u32) -> u32 {
        let mut buf = self.memory.read_code(register_addr);
        buf.read_u32::<LittleEndian>().unwrap_or_else(|_| {
            panic!("Couldn't read the register with address: {}", register_addr)
        })
    }

    fn set_register_impl(&mut self, register_addr: u32, value: u32) {
        let mut wrt = vec![]; // FIXME: It doesn't allow me to use slices
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
