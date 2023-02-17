use std::fs::OpenOptions;
use std::path::Path;

use memmap::MmapMut;

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

    pub fn read_addr(&self, addr: u32) -> u8 {
        let addr = addr as usize;
        self.base_pointer[addr]
    }

    pub fn write_addr(&mut self, addr: u32, val: u8) {
        let addr = addr as usize;
        self.base_pointer[addr] = val;
    }

    pub fn read_code(&self, addr: u32) -> &[u8] {
        assert_eq!(addr % 4, 0);
        let addr = addr as usize;
        &self.base_pointer[addr..addr + 4]
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

        assert_eq!(memory_handler.read_code(0), [18, 52, 0, 0]);

        memory_handler.write_addr(5, 12);
        assert_eq!(memory_handler.read_addr(5), 12);

        memory_handler.write_addr(5, 0);
    }
}
