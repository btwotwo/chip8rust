mod display;
mod opcode;
mod program_counter;
mod registers;

use opcode::Opcode;
use program_counter::ProgramCounter;
use registers::Registers;

pub type Memory = [u8; 4096];
pub type Stack = [u16; 16];

fn main() {
    let bytecode = vec![
        // simple program: print line, then beep
        0x00, 0xE0, // clear screen
        0x60, 0x20, // sets V0 to 32
        0x61, 0x10, // sets V1 to 16
        0xD0, 0x1F, //draw a line
    ];

    let mut chip = Chip::new();

    chip.load_program(&bytecode);
}

pub struct Chip {
    current_opcode: Opcode,
    pub memory: Memory,
    pub v: Registers, //registers (V0-VE), VF is "carry flag"
    pub i: u16,       // address register
    pub program_counter: ProgramCounter,

    screen: [bool; 64 * 32], //maybe enum?

    pub delay_timer: u8,
    pub sound_timer: u8,

    pub stack: Stack,
    pub stack_pointer: u8,
}

impl Chip {
    pub fn new() -> Chip {
        Chip {
            current_opcode: 0,
            program_counter: ProgramCounter::new(0x200),
            memory: [0; 4096],

            v: Registers::new(),
            i: 0,

            screen: [false; 64 * 32],
            stack: [0; 16],

            delay_timer: 0,
            sound_timer: 0,

            stack_pointer: 0,
        }
    }

    pub fn load_program(&mut self, bytecode: &[u8]) {
        for (i, item) in bytecode.iter().enumerate() {
            self.memory[i + 512] = *item;
        }
    }

    pub fn start(self) {
        loop {}
    }

    fn emulate_cycle(&mut self) {

        //get opcode
        //decode opcode
        //execute opcode
        //update timers

    }

    fn set_opcode(&mut self) {
        let memory_pointer = self.program_counter.get() as usize;
        let first_byte = self.memory[memory_pointer] as u16;
        let second_byte = self.memory[memory_pointer + 1] as u16;
        self.current_opcode = first_byte << 8 | second_byte
    }
}
