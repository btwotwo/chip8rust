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

#[derive(Debug)]
pub struct Display {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_not_set_collision() {
        let mut disp = Display::new();

        disp.contents[0] = 0b_00001111;
        let sprite = vec![
            0b_11110000
        ];

        let col = disp.draw(0, 0, &sprite);

        assert_eq!(col, false);
    }

    #[test]
    fn sets_collision() {
        let mut disp = Display::new();

        disp.contents[0] = 0b_00001111;
        let sprite = vec![
            0b_0000_1001
        ];

        let col = disp.draw(0, 0, &sprite);

        assert_eq!(col, true);
    }

    #[test]
    fn y_moves_to_the_other_side() {
        let mut disp = Display::new();

        disp.contents[31] = 0b_0000_0000;

        let sprite = vec![
            0b0000_1111,
            0b0000_1111
        ];

        disp.draw(0, 31, &sprite);

        println!("{:?}", disp);
        assert_eq!(disp.contents[0], 0b0000_1111);

    }
}