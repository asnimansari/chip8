pub struct Memory {
    mem: [u8; 4096]
}

impl Memory {
    pub fn write_byte(&mut self, address: usize, value: u8) {
        self.mem[address] = value
    }

    pub fn read_byte(&mut self, address: u16) {}

    pub fn new() -> Memory {
        Memory {
            mem: [0; 4096]
        }
    }
}