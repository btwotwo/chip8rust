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
            0x8004 => addreg,
            0x8005 => subreg,
            0x8006 => shiftr
        );

        OpcodeHandler { opcode_map: map }
    }
}

///Return from a subroutine (00EE)
fn ret(_: Opcode, chip: &mut Chip) {
    chip.program_counter
        .set(chip.stack[chip.stack_pointer as usize]);
    chip.stack_pointer -= 1;
}

///Jump to address NNN (1NNN)
fn jp(opcode: Opcode, chip: &mut Chip) {
    chip.program_counter.set(opcode & 0x0FFF);
}

///Call subroutine at NNN (2NNN)
fn call(opcode: Opcode, chip: &mut Chip) {
    chip.stack_pointer += 1;
    chip.stack[chip.stack_pointer as usize] = chip.program_counter.get();
    chip.program_counter.set(opcode & 0x0FFF);
}

//Skip if Vx equals NN (3XNN)
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

//Skip if Vx doesn't equal NN (4XNN)
fn sne(opcode: Opcode, chip: &mut Chip) {
    let register = chip.v[(opcode, Position::X)];
    let to_compare = (opcode & 0x00FF) as u8;

    if register != to_compare {
        chip.program_counter.skip(1);
    } else {
        chip.program_counter.increment();
    }
}

///Skip if Vx equal Vy (5XY0)
fn sre(opcode: Opcode, chip: &mut Chip) {
    let first_register = chip.v[(opcode, Position::X)];
    let second_register = chip.v[(opcode, Position::Y)];

    if first_register == second_register {
        chip.program_counter.skip(1);
    } else {
        chip.program_counter.increment();
    }
}

///Set Vx to NN (6XNN)
fn ld(opcode: Opcode, chip: &mut Chip) {
    chip.v[(opcode, Position::X)] = (opcode & 0x00FF) as u8;
    chip.program_counter.increment();
}

///Add NN to Vx, carry flag not changed (7XNN)
fn add(opcode: Opcode, chip: &mut Chip) {
    let index = Registers::get_index(opcode, Position::X);
    chip.v.add_immediate(index, (opcode & 0x00FF) as u8);
    chip.program_counter.increment();
}

///Set Vx to the value of Vy (8XY0)
fn ldr(opcode: Opcode, chip: &mut Chip) {
    chip.v[(opcode, Position::X)] = chip.v[(opcode, Position::Y)];
    chip.program_counter.increment();
}

///Set Vx to the result of bitwise OR with Vy (8XY1)
fn or(opcode: Opcode, chip: &mut Chip) {
    chip.v[(opcode, Position::X)] |= chip.v[(opcode, Position::Y)];
    chip.program_counter.increment();
}

///Set Vx to the result of bitwise AND with Vy (8XY2)
fn and(opcode: Opcode, chip: &mut Chip) {
    chip.v[(opcode, Position::X)] &= chip.v[(opcode, Position::Y)];
    chip.program_counter.increment();
}

///Set Vx to the result of bitwise XOR with Vy(8XY3)
fn xor(opcode: Opcode, chip: &mut Chip) {
    chip.v[(opcode, Position::X)] ^= chip.v[(opcode, Position::Y)];
    chip.program_counter.increment();
}

///Add Vy to VX, change carry flag if there's a borrow (8XY4)
fn addreg(opcode: Opcode, chip: &mut Chip) {
    let left = chip.v[(opcode, Position::X)];
    let right = chip.v[(opcode, Position::Y)];

    let (result, carried) = left.overflowing_add(right);

    chip.v[(opcode, Position::X)] = result;
    chip.v.set_carry(carried);
    chip.program_counter.increment();
}

///Subtract Vy from Vx, change carry flag if there's a borrow (8XY5)
fn subreg(opcode: Opcode, chip: &mut Chip) {
    let (result, carried) =
        chip.v[(opcode, Position::X)].overflowing_sub(chip.v[(opcode, Position::Y)]);

    chip.v[(opcode, Position::X)] = result;
    chip.v.set_carry(carried);
    chip.program_counter.increment();
}

///Store least significant bit of Vx in VF and then shift Vx to the right by 1
fn shiftr(opcode: Opcode, chip: &mut Chip) {
    chip.v[0xF] = chip.v[(opcode, Position::X)] & 1;
    chip.v[(opcode, Position::X)] >>= 1;
    chip.program_counter.increment();
}

fn sub(opcode: Opcode, chip: &mut Chip) {
    let x = chip.v[(opcode, Position::X)];
    let y = chip.v[(opcode, Position::Y)];
    let (result, carried) = y.overflowing_sub(x);

    chip.v[(opcode, Position::X)] = result;

    chip.v.set_carry(carried);
    chip.program_counter.increment();
}

#[cfg(test)]
mod test;
