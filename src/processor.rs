use minifb::{Key, Window, WindowOptions};

use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::cpu::PROGRAM_START;

pub struct Processor {
    bus: Bus,
    cpu: Cpu,

}

impl Processor {
    pub fn new(window: &mut Window) -> Processor {
        Processor { bus: Bus::new(window), cpu: Cpu::new() }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        for i in 0..data.len() {
            self.bus.ram_write_byte(PROGRAM_START + (i as u16), data[i]);
        }
    }

    pub fn run_instruction(&mut self) {
        self.bus.tick();
        self.cpu.run_instruction(&mut self.bus);
        println!("Cpu state: {:?}", self.cpu);
        println!("Bus state: {:?}", self.bus);
    }

    pub fn get_display_buffer(&self) -> &[u8] {
        self.bus.get_display_buffer()
    }
}