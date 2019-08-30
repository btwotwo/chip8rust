#[derive(Debug)]
pub struct ProgramCounter(u16);

impl ProgramCounter {
    pub fn new(initial: u16) -> ProgramCounter {
        ProgramCounter(initial)
    }

    pub fn set(&mut self, num: u16) {
        self.0 = num;
    }

    pub fn get(&self) -> u16 {
        self.0
    }

    pub fn increment(&mut self) {
        self.0 += 2;
    }
}

impl PartialEq<u16> for ProgramCounter {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}
