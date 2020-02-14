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
            ret_stack: Vec::<u16>::new(),
        }
    }

    pub fn run_instruction(&mut self, mem: &mut Memory) {
        let hi = mem.read_byte(self.pc) as u16;
        let low = mem.read_byte(self.pc + 1) as u16;


        let instruction = (hi << 8) | low;

        println!("Instruction Read  {:#X}: high:{:#X}  low: {:#X}", instruction, hi, low);


        let nnn = instruction & 0x0FFF;
        let nn = (instruction & 0x00FF) as u8;
        let n = (instruction & 0x000F) as u8;

        let x = ((instruction & 0x0f00) >> 8) as u8;
        let y = ((instruction & 0x00f0) >> 4) as u8;


        println!("instruction={:#X} nnn={:#X} nn={:#X} n={:#X} x={:#X} y={:#X} ", instruction, nnn, nn, n, x, y);

        if self.prev_pc == self.pc {
            panic!("Please increment");
        }
        self.prev_pc = self.pc;
        match (instruction & 0xF000) >> 12 {
            0x1 => {
                self.pc = nnn
            }
            0x2 => {
//                Suboutine call
                self.ret_stack.push(self.pc + 2);
                self.pc = nnn;
            }
            0x3 => {
                let vx = self.read_to_vx_reg(x);
                self.pc += 2;
                if vx == nn {
                    self.pc += 2;
                }
            }

            0x6 => {
                self.write_to_vx_reg(x, nn);
                self.pc += 2;
            }
            0x7 => {
                let current_vx_value = self.read_to_vx_reg(x);

                self.write_to_vx_reg(x, current_vx_value + nn);
                self.pc += 2;
            }
            0x8 => {
                match n {
                    0x0 => {
                        let vy = self.read_to_vx_reg(y);
                        self.write_to_vx_reg(x, vy);
                        self.pc += 2
                    }
                    _ => panic!("Un recognized instruction {:#X} :{:#X}", self.pc, instruction)
                }
            }
            0xA => {
                self.i = nnn;
                self.pc += 2;
            }
            0xD => {
                self.deug_draw_sprite(mem, x, y, n);
                self.pc += 2;
            }
            0xE => {
                match nn {
                    0xA1 => {}
                    _ => panic!("Un recognized instruction {:#X} :{:#X}", self.pc, instruction)
                }
            }
            0xF => {
                match nn {
                    0x1e => {
                        self.i += self.read_to_vx_reg(x) as u16;
                        self.pc += 2;
                    }
                    _ => unreachable!()
                }
            }

            _ => panic!("Un recognized instruction {:#X} :{:#X}", self.pc, instruction)
        }
    }

    fn deug_draw_sprite(&self, mem: &mut Memory, x: u8, y: u8, height: u8) {
        println!("Drawing sprite at ({}, {})", x, y);
        for y in 0..height {
            let mut b = mem.read_byte(self.i + y as u16);
            for _ in 0..8 {
                match (b & 0b1000_0000) >> 7 {
                    0 => print!("_"),
                    1 => print!("#"),
                    _ => unreachable!()
                }
                b = b << 1;
            }
            println!();
        }
        println!();
    }

    fn write_to_vx_reg(&mut self, index: u8, value: u8) {
        self.vx[index as usize] = value;
    }
    fn read_to_vx_reg(&mut self, index: u8) -> u8 {
        self.vx[index as usize]
    }
}


pub struct CPU {
    vx: [u8; 16],
    pc: u16,
    i: u16,
    prev_pc: u16,
    ret_stack: Vec<u16>,
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