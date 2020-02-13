use std::fmt;
use std::fmt::{Error, Formatter};

use crate::memory::Memory;

pub const PC_START: u16 = 0x200;

impl CPU {
    pub fn new() -> CPU {
        CPU {
            vx: [0; 16],
            pc: PC_START,
            i: 0,
            prev_pc: 0,
        }
    }

    pub fn run_instruction(&mut self, mem: &mut Memory) {
        let hi = mem.read_byte(self.pc) as u16;
        let low = mem.read_byte(self.pc + 1) as u16;


        let instruction = (hi << 8) | low;

        println!("Instruction Read  {:#X}: high:{:#X}  low: {:#X}", instruction, hi, low);


        let nnn = instruction & 0x0FFF;
        let nn = (instruction & 0x00FF) as u8;
        let n = instruction & 0x000F;

        let x = (instruction & 0x0f00) >> 8;
        let y = (instruction & 0x00f0) >> 4;


        println!("instruction={:#X} nnn={:#X} nn={:#X} n={:#X} x={:#X} y={:#X} ", instruction, nnn, nn, n, x, y);

        if self.prev_pc == self.pc {
            panic!("Please increment");
        }
        self.prev_pc = self.pc;
        match (instruction & 0xF000) >> 12 {
            0x1 => {
                self.pc = nnn
            }

            0x6 => {
                self.write_to_vx_reg(x, nn);
                self.pc += 2;
            }
            0xA => {
                self.i = nnn;
                self.pc += 2;
            }
            0xD => {}

            _ => panic!("Un recognized instruction {:#X} :{:#X}", self.pc, instruction)
        }
    }

    fn write_to_vx_reg(&mut self, index: u16, value: u8) {
        self.vx[index as usize] = value;
    }
    fn read_to_vx_reg(&mut self, index: u16) -> u8 {
        self.vx[index as usize]
    }
}


pub struct CPU {
    vx: [u8; 16],
    pc: u16,
    i: u16,
    prev_pc: u16,
}


impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\npc: {:#X}\n", self.pc)?;
        write!(f, "vx: ")?;
        for item in &self.vx {
            write!(f, "{:#X} ", *item)?;
        }
        write!(f, "\n")?;
        write!(f, "i: {:#X}\n", self.i)
    }
}