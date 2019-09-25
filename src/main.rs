mod keyboard;
mod opcode;
mod program_counter;
mod registers;
mod screen;

use opcode::{Opcode, OpcodeHandler};
use program_counter::ProgramCounter;
use registers::Registers;

use keyboard::Keyboard;
use crossterm::{
    execute, input, style, AsyncReader, Clear, ClearType, Color, Crossterm, Goto, InputEvent,
    KeyEvent, PrintStyledFont, RawScreen, Result, Show,
};

pub type Memory = [u8; 4096];
pub type Stack = [u16; 16];

use screen::display::*;

fn main() {
    let mut display = Display::new();
    let mut crossterm = Crossterm::new();
    let _raw = RawScreen::into_raw_mode().unwrap();
    crossterm.cursor().hide().unwrap();

    let keyboard = keyboard::Keyboard::new();

    loop {
        display.contents = [0; 32];
        let inp = keyboard.wait_for_key();
        let symbol = &FONT[(inp * 5) as usize..(inp * 5 + 5) as usize];
        display.draw(0, 62, symbol);
        screen::screen::redraw(&display, &mut crossterm);
    }

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
    pub memory: Memory,

    /// Main registers of the chip (V0 - VE), VF is used as carry flag.
    pub v: Registers,

    /// Special "I" register, generaly used to store memory addresses.
    pub i: u16,

    /// Index of the current instruction.
    pub program_counter: ProgramCounter,

    pub screen: Display,

    pub delay_timer: u8,
    pub sound_timer: u8,

    pub stack: Stack,
    pub stack_pointer: u8,

    pub keyboard: Keyboard,
}

impl Chip {
    pub fn new() -> Chip {
        Chip {
            program_counter: ProgramCounter::new(0x200),
            memory: [0; 4096],

            v: Registers::new(),

            i: 0,
            screen: Display::new(),
            stack: [0; 16],

            delay_timer: 0,
            sound_timer: 0,

            stack_pointer: 0,
            keyboard: Keyboard::new(),
        }
    }

    pub fn load_program(&mut self, bytecode: &[u8]) {
        for (i, item) in bytecode.iter().enumerate() {
            self.memory[i + 512] = *item;
        }
    }

    pub fn start(mut self) {
        loop {
            self.tick()
        }
    }

    fn tick(&mut self) {
        //get and decode opcode
        let opcode = self.decode_opcode();

        //execute opcode
        OpcodeHandler::next(opcode, self);

        //update timers
        self.delay_timer += 1;
    }

    fn decode_opcode(&self) -> Opcode {
        let program_counter = self.program_counter.get() as usize;
        let first_byte = u16::from(self.memory[program_counter]);
        let second_byte = u16::from(self.memory[program_counter + 1]);
        first_byte << 8 | second_byte
    }
}
