pub struct Memory {
    mem: [u8; 4096]
}

impl Memory {
    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.mem[i] = value
    }

    pub fn read_byte(&mut self, address: u16) {}

    pub fn new() -> Memory {
        Memory {
            mem: [0; 4096]
        }
    }
}