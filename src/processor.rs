use crate::bus::Bus;
use crate::cpu;
use crate::cpu::CPU;

pub struct C8Processor {
    bus: Bus,
    cpu: CPU,
}

impl C8Processor {
    pub fn new() -> C8Processor {
        C8Processor {
            bus: Bus::new(),
            cpu: CPU::new(),
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        for i in 0..data.len() {
            self.bus
                .ram_write_byte(cpu::PROGRAM_START + (i as u16), data[i]);
        }
    }

    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction(&mut self.bus);
    }

    pub fn get_display_buffer(&self) -> &[u8] {
        self.bus.get_display_buffer()
    }

    pub fn set_key_pressed(&mut self, key: Option<u8>) {
        self.bus.set_key_pressed(key);
    }
}
