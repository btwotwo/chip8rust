use super::{Chip};
use std::collections::HashMap;

pub type Opcode = u16;

type OpcodeImpl = fn(Opcode, &mut Chip) -> ();


struct OpcodeHandler {
    opcode_map: HashMap<Opcode, OpcodeImpl>
}

impl OpcodeHandler {
    pub fn new() -> OpcodeHandler {
        let mut map = HashMap::new();
        
        map.insert(
            0x00EE,
            ret as OpcodeImpl
        );

        map.insert(
            0x1000,
            jp as OpcodeImpl
        );

        map.insert(
            0x2000,
            call as OpcodeImpl
        );


        OpcodeHandler {
            opcode_map: map
        }
    }
}


fn ret(_: Opcode, chip: &mut Chip) {
    chip.program_counter = chip.stack[chip.stack_pointer as usize];
    chip.stack_pointer = chip.stack_pointer - 1
}

fn jp(opcode: Opcode, chip: &mut Chip) {
    chip.program_counter = opcode & 0x0FFF;
}

fn call(opcode: Opcode, chip: &mut Chip) {
    chip.stack_pointer = chip.stack_pointer + 1;
    chip.stack[chip.stack_pointer as usize] = chip.program_counter;
    chip.program_counter = opcode & 0x0FFF;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ret_test() {
        let mut chip = Chip::new();
        chip.stack_pointer = 10;
        chip.stack[chip.stack_pointer as usize] = 0x1234;

        ret(0x00EE, &mut chip);

        assert_eq!(chip.program_counter, 0x1234);
        assert_eq!(chip.stack_pointer, 9);
    }

    #[test]
    fn jp_test() {
        let mut chip = Chip::new();
        chip.program_counter = 0xABC;
        
        jp(0x1DEA, &mut chip);
        
        assert_eq!(chip.program_counter, 0xDEA)
    }

    #[test]
    fn call_test() {
        let mut chip = Chip::new();
        chip.program_counter = 12;

        let opcode = 0x2DEA;

        call(0x2DEA, &mut chip);
        
        assert_eq!(chip.program_counter, 0xDEA);
        assert_eq!(chip.stack_pointer, 1);
        assert_eq!(chip.stack[chip.stack_pointer as usize], 12);
    }
}