use std::fs::OpenOptions;
use std::path::Path;

use crate::vm::arch::ARCH_BYTES;
use memmap::MmapMut;

/// # Virtual Memory
/// Simulates memory of the machine.
/// Maps given image file to the host memory using mmap.
pub struct VirtualMemory {
    base_pointer: MmapMut,
}

impl VirtualMemory {
    pub fn new(image_path: &Path) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(image_path)
            .expect("Couldn't open image file");

        let mmap_pointer =
            unsafe { MmapMut::map_mut(&file).expect("Failed to initialize virtual memory") };

        VirtualMemory {
            base_pointer: mmap_pointer,
        }
    }

    pub fn read_byte(&self, addr: u32) -> u8 {
        let addr = addr as usize;
        self.base_pointer[addr]
    }

    pub fn write_byte(&mut self, addr: u32, val: u8) {
        let addr = addr as usize;
        self.base_pointer[addr] = val;
    }

    pub fn read_word(&self, addr: u32) -> &[u8] {
        assert_eq!(addr % ARCH_BYTES, 0, "The word address is not aligned");
        let addr = addr as usize;
        &self.base_pointer[addr..(addr + ARCH_BYTES as usize)]
    }

    pub fn write_word(&mut self, addr: u32, value: &[u8]) {
        assert_eq!(addr % ARCH_BYTES, 0, "The word address is not aligned");
        let addr = addr as usize;
        for i in 0..ARCH_BYTES as usize {
            self.base_pointer[addr + i] = value[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::VirtualMemory;
    use memmap::MmapMut;

    #[test]
    fn it_works() {
        let mut mmap_pointer = MmapMut::map_anon(256).unwrap();

        let data = [18u8, 52, 0, 0];
        for (i, elem) in data.into_iter().enumerate() {
            mmap_pointer[i] = elem;
        }

        let mut memory_handler = VirtualMemory {
            base_pointer: mmap_pointer,
        };

        assert_eq!(memory_handler.read_word(0), [18, 52, 0, 0]);

        memory_handler.write_byte(5, 12);
        assert_eq!(memory_handler.read_byte(5), 12);

        memory_handler.write_byte(5, 0);
    }
}
