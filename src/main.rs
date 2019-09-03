mod display;
mod opcode;
mod program_counter;
mod registers;

use opcode::Opcode;
use program_counter::ProgramCounter;
use registers::Registers;

pub type Memory = [u8; 4096];
pub type Stack = [u16; 16];

use cursive::Cursive;
use cursive::views::{Canvas};
use cursive::XY;
use cursive::theme::{BaseColor, PaletteColor, Color};

fn main() {
    let mut siv = Cursive::default();

    let mut current_theme = siv.current_theme().clone();
    current_theme.palette[PaletteColor::Background] = Color::Dark(BaseColor::Black);

    siv.set_theme(current_theme);

    let canvas = Canvas::new(())
    .with_required_size(|_, _| XY::new(64, 64))
    .with_draw(|_, c| {
        c.print_box((0, 10), (20, 30), false);
    });

    siv.run();

    // let bytecode = vec![
    //     // simple program: print line, then beep
    //     0x00, 0xE0, // clear screen
    //     0x60, 0x20, // sets V0 to 32
    //     0x61, 0x10, // sets V1 to 16
    //     0xD0, 0x1F, //draw a line
    // ];

    // let mut chip = Chip::new();

    // chip.load_program(&bytecode);
}

pub struct Chip {
    current_opcode: Opcode,
    pub memory: Memory,

    /// Main registers of the chip (V0 - VE), VF is used as carry flag.
    pub v: Registers,

    /// Special "I" register, generaly used to store memory addresses.
    pub i: u16,

    /// Index of the current instruction.
    pub program_counter: ProgramCounter,

    screen: [bool; 64 * 32],

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
        let first_byte = u16::from(self.memory[memory_pointer]);
        let second_byte = u16::from(self.memory[memory_pointer + 1]);
        self.current_opcode = first_byte << 8 | second_byte
    }
}
