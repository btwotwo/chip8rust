use super::*;

fn prepare() -> Chip {
    Chip::new()
}

#[test]
fn ret_test() {
    let mut chip = prepare();

    chip.stack_pointer = 10;
    chip.stack[chip.stack_pointer as usize] = 0x1234;

    OpcodeHandler::ret(0x00EE, &mut chip);

    assert_eq!(chip.program_counter, 0x1234);
    assert_eq!(chip.stack_pointer, 9);
}

#[test]
fn jp_test() {
    let mut chip = prepare();
    chip.program_counter.set(0xABC);

    OpcodeHandler::jp(0x1DEA, &mut chip);

    assert_eq!(chip.program_counter, 0xDEA)
}

#[test]
fn call_test() {
    let mut chip = prepare();
    chip.program_counter.set(12);

    let opcode = 0x2DEA;

    OpcodeHandler::call(0x2DEA, &mut chip);

    assert_eq!(chip.program_counter, 0xDEA);
    assert_eq!(chip.stack_pointer, 1);
    assert_eq!(chip.stack[chip.stack_pointer as usize], 12);
}

#[test]
fn se_test_neq() {
    let mut chip = prepare();
    chip.program_counter.set(12);
    chip.v[5] = 0x10;

    OpcodeHandler::se(0x3512, &mut chip);

    assert_eq!(chip.program_counter, 12);
}

#[test]
fn se_test_eq() {
    let mut chip = prepare();
    chip.program_counter.set(12);
    chip.v[4] = 0x12;

    OpcodeHandler::se(0x3412, &mut chip);

    assert_eq!(chip.program_counter, 14);
}

#[test]
fn sne_test_eq() {
    let mut chip = prepare();
    chip.program_counter.set(12);
    chip.v[4] = 0x12;

    OpcodeHandler::sne(0x4412, &mut chip);

    assert_eq!(chip.program_counter, 12);
}

#[test]
fn sne_test_neq() {
    let mut chip = prepare();

    chip.program_counter.set(12);

    chip.v[4] = 0x13;

    OpcodeHandler::sne(0x4412, &mut chip);

    assert_eq!(chip.program_counter, 14);
}

#[test]
fn sre_test() {
    let mut chip = prepare();

    chip.program_counter.set(12);

    chip.v[5] = 0x12;
    chip.v[6] = 0x13;

    OpcodeHandler::sre(0x5460, &mut chip);

    assert_eq!(chip.program_counter, 12)
}

#[test]
fn sre_test_eq() {
    let mut chip = prepare();

    chip.program_counter.set(12);

    chip.v[4] = 0x12;
    chip.v[5] = 0x12;

    OpcodeHandler::sre(0x5450, &mut chip);

    assert_eq!(chip.program_counter, 14);
}

#[test]
fn ld_test() {
    let mut chip = prepare();

    chip.v[0xE] = 0xAB;

    OpcodeHandler::ld(0x6EAB, &mut chip);

    assert_eq!(chip.v[0xE], 0xAB);
}

#[test]
fn add_test() {
    let mut chip = prepare();

    chip.v[0x5] = 0x1;

    OpcodeHandler::add(0x75AB, &mut chip);

    assert_eq!(chip.v[0x5], 0xAC);
}

#[test]
fn add_overflow_test() {
    let mut chip = prepare();

    chip.v[0x5] = 0xFF;

    OpcodeHandler::add(0x7502, &mut chip);

    assert_eq!(chip.v[0x5], 0x1);
}

#[test]
fn ldr_test() {
    let mut chip = prepare();
    chip.v[1] = 111;
    chip.v[2] = 222;

    OpcodeHandler::ldr(0x8120, &mut chip);

    assert_eq!(chip.v[1], 222);
}

#[test]
fn or_test() {
    let mut chip = prepare();
    chip.v[1] = 0x12;
    chip.v[2] = 0x34;

    OpcodeHandler::or(0x8121, &mut chip);
    assert_eq!(chip.v[1], 0x36);
    assert_eq!(chip.v[2], 0x34);
}

#[test]
fn and_test() {
    let mut chip = prepare();

    chip.v[1] = 0x12;
    chip.v[2] = 0x34;

    OpcodeHandler::and(0x8122, &mut chip);

    assert_eq!(chip.v[1], 0x10);
    assert_eq!(chip.v[2], 0x34);
}

#[test]
fn xor_test() {
    let mut chip = prepare();

    chip.v[1] = 0x12;
    chip.v[2] = 0x34;

    OpcodeHandler::xor(0x8123, &mut chip);

    assert_eq!(chip.v[1], 0x26);
    assert_eq!(chip.v[2], 0x34);
}

#[test]
fn addreg_test() {
    let mut chip = prepare();

    chip.v[0] = 0x20;
    chip.v[1] = 0x01;
    chip.v[0xF] = 1;

    OpcodeHandler::addreg(0x8014, &mut chip);

    assert_eq!(chip.v[0], 0x21);
    assert_eq!(chip.v[1], 0x01);
    assert_eq!(chip.v[0xF], 0);
}

#[test]
fn addreg_carry_test() {
    let mut chip = prepare();

    chip.v[0] = 0xFF;
    chip.v[1] = 0x02;
    chip.v[0xF] = 0;

    OpcodeHandler::addreg(0x8014, &mut chip);

    assert_eq!(chip.v[0], 0x01);
    assert_eq!(chip.v[1], 0x02);
    assert_eq!(chip.v[0xF], 1);
}

#[test]
fn subreg_test() {
    let mut chip = prepare();

    chip.v[0] = 0x20;
    chip.v[1] = 0x01;
    chip.v[0xF] = 1;

    OpcodeHandler::subreg(0x8015, &mut chip);

    assert_eq!(chip.v[0], 0x1F);
    assert_eq!(chip.v[1], 0x01);
    assert_eq!(chip.v[0xF], 0);
}

#[test]
fn subreg_carry_test() {
    let mut chip = prepare();

    chip.v[0] = 0x00;
    chip.v[1] = 0x01;
    chip.v[0xF] = 0;

    OpcodeHandler::subreg(0x8015, &mut chip);

    assert_eq!(chip.v[0], 0xFF);
    assert_eq!(chip.v[1], 0x01);
    assert_eq!(chip.v[0xF], 1);
}

#[test]
fn shiftr_test() {
    let mut chip = prepare();

    chip.v[0] = 0x10;
    chip.v[0xF] = 1;

    OpcodeHandler::shiftr(0x8016, &mut chip);

    assert_eq!(chip.v[0], 0x8);
    assert_eq!(chip.v[0xF], 0);
}

#[test]
fn sub_test() {
    let mut chip = prepare();

    chip.v[0] = 0;
    chip.v[1] = 1;
    chip.v[0xF] = 1;

    OpcodeHandler::sub(0x8107, &mut chip);

    assert_eq!(chip.v[1], 255);
    assert_eq!(chip.v[0xF], 1);
}

#[test]
fn sub_test_overflow() {
    let mut chip = prepare();

    chip.v[0] = 0;
    chip.v[1] = 1;
    chip.v[0xF] = 1;

    OpcodeHandler::sub(0x8017, &mut chip);

    assert_eq!(chip.v[0], 1);
    assert_eq!(chip.v[0xF], 0);
}

#[test]
fn shiftl_test_significant_one() {
    let mut chip = prepare();
    chip.v[0] = 0b100_0100_1;
    chip.v[0xf] = 0;

    OpcodeHandler::shiftl(0x801E, &mut chip);

    assert_eq!(chip.v[0], 0b0001_001_0);
    assert_eq!(chip.v[0xf], 1);
}

#[test]
fn srne_test_eq() {
    let mut chip = prepare();

    chip.program_counter.set(12);

    chip.v[1] = 0x12;
    chip.v[2] = 0x12;

    OpcodeHandler::srne(0x9120, &mut chip);

    assert_eq!(chip.program_counter, 12);
}

#[test]
fn srne_test_neq() {
    let mut chip = prepare();

    chip.program_counter.set(12);

    chip.v[1] = 0x12;
    chip.v[2] = 0x13;

    OpcodeHandler::srne(0x9120, &mut chip);

    assert_eq!(chip.program_counter, 14);
}

#[test]
fn jmpv0_test() {
    let mut chip = prepare();
    chip.v[0] = 0x0002;
    chip.program_counter.set(0x1);

    OpcodeHandler::jmpv0(0xA123, &mut chip);

    assert_eq!(chip.program_counter, 0x0125)
}
