
pub struct Memory {
    mem: [u8; 4096]
}

impl Memory {
    pub fn write_byte(&mut self, address: usize, value: u8) {
        self.mem[address] = value
    }

    pub fn read_byte(&mut self, address: u16) {}

    pub fn new() -> Memory {
        let mut memory = Memory {
            mem: [0; 4096]
        };
        let sprites = [
            [
                0xF0,
                0x90,
                0x90,
                0x90,
                0xF0
            ],
            [
                0x20,
                0x60,
                0x20,
                0x20,
                0x70
            ],
            [
                0xF0,
                0x10,
                0xF0,
                0x80,
                0xF0
            ],
            [
                0xF0,
                0x10,
                0xF0,
                0x10,
                0xF0
            ],
            [
                0x90,
                0x90,
                0xF0,
                0x10,
                0x10,
            ],
            [
                0xF0,
                0x80,
                0xF0,
                0x10,
                0xF0
            ],
            [
                0xF0,
                0x80,
                0xF0,
                0x90,
                0xF0,
            ],
            [
                0xF0,
                0x10,
                0x20,
                0x40,
                0x40,
            ],
            [
                0xF0,
                0x90,
                0xF0,
                0x90,
                0xF0,
            ],
            [
                0xF0,
                0x90,
                0xF0,
                0x10,
                0xF0
            ],
            [
                0xF0,
                0x90,
                0xF0,
                0x90,
                0x90,
            ],
            [
                0xE0,
                0x90,
                0xE0,
                0x90,
                0xE0,
            ],
            [
                0xF0,
                0x80,
                0x80,
                0x80,
                0xF0,
            ],
            [
                0xE0,
                0x90,
                0x90,
                0x90,
                0xE0,
            ],
            [
                0xF0,
                0x80,
                0xF0,
                0x80,
                0xF0,
            ],
            [
                0xF0,
                0x80,
                0xF0,
                0x80,
                0x80,
            ]
        ];

        let mut i = 0;
        for sprite in sprites.iter() {
            for ch in sprite.iter() {
                memory.mem[i] = *ch;
                i += 1;
            }
        }

//
//        for i in 0..0xff{
//            println!("RAM : {} {:#X}",i, memory.mem[i]);
//        }


        memory
    }
}