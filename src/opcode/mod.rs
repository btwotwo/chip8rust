use super::registers::{Position, Registers};
use super::Chip;
use std::collections::HashMap;

macro_rules! opcode_func_map {
    ($($opcode:expr => $func: expr), *) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($opcode, $func as OpcodeImpl);
            )*
            map
        }
    };
}

pub type Opcode = u16;
type OpcodeImpl = fn(Opcode, &mut Chip) -> ();

struct OpcodeHandler {
    opcode_map: HashMap<Opcode, OpcodeImpl>,
}

impl OpcodeHandler {
    pub fn new() -> OpcodeHandler {
        let map = opcode_func_map!(
            0x00EE => ret,
            0x1000 => jp,
            0x2000 => call,
            0x3000 => se,
            0x4000 => sne,
            0x5000 => sre,
            0x6000 => ld,
            0x7000 => add
        );

        OpcodeHandler { opcode_map: map }
    }
}

fn ret(_: Opcode, chip: &mut Chip) {
    chip.program_counter
        .set(chip.stack[chip.stack_pointer as usize]);
    chip.stack_pointer -= 1;
}

fn jp(opcode: Opcode, chip: &mut Chip) {
    chip.program_counter.set(opcode & 0x0FFF);
}

fn call(opcode: Opcode, chip: &mut Chip) {
    chip.stack_pointer += 1;
    chip.stack[chip.stack_pointer as usize] = chip.program_counter.get();
    chip.program_counter.set(opcode & 0x0FFF);
}

//skip if equal
fn se(opcode: Opcode, chip: &mut Chip) {
    let register_index = (opcode & 0x0f00) >> 8;
    let compare = (opcode & 0x00FF) as u8;
    let register = chip.v.get_by_position(opcode, Position::X);

    if register == compare {
        chip.program_counter.skip(1);
    } else {
        chip.program_counter.increment();
    }
}

//skip if not equal
fn sne(opcode: Opcode, chip: &mut Chip) {
    let register = chip.v.get_by_position(opcode, Position::X);
    let to_compare = (opcode & 0x00FF) as u8;

    if register != to_compare {
        chip.program_counter.skip(1);
    } else {
        chip.program_counter.increment();
    }
}

///Skip if registers equal
fn sre(opcode: Opcode, chip: &mut Chip) {
    let first_register = chip.v.get_by_position(opcode, Position::X);
    let second_register = chip.v.get_by_position(opcode, Position::Y);

    if first_register == second_register {
        chip.program_counter.skip(1);
    } else {
        chip.program_counter.increment();
    }
}

///Set Vx equal NN bytes
fn ld(opcode: Opcode, chip: &mut Chip) {
    let index = Registers::get_index_by_position(opcode, Position::X);
    chip.v.set(index, (opcode & 0x00FF) as u8);
    chip.program_counter.increment();
}

fn add(opcode: Opcode, chip: &mut Chip) {
    let index = Registers::get_index_by_position(opcode, Position::X);
    chip.v.add_immediate(index, (opcode & 0x00FF) as u8);
    chip.program_counter.increment();
}

#[cfg(test)]
mod test;
