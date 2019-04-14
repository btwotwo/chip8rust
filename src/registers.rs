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

    pub fn get(&self, index: u8) -> u8 {
        self.regs[index as usize]
    }

    pub fn set(&mut self, index: u8, value: u8) {
        self.regs[index as usize] = value;
    }

    pub fn add_immediate(&mut self, index: u8, value: u8) {
        let (result, _) = self.regs[index as usize].overflowing_add(value);
        self.regs[index as usize] = result;
    }

    /// Returns register by its position
    pub fn get_by_position(&self, opcode: u16, position: Position) -> u8 {
        self.get(Registers::get_index_by_position(opcode, position))
    }

    pub fn get_index_by_position(opcode: u16, position: Position) -> u8 {
        match position {
            Position::X => ((opcode & 0x0F00) >> 8) as u8,
            Position::Y => ((opcode & 0x00F0) >> 4) as u8,
        }
    }
}
