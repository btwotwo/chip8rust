use std::ops::{Index, IndexMut};

pub struct Registers {
    regs: [u8; 16],
}

pub enum Position {
    ///Second 4 bits (0X00)
    X,
    ///Third 4 bits (00Y0)
    Y,
}

impl Registers {
    pub fn new() -> Registers {
        Registers { regs: [0; 16] }
    }

    pub fn add_immediate(&mut self, index: u8, value: u8) {
        let (result, _) = self[index].overflowing_add(value);
        self[index] = result;
    }

    pub fn get_index(opcode: u16, position: Position) -> u8 {
        match position {
            Position::X => ((opcode & 0x0F00) >> 8) as u8,
            Position::Y => ((opcode & 0x00F0) >> 4) as u8,
        }
    }

    pub fn set_carry(&mut self, carried: bool) {
        if carried {
            self[0xF] = 1;
        } else {
            self[0xF] = 0;
        }
    }
}

impl Index<u8> for Registers {
    type Output = u8;

    fn index(&self, index: u8) -> &Self::Output {
        &self.regs[index as usize]
    }
}

impl Index<(u16, Position)> for Registers {
    type Output = u8;

    fn index(&self, index: (u16, Position)) -> &Self::Output {
        let index = Registers::get_index(index.0, index.1);
        &self.regs[index as usize]
    }
}

impl IndexMut<(u16, Position)> for Registers {
    fn index_mut(&mut self, index: (u16, Position)) -> &mut Self::Output {
        let index = Registers::get_index(index.0, index.1);
        &mut self.regs[index as usize]
    }
}

impl IndexMut<u8> for Registers {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.regs[index as usize]
    }
}
