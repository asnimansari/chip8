pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

pub struct Display {
    screen: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
//    width: u8,
//    height: u8,
}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT],
//            width: 64,
//            height: 32,
        }
    }


    pub fn debug_draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool {
        let mut flipped = false;
        let mut b = byte.clone();

        let cord_x = x as usize;
        let cord_y = y as usize;


        for _ in 0..8 {
            match (b & 0b1000_0000) >> 7 {
                0 => {
                    if self.screen[cord_x][cord_y] == 1 {
                        flipped = true;
                    }

                    self.screen[cord_x][cord_y] = 0
                }
                1 => self.screen[cord_x][cord_y] = 1,
                _ => unreachable!()
            }
            b = b << 1;
        }
        println!();
        flipped
    }
}

