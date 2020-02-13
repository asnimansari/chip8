use crate::cpu::CPU;
use crate::cpu::PC_START;
use crate::memory::Memory;

pub struct Chip8 {
    memory: Memory,
    cpu: CPU,
}


impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            memory: Memory::new(),
            cpu: CPU::new(),
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        for i in 0..data.len() {
            self.memory.write_byte(PC_START + (i as u16), data[i])
        }
    }
    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction(&mut self.memory)
    }
}