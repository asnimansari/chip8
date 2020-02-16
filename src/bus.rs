use crate::display::Display;
use crate::keyboard::Keyboard;
use crate::memory::Memory;

pub struct Bus {
    memory: Memory,
    keyboard: Keyboard,
    display: Display,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            memory: Memory::new(),
            keyboard: Keyboard::new(),
            display: Display::new(),

        }
    }

    pub fn memory_read_byte(&mut self, address: u16) -> u8 {
        self.memory.read_byte(address)
    }
    pub fn memory_write_byte(&mut self, address: u16, value: u8) {
        self.memory.write_byte(address, value)
    }

    pub fn debug_draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool {
        self.display.debug_draw_byte(byte, x, y)
    }

    pub fn key_pressed(&self, key_code: u8) -> bool {
        self.keyboard.key_pressed(key_code)
    }
}