use super::registers::{Position, Registers};
use super::Chip;
use lazy_static::lazy_static;
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
type OpcodeImpl = fn(Opcode, &mut Chip);

lazy_static! {
    pub static ref OPCODE_MAP: HashMap<Opcode, OpcodeImpl> = opcode_func_map!(
        0x00E0 => OpcodeHandler::clear,
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
        0xB000 => OpcodeHandler::jmpv0,
        0xC000 => OpcodeHandler::rand,
        0xD000 => OpcodeHandler::print,
        0xE09E => OpcodeHandler::skp,
        0xE0A1 => OpcodeHandler::sknp,
        0xF007 => OpcodeHandler::ldvxdt,
        0xF00A => OpcodeHandler::ldvxkey,
        0xF015 => OpcodeHandler::lddtvx,
        0xF018 => OpcodeHandler::ldstvs,
        0xF01E => OpcodeHandler::addivx,
        0xF029 => OpcodeHandler::ldfvx,
        0xF033 => OpcodeHandler::bcd,
        0xF055 => OpcodeHandler::ldiv0vx,
        0xF065 => OpcodeHandler::ldv0vxi
    );
}

pub struct OpcodeHandler;

impl OpcodeHandler {
    pub fn next(opcode: Opcode, chip: &mut Chip) {
        let normalized_opcode = opcode & 0xF000;
        let normalized_opcode = match normalized_opcode {
            0x0000 => opcode & 0x00FF,
            0x8000 => opcode & 0xF00F,
            0xE000 | 0xF000 => opcode & 0xF0FF,
            _ => normalized_opcode,
        };

        // println!("{:?}", normalized_opcode);

        match OPCODE_MAP.get(&normalized_opcode) {
            Some(func) => func(opcode, chip),
            None => panic!("Wrong opcode! {}", opcode),
        };

        match normalized_opcode {
            0x1000 | 0x2000 | 0x00EE | 0xB000 => (), // JP, CALL, RET, JMPv0 opcodes
            _ => chip.program_counter.increment(),
        };
    }
    fn clear(opcode: Opcode, chip: &mut Chip) {
        chip.screen.clear();
    }

    /// `00EE` - Return from a subroutine
    fn ret(_opcode: Opcode, chip: &mut Chip) {
        chip.program_counter
            .set(chip.stack[chip.stack_pointer as usize]);
        chip.stack_pointer -= 1;
    }

    ///`1NNN` - Jump to the address `NNN`
    fn jp(opcode: Opcode, chip: &mut Chip) {
        chip.program_counter.set(opcode & 0x0FFF);
    }

    ///`2NNN` - Call subroutine at `NNN`
    fn call(opcode: Opcode, chip: &mut Chip) {
        chip.stack_pointer += 1;
        chip.stack[chip.stack_pointer as usize] = chip.program_counter.get();
        chip.program_counter.set(opcode & 0x0FFF);
    }

    ///`3XNN` - Skip next instruction if V[`X`] equals `NN`
    fn se(opcode: Opcode, chip: &mut Chip) {
        let compare = (opcode & 0x00FF) as u8;
        let register = chip.v[(opcode, Position::X)];

        if register == compare {
            chip.program_counter.increment();
        }
    }

    ///`4XNN` - Skip next instruction if V[`X`] doesn't equal `NN`
    fn sne(opcode: Opcode, chip: &mut Chip) {
        let register = chip.v[(opcode, Position::X)];
        let to_compare = (opcode & 0x00FF) as u8;

        if register != to_compare {
            chip.program_counter.increment();
        }
    }

    ///`5XY0` - Skip if V[`X`] equal V[`Y`]
    fn sre(opcode: Opcode, chip: &mut Chip) {
        let first_register = chip.v[(opcode, Position::X)];
        let second_register = chip.v[(opcode, Position::Y)];

        if first_register == second_register {
            chip.program_counter.increment();
        }
    }

    ///`6XNN` - Set V[`X`] to `NN`
    fn ld(opcode: Opcode, chip: &mut Chip) {
        chip.v[(opcode, Position::X)] = (opcode & 0x00FF) as u8;
    }

    ///`7XNN` - Add `NN` to V[`X`], carry flag not changed
    fn add(opcode: Opcode, chip: &mut Chip) {
        let index = Registers::get_index(opcode, Position::X);
        chip.v.add_immediate(index, (opcode & 0x00FF) as u8);
    }

    ///`8XY0` Set V[`X`] to the value of V[`Y`]
    fn ldr(opcode: Opcode, chip: &mut Chip) {
        chip.v[(opcode, Position::X)] = chip.v[(opcode, Position::Y)];
    }

    ///`8XY1` - Set V[`X`] to the result of bitwise OR with V[`Y`]
    fn or(opcode: Opcode, chip: &mut Chip) {
        chip.v[(opcode, Position::X)] |= chip.v[(opcode, Position::Y)];
    }

    ///`8XY2` - Set V[`X`] to the result of bitwise AND with V[`Y`]
    fn and(opcode: Opcode, chip: &mut Chip) {
        chip.v[(opcode, Position::X)] &= chip.v[(opcode, Position::Y)];
    }

    ///`8XY3` - Set V[`X`] to the result of bitwise XOR with V[`Y`]
    fn xor(opcode: Opcode, chip: &mut Chip) {
        chip.v[(opcode, Position::X)] ^= chip.v[(opcode, Position::Y)];
    }

    ///`8XY4` - Add V[`Y`] to V[`X`], change carry flag if there's a borrow
    fn addreg(opcode: Opcode, chip: &mut Chip) {
        let left = chip.v[(opcode, Position::X)];
        let right = chip.v[(opcode, Position::Y)];

        let (result, carried) = left.overflowing_add(right);

        chip.v[(opcode, Position::X)] = result;
        chip.v.set_carry(carried);
    }

    ///`8XY5` - Subtract V[`Y`] from V[`X`], change carry flag if there's a borrow
    fn subreg(opcode: Opcode, chip: &mut Chip) {
        let x = chip.v[(opcode, Position::X)];
        let y = chip.v[(opcode, Position::Y)];
        
        let (result, carried) = x.overflowing_sub(y);

        chip.v[(opcode, Position::X)] = result;
        chip.v.set_carry(!carried);
    }

    ///`8XY6` - Store least significant bit of V[`X`] in VF and then shift V[`X`] to the right by 1
    fn shiftr(opcode: Opcode, chip: &mut Chip) {
        chip.v[0xF] = chip.v[(opcode, Position::X)] & 1;
        chip.v[(opcode, Position::X)] >>= 1;
    }

    ///`8XY7` - Sets V[`X`] to V[`Y`] minus V[`X`]. VF is set to 0 when there's a borrow, and 1 when there isn't.
    fn sub(opcode: Opcode, chip: &mut Chip) {
        let x = chip.v[(opcode, Position::X)];
        let y = chip.v[(opcode, Position::Y)];
        let (result, carried) = y.overflowing_sub(x);

        chip.v[(opcode, Position::X)] = result;

        chip.v.set_carry(!carried);
    }

    ///`8XYE` - Stores the most significant bit of V[`X`] in VF and then shifts V[`X`] to the left by 1
    fn shiftl(opcode: Opcode, chip: &mut Chip) {
        let index = Registers::get_index(opcode, Position::X);
        chip.v[0xF] = (chip.v[index] >= 128) as u8;
        chip.v[index] <<= 1;
    }

    ///`9XY0` - Skips the next instruction if V[`X`] does not equal V[`Y`]
    fn srne(opcode: Opcode, chip: &mut Chip) {
        if chip.v[(opcode, Position::X)] != chip.v[(opcode, Position::Y)] {
            chip.program_counter.increment();
        }
    }

    ///`ANNN` - Set I to address NNN
    fn ldi(opcode: Opcode, chip: &mut Chip) {
        chip.i = opcode & 0x0FFF;
    }

    ///`BNNN` - Jump to the address NNN plus V[0]
    fn jmpv0(opcode: Opcode, chip: &mut Chip) {
        let address = u16::from(chip.v[0]) + (opcode & 0x0FFF);
        chip.program_counter.set(address);
    }

    ///`CXNN` - Set V[`X`] equal `random_number & NN`
    fn rand(opcode: Opcode, chip: &mut Chip) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let random = rng.gen_range(0, 256) as u8;
        let nn = (opcode & 0x00FF) as u8;

        chip.v[(opcode, Position::X)] = random & nn;
    }

    ///`DXYN` - Display `N`-byte sprite, starting at I, at (V[`X`], V[`Y`]), set V[F] if collision occured
    fn print(opcode: Opcode, chip: &mut Chip) {
        let x = chip.v[(opcode, Position::X)];
        let y = chip.v[(opcode, Position::Y)];

        let n = opcode & 0x000F;
        let sprites: Vec<&u8> = chip
            .memory
            .iter()
            .skip(chip.i as usize)
            .take(n as usize)
            .collect();

        chip.v[0xF] = chip.screen.draw(x, y, &sprites) as u8;
    }

    ///`EX9E` - Skip the next instruction if the V[`X`] key is pressed.
    fn skp(opcode: Opcode, chip: &mut Chip) {
        let x = chip.v[(opcode, Position::X)];

        if chip.keyboard.is_pressed(x) {
            chip.program_counter.increment();
        }
    }

    ///`EXA1` - Skip the next instruction if the V[`X`] key is not pressed.
    fn sknp(opcode: Opcode, chip: &mut Chip) {
        let x = chip.v[(opcode, Position::X)];

        if !chip.keyboard.is_pressed(x) {
            chip.program_counter.increment();
        }
    }

    ///`FX07` - Set V[`X`] equal to delay timer.
    fn ldvxdt(opcode: Opcode, chip: &mut Chip) {
        chip.v[(opcode, Position::X)] = chip.delay_timer;
    }

    ///`FX0A` - Wait for a key press, store the value of the key in V[`X`]
    fn ldvxkey(opcode: Opcode, chip: &mut Chip) {
        let key = chip.keyboard.wait_for_key();

        chip.v[(opcode, Position::X)] = key;
    }

    ///`FX15` - Set delay timer equal to V[`X`]
    fn lddtvx(opcode: Opcode, chip: &mut Chip) {
        chip.delay_timer = chip.v[(opcode, Position::X)];
    }

    ///`FX18` - Set sound timer equal to V[`X`]
    fn ldstvs(opcode: Opcode, chip: &mut Chip) {
        chip.sound_timer = chip.v[(opcode, Position::X)];
    }

    ///`FX1E` - Set I equal to V[`X`] + I
    fn addivx(opcode: Opcode, chip: &mut Chip) {
        let to_add = u16::from(chip.v[(opcode, Position::X)]);
        let (result, carried) = chip.i.overflowing_add(to_add);

        chip.i = result;
        chip.v.set_carry(carried);
    }

    ///`FX29` - Set I equal to sprite location for digit V[`X`]
    fn ldfvx(opcode: Opcode, chip: &mut Chip) {
        let num = 5 * chip.v[(opcode, Position::X)];
        chip.i = num.into()
    }

    ///`FX33` - Store BCD representation of V[`X`] to I, I+1, I+2
    fn bcd(opcode: Opcode, chip: &mut Chip) {
        let vx_val = chip.v[(opcode, Position::X)];
        let onemial = vx_val % 10;
        let decimal: u8 = (vx_val / 10) % 10;
        let hundred: u8 = (vx_val / 100) % 10;

        chip.memory[chip.i as usize] = hundred;
        chip.memory[(chip.i + 1) as usize] = decimal;
        chip.memory[(chip.i + 2) as usize] = onemial;
    }

    ///`FX55` - Store registers V0 through V[`X`] in memory starting at location I.`
    fn ldiv0vx(opcode: Opcode, chip: &mut Chip) {
        for idx in 0..=Registers::get_index(opcode, Position::X) {
            chip.memory[(chip.i + u16::from(idx)) as usize] = chip.v[idx];
        }
    }

    ///`FX65` - Read registers V0 through V[`X`] from memory starting at location I.
    fn ldv0vxi(opcode: Opcode, chip: &mut Chip) {
        for idx in 0..=Registers::get_index(opcode, Position::X) {
            chip.v[idx] = chip.memory[(chip.i + u16::from(idx)) as usize];
        }
    }
}

#[cfg(test)]
mod opcodes_tests;

#[cfg(test)]
mod handler_tests;
