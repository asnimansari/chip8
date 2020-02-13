use crate::memory::Memory;

pub const PC_START: u16 = 0x200;

pub struct CPU {
    vx: [u8; 16],
    pc: u16,
    i: u16,

}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            vx: [0; 16],
            pc: PC_START,
            i: 0,
        }
    }

    pub fn run_instruction(&mut self, mem: &mut Memory) {}
}