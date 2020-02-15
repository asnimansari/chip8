use std::fs::File;
use std::io::Read;

use crate::processor::Chip8;

mod memory;
mod processor;
mod cpu;
mod bus;
mod display;
mod keyboard;

fn main() {
    let mut file = File::open("data/INVADERS").unwrap();

    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);

    println!("Data {:?}", data);

    let mut cpu = Chip8::new();
    cpu.load_rom(&data);

    loop {
        cpu.run_instruction();
    }
}
