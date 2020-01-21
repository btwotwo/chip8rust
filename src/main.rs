mod keyboard;
mod opcode;
mod program_counter;
mod registers;
mod screen;

use opcode::{Opcode, OpcodeHandler};
use program_counter::ProgramCounter;
use registers::Registers;

use keyboard::Keyboard;

pub type Memory = [u8; 4096];
pub type Stack = [u16; 16];

use screen::display::*;

fn main() -> crossterm::Result<()> {
    use std::fs;
    use std::io::Read;

    let filename = "race.ch8";

    let mut size = fs::metadata(filename)?.len();

    let mut buffer: Vec<u8> = Vec::with_capacity(size as usize);

    fs::File::open(filename)?.read_to_end(&mut buffer)?;

    let mut chip = Chip::new();

    chip.load_program(&buffer);
    chip.start();

    Ok(())
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
        let mut memory = [0; 4096];

        for (i, val) in screen::display::FONT.iter().enumerate() {
            memory[i] = *val;
        }

        Chip {
            program_counter: ProgramCounter::new(512),
            memory,

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
        screen::screen::init().unwrap();

        loop {
            self.screen.should_redraw = false;
            //get and decode opcode
            let opcode = self.decode_opcode();

            self.keyboard.register_key_press();

            //execute opcode
            OpcodeHandler::next(opcode, &mut self);

            //update timers
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            if self.sound_timer > 0 {
                // todo: beep
                self.sound_timer -= 1;
            }

            if self.screen.should_redraw {
                screen::screen::redraw(&self.screen).unwrap()
            }
        }
    }

    fn decode_opcode(&self) -> Opcode {
        let program_counter = self.program_counter.get() as usize;
        let first_byte = u16::from(self.memory[program_counter]);
        let second_byte = u16::from(self.memory[program_counter + 1]);
        first_byte << 8 | second_byte
    }
}
