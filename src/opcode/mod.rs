use super::registers::{Position, Registers};
use super::Chip;
use std::collections::HashMap;

macro_rules! opcode_func_map {
    ($($opcode:expr => $func: expr), *) => {
        {
            let mut map = HashMap::<Opcode, OpcodeImpl>::new();
            $(
                map.insert($opcode, $func);
            )*
            map
        }
    };
}

pub type Opcode = u16;
pub type ShouldIncrementPc = bool;
type OpcodeImpl = fn(&OpcodeHandler, &mut Chip);

struct OpcodeHandler {
    opcode_map: HashMap<Opcode, OpcodeImpl>,
    pub current: Opcode,
}

impl OpcodeHandler {
    pub fn new() -> OpcodeHandler {
        let map = opcode_func_map!(
            0x00EE => OpcodeHandler::ret,
            0x1000 => OpcodeHandler::jp,
            0x2000 => OpcodeHandler::call,
            0x3000 => OpcodeHandler::se,
            0x4000 => OpcodeHandler::sne,
            0x5000 => OpcodeHandler::sre,
            0x6000 => OpcodeHandler::ld,
            0x7000 => OpcodeHandler::add,
            0x8000 => OpcodeHandler::ldr,
            0x8001 => OpcodeHandler::or,
            0x8002 => OpcodeHandler::and,
            0x8003 => OpcodeHandler::xor,
            0x8004 => OpcodeHandler::addreg,
            0x8005 => OpcodeHandler::subreg,
            0x8006 => OpcodeHandler::shiftr,
            0x8007 => OpcodeHandler::sub,
            0x800E => OpcodeHandler::shiftl,
            0x9000 => OpcodeHandler::snexy
        );

        OpcodeHandler {
            opcode_map: map,
            current: 0x0000,
        }
    }

    pub fn next(&self, opcode: Opcode, chip: &mut Chip) {
        let normalized_opcode = opcode & 0xF000;
        let normalized_opcode = match normalized_opcode {
            0x0000 => opcode & 0x00FF,
            0x8000 => opcode & 0xF00F,
            0xE000 | 0xF000 => opcode & 0xF0FF,
            _ => normalized_opcode,
        };

        match self.opcode_map.get(&normalized_opcode) {
            Some(func) => func(self, chip),
            None => {
                println!("Opcode {} not found! Skipping...", opcode);
            }
        };

        match normalized_opcode {
            0x1000 | 0x2000 => (), // JP and CALL opcodes
            _ => chip.program_counter.increment(),
        };
    }

    ///Return from a subroutine (00EE)
    fn ret(&self, chip: &mut Chip) {
        chip.program_counter
            .set(chip.stack[chip.stack_pointer as usize]);
        chip.stack_pointer -= 1;
    }

    ///Jump to address NNN (1NNN)
    fn jp(&self, chip: &mut Chip) {
        chip.program_counter.set(self.current & 0x0FFF);
    }

    ///Call subroutine at NNN (2NNN)
    fn call(&self, chip: &mut Chip) {
        chip.stack_pointer += 1;
        chip.stack[chip.stack_pointer as usize] = chip.program_counter.get();
        chip.program_counter.set(self.current & 0x0FFF);
    }

    //Skip if Vx equals NN (3XNN)
    fn se(&self, chip: &mut Chip) {
        let compare = (self.current & 0x00FF) as u8;
        let register = chip.v[(self.current, Position::X)];

        if register == compare {
            chip.program_counter.increment();
        }
    }

    //Skip if Vx doesn't equal NN (4XNN)
    fn sne(&self, chip: &mut Chip) {
        let register = chip.v[(self.current, Position::X)];
        let to_compare = (self.current & 0x00FF) as u8;

        if register != to_compare {
            chip.program_counter.increment();
        }
    }

    ///Skip if Vx equal Vy (5XY0)
    fn sre(&self, chip: &mut Chip) {
        let first_register = chip.v[(self.current, Position::X)];
        let second_register = chip.v[(self.current, Position::Y)];

        if first_register == second_register {
            chip.program_counter.increment();
        }
    }

    ///Set Vx to NN (6XNN)
    fn ld(&self, chip: &mut Chip) {
        chip.v[(self.current, Position::X)] = (self.current & 0x00FF) as u8;
    }

    ///Add NN to Vx, carry flag not changed (7XNN)
    fn add(&self, chip: &mut Chip) {
        let index = Registers::get_index(self.current, Position::X);
        chip.v.add_immediate(index, (self.current & 0x00FF) as u8);
    }

    ///Set Vx to the value of Vy (8XY0)
    fn ldr(&self, chip: &mut Chip) {
        chip.v[(self.current, Position::X)] = chip.v[(self.current, Position::Y)];
    }

    ///Set Vx to the result of bitwise OR with Vy (8XY1)
    fn or(&self, chip: &mut Chip) {
        chip.v[(self.current, Position::X)] |= chip.v[(self.current, Position::Y)];
    }

    ///Set Vx to the result of bitwise AND with Vy (8XY2)
    fn and(&self, chip: &mut Chip) {
        chip.v[(self.current, Position::X)] &= chip.v[(self.current, Position::Y)];
    }

    ///Set Vx to the result of bitwise XOR with Vy(8XY3)
    fn xor(&self, chip: &mut Chip) {
        chip.v[(self.current, Position::X)] ^= chip.v[(self.current, Position::Y)];
    }

    ///Add Vy to VX, change carry flag if there's a borrow (8XY4)
    fn addreg(&self, chip: &mut Chip) {
        let left = chip.v[(self.current, Position::X)];
        let right = chip.v[(self.current, Position::Y)];

        let (result, carried) = left.overflowing_add(right);

        chip.v[(self.current, Position::X)] = result;
        chip.v.set_carry(carried);
    }

    ///Subtract Vy from Vx, change carry flag if there's a borrow (8XY5)
    fn subreg(&self, chip: &mut Chip) {
        let (result, carried) = chip.v[(self.current, Position::X)]
            .overflowing_sub(chip.v[(self.current, Position::Y)]);

        chip.v[(self.current, Position::X)] = result;
        chip.v.set_carry(carried);
    }

    ///Store least significant bit of Vx in VF and then shift Vx to the right by 1 (8X06)
    fn shiftr(&self, chip: &mut Chip) {
        chip.v[0xF] = chip.v[(self.current, Position::X)] & 1;
        chip.v[(self.current, Position::X)] >>= 1;
    }

    ///Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't. (8X07)
    fn sub(&self, chip: &mut Chip) {
        let x = chip.v[(self.current, Position::X)];
        let y = chip.v[(self.current, Position::Y)];
        let (result, carried) = y.overflowing_sub(x);

        chip.v[(self.current, Position::X)] = result;

        chip.v.set_carry(carried);
    }

    ///Stores the most significant bit of VX in VF and then shifts VX to the left by 1 (8X0E)
    fn shiftl(&self, chip: &mut Chip) {
        let index = Registers::get_index(self.current, Position::X);
        chip.v[0xF] = (chip.v[index] >= 128) as u8;
        chip.v[index] <<= 1;
    }

    fn snexy(&self, chip: &mut Chip) {
        if chip.v[(self.current, Position::X)] != chip.v[(self.current, Position::Y)] {
            chip.program_counter.increment();
        }
    }
}

#[cfg(test)]
mod test;
