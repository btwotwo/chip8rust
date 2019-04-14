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
            0x7000 => add,
            0x8000 => ldr,
            0x8001 => or,
            0x8002 => and,
            0x8003 => xor,
            0x8004 => addreg
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
    let register = chip.v[(opcode, Position::X)];

    if register == compare {
        chip.program_counter.skip(1);
    } else {
        chip.program_counter.increment();
    }
}

//skip if not equal
fn sne(opcode: Opcode, chip: &mut Chip) {
    let register = chip.v[(opcode, Position::X)];
    let to_compare = (opcode & 0x00FF) as u8;

    if register != to_compare {
        chip.program_counter.skip(1);
    } else {
        chip.program_counter.increment();
    }
}

///Skip if registers equal
fn sre(opcode: Opcode, chip: &mut Chip) {
    let first_register = chip.v[(opcode, Position::X)];
    let second_register = chip.v[(opcode, Position::Y)];

    if first_register == second_register {
        chip.program_counter.skip(1);
    } else {
        chip.program_counter.increment();
    }
}

///Set Vx equal NN bytes
fn ld(opcode: Opcode, chip: &mut Chip) {
    chip.v[(opcode, Position::X)] = (opcode & 0x00FF) as u8;
    chip.program_counter.increment();
}

fn add(opcode: Opcode, chip: &mut Chip) {
    let index = Registers::get_index(opcode, Position::X);
    chip.v.add_immediate(index, (opcode & 0x00FF) as u8);
    chip.program_counter.increment();
}

fn ldr(opcode: Opcode, chip: &mut Chip) {
    chip.v[(opcode, Position::X)] = chip.v[(opcode, Position::Y)];
    chip.program_counter.increment();
}

fn or(opcode: Opcode, chip: &mut Chip) {
    chip.v[(opcode, Position::X)] |= chip.v[(opcode, Position::Y)];
    chip.program_counter.increment();
}

fn and(opcode: Opcode, chip: &mut Chip) {
    chip.v[(opcode, Position::X)] &= chip.v[(opcode, Position::Y)];
    chip.program_counter.increment();
}

fn xor(opcode: Opcode, chip: &mut Chip) {
    chip.v[(opcode, Position::X)] ^= chip.v[(opcode, Position::Y)];
    chip.program_counter.increment();
}

fn addreg(opcode: Opcode, chip: &mut Chip) {
    let left_index = Registers::get_index(opcode, Position::X);
    let right_index = Registers::get_index(opcode, Position::Y);

    let carried = chip.v.add_reg(left_index, right_index);

    if carried {
        chip.v[0xF] = 1;
    } else {
        chip.v[0xF] = 0;
    }

    chip.program_counter.increment();
}

fn subreg(opcode: Opcode, chip: &mut Chip) {
    
}

#[cfg(test)]
mod test;
