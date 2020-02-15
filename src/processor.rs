use crate::bus::Bus;
use crate::cpu::CPU;
use crate::cpu::PC_START;
use crate::memory::Memory;

pub struct Chip8 {
    cpu: CPU,
    bus: Bus,
}


impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            cpu: CPU::new(),
            bus: Bus::new(),
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        for i in 0..data.len() {
            self.bus.memory_write_byte(PC_START + (i as u16), data[i])
        }
    }
    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction(&mut self.bus);

        println!("CPU State {:?}", self.cpu);
    }
}