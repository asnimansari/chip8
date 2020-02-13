use std::fs::File;
use std::io::Read;

use crate::cpu::CPU;

mod memory;
mod cpu;

fn main() {
    let mut file = File::open("data/INVADERS").unwrap();

    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);

    println!("Data {:?}", data);

    let mut cpu = CPU::new();
    cpu.load_rom(&data)
}
