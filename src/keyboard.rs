use minifb::{Key, KeyRepeat, Window, WindowOptions};

pub struct Keyboard<'a> {
    key_pressed: Option<u8>
}


impl Keyboard {
    pub fn new(window: &mut Window) -> Keyboard {
        Keyboard {
            key_pressed: None,
        }
    }

    //Todo implement proper key handling
    pub fn key_pressed(&self, key_code: u8) -> bool {
        true
    }

    pub fn set_keys(&mut self, key: Vec<Option<u8>>) {
        self.key_pressed = match key {
            Some(Key::Key1) => Some(0x1),
            Some(Key::Key2) => Some(0x2),
            Some(Key::Key3) => Some(0x3),
            Some(Key::Key4) => Some(0xC),

            Some(Key::Q) => Some(0x4),
            Some(Key::W) => Some(0x5),
            Some(Key::E) => Some(0x6),
            Some(Key::R) => Some(0xD),

            Some(Key::A) => Some(0x7),
            Some(Key::S) => Some(0x8),
            Some(Key::D) => Some(0x9),
            Some(Key::F) => Some(0xE),

            Some(Key::Z) => Some(0xA),
            Some(Key::X) => Some(0x0),
            Some(Key::C) => Some(0xB),
            Some(Key::V) => Some(0xF),
            _ => None
        }
    }
    pub fn get_key_blocking(&self) -> Option<u8> {
        self.key_pressed
    }
}

//https://youtu.be/zm44LNb2n0s?t=6584