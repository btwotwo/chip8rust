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
            0x9000 => OpcodeHandler::srne,
            0xA000 => OpcodeHandler::ldi,
            0xB000 => OpcodeHandler::jmpv0 
        );

        OpcodeHandler {
            opcode_map: map,
            current: 0x0000,
        }
    }

    pub fn next(&mut self, opcode: Opcode, chip: &mut Chip) {
        self.current = opcode;

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
            0x1000 | 0x2000 | 0x00EE | 0xB000 => (), // JP, CALL, RET, JMPv0 opcodes
            _ => chip.program_counter.increment(),
        };
    }

    /// `00EE` - Return from a subroutine
    fn ret(&self, chip: &mut Chip) {
        chip.program_counter
            .set(chip.stack[chip.stack_pointer as usize]);
        chip.stack_pointer -= 1;
    }

    ///`1NNN` - Jump to the address `NNN`
    fn jp(&self, chip: &mut Chip) {
        chip.program_counter.set(self.current & 0x0FFF);
    }

    ///`2NNN` - Call subroutine at `NNN`
    fn call(&self, chip: &mut Chip) {
        chip.stack_pointer += 1;
        chip.stack[chip.stack_pointer as usize] = chip.program_counter.get();
        chip.program_counter.set(self.current & 0x0FFF);
    }

    ///`3XNN` - Skip next instruction if V[`X`] equals `NN`
    fn se(&self, chip: &mut Chip) {
        let compare = (self.current & 0x00FF) as u8;
        let register = chip.v[(self.current, Position::X)];

        if register == compare {
            chip.program_counter.increment();
        }
    }

    ///`4XNN` - Skip next instruction if V[`X`] doesn't equal `NN`
    fn sne(&self, chip: &mut Chip) {
        let register = chip.v[(self.current, Position::X)];
        let to_compare = (self.current & 0x00FF) as u8;

        if register != to_compare {
            chip.program_counter.increment();
        }
    }

    ///`5XY0` - Skip if V[`X`] equal V[`Y`]
    fn sre(&self, chip: &mut Chip) {
        let first_register = chip.v[(self.current, Position::X)];
        let second_register = chip.v[(self.current, Position::Y)];

        if first_register == second_register {
            chip.program_counter.increment();
        }
    }

    ///`6XNN` - Set V[`X`] to `NN`
    fn ld(&self, chip: &mut Chip) {
        chip.v[(self.current, Position::X)] = (self.current & 0x00FF) as u8;
    }

    ///`7XNN` - Add `NN` to V[`X`], carry flag not changed
    fn add(&self, chip: &mut Chip) {
        let index = Registers::get_index(self.current, Position::X);
        chip.v.add_immediate(index, (self.current & 0x00FF) as u8);
    }

    ///`8XY0` Set V[`X`] to the value of V[`Y`]
    fn ldr(&self, chip: &mut Chip) {
        chip.v[(self.current, Position::X)] = chip.v[(self.current, Position::Y)];
    }

    ///`8XY1` - Set V[`X`] to the result of bitwise OR with V[`Y`] 
    fn or(&self, chip: &mut Chip) {
        chip.v[(self.current, Position::X)] |= chip.v[(self.current, Position::Y)];
    }

    ///`8XY2` - Set V[`X`] to the result of bitwise AND with V[`Y`] 
    fn and(&self, chip: &mut Chip) {
        chip.v[(self.current, Position::X)] &= chip.v[(self.current, Position::Y)];
    }

    ///`8XY3` - Set V[`X`] to the result of bitwise XOR with V[`Y`]
    fn xor(&self, chip: &mut Chip) {
        chip.v[(self.current, Position::X)] ^= chip.v[(self.current, Position::Y)];
    }

    ///`8XY4` - Add V[`Y`] to V[`X`], change carry flag if there's a borrow 
    fn addreg(&self, chip: &mut Chip) {
        let left = chip.v[(self.current, Position::X)];
        let right = chip.v[(self.current, Position::Y)];

        let (result, carried) = left.overflowing_add(right);

        chip.v[(self.current, Position::X)] = result;
        chip.v.set_carry(carried);
    }

    ///`8XY5` - Subtract V[`Y`] from V[`X`], change carry flag if there's a borrow 
    fn subreg(&self, chip: &mut Chip) {
        let (result, carried) = chip.v[(self.current, Position::X)]
            .overflowing_sub(chip.v[(self.current, Position::Y)]);

        chip.v[(self.current, Position::X)] = result;
        chip.v.set_carry(carried);
    }

    ///`8X06` - Store least significant bit of V[`X`] in VF and then shift V[`X`] to the right by 1 
    fn shiftr(&self, chip: &mut Chip) {
        chip.v[0xF] = chip.v[(self.current, Position::X)] & 1;
        chip.v[(self.current, Position::X)] >>= 1;
    }

    ///`8X07` - Sets V[`X`] to V[`Y`] minus V[`X`]. VF is set to 0 when there's a borrow, and 1 when there isn't. 
    fn sub(&self, chip: &mut Chip) {
        let x = chip.v[(self.current, Position::X)];
        let y = chip.v[(self.current, Position::Y)];
        let (result, carried) = y.overflowing_sub(x);

        chip.v[(self.current, Position::X)] = result;

        chip.v.set_carry(carried);
    }

    ///`8X0E` - Stores the most significant bit of V[`X`] in VF and then shifts V[`X`] to the left by 1 
    fn shiftl(&self, chip: &mut Chip) {
        let index = Registers::get_index(self.current, Position::X);
        chip.v[0xF] = (chip.v[index] >= 128) as u8;
        chip.v[index] <<= 1;
    }

    fn srne(&self, chip: &mut Chip) {
        if chip.v[(self.current, Position::X)] != chip.v[(self.current, Position::Y)] {
            chip.program_counter.increment();
        }
    }

    fn ldi(&self, chip: &mut Chip) {
        chip.i = self.current & 0x0FFF;
    }

    fn jmpv0(&self, chip: &mut Chip) {
        let address = u16::from(chip.v[0]) + (self.current & 0x0FFF);
        chip.program_counter.set(address);
    }
}

#[cfg(test)]
mod opcodes_tests;

#[cfg(test)]
mod handler_tests;