use crate::memory::Memory;

pub struct CPU {
    memory: Memory
}


impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: Memory::new()
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        let offset = 0x200;
        for i in 0..data.len() {
            self.memory.write_byte(offset + i, data[i])
        }
    }
}