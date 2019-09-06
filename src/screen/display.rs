type Row = u64;
type Font = [u8; 80];

// credits to the http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#dispcoords
const FONT: [u8; 80] = [
// ****
// *  *
// *  *
// *  *
// ****
0xF0,0x90,0x90,0x90,0xF0,
//   * 
//  ** 
//   * 
//   * 
//  ***
0x20,0x60,0x20,0x20,0x70,
// ****
//    *
// ****
// *   
// ****
0xF0,0x10,0xF0,0x80,0xF0,
// ****
//    *
// ****
//    *
// ****
0xF0,0x10,0xF0,0x10,0xF0,

// *  *
// *  *
// ****
//    *
//    *
0x90,0x90,0xF0,0x10,0x10,

// ****
// *   
// ****
//    *
// ****
0xF0,0x80,0xF0,0x10,0xF0,

// ****
// *   
// ****
// *  *
// ****
0xF0,0x80,0xF0,0x90,0xF0,

// ****
//    *
//   * 
//  *  
//  * 
0xF0,0x10,0x20,0x40,0x40,

// ****
// *  *
// ****
// *  *
// ****
0xF0,0x90,0xF0,0x90,0xF0,

// ****
// *  *
// ****
//    *
// ****
0xF0,0x90,0xF0,0x10,0xF0,

// ****
// *  *
// ****
// *  *
// *  *
0xF0,0x90,0xF0,0x90,0x90,

// *** 
// *  *
// *** 
// *  *
// *** 
0xE0,0x90,0xE0,0x90,0xE0,

// ****
// *   
// *   
// *   
// ****
0xF0,0x80,0x80,0x80,0xF0,

// *** 
// *  *
// *  *
// *  *
// ***
0xE0,0x90,0x90,0x90,0xE0,

// ****
// *   
// ****
// *   
// ****
0xF0,0x80,0xF0,0x80,0xF0,

// ****
// *   
// ****
// *   
// * 
0xF0,0x80,0xF0,0x80,0x80
];

struct Display {
    contents: [Row; 32]
}

impl Display {
    pub fn new() -> Self{
        Display {
            contents: [0; 32]
        }
    }

    pub fn get_font() -> Font {
        FONT
    }

    pub fn draw(&mut self, x: u8, y: u8, sprites: &[u8]) -> bool {
        let mut collision = false;

        for (row_idx, part) in sprites.iter().enumerate() {
            let row_idx = (row_idx + (y as usize)) % 32;
            let row = self.contents[row_idx];
            let part = u64::from(*part);
            let part = part.rotate_right(x.into());
            
            for pixel_idx in 0..64 {
                if collision {
                    break;
                }

                const MASK: u64 = 0x8000_0000_0000_0000;

                let pixel = (MASK >> pixel_idx) & row;
                let part_pixel = (MASK >> pixel_idx) & part;

                if pixel != 0 && part_pixel != 0 {
                    collision = true;
                }
            }

            self.contents[row_idx] = row ^ part
        }

        collision
    }
}