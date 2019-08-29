use super::*;


fn prepare(opcode: Opcode) -> (Chip, OpcodeHandler) {
    let mut handler = OpcodeHandler::new();

    handler.current = opcode;

    (Chip::new(), handler)
}

#[test]
fn ret_test() {
    let (mut chip, mut handler) = prepare(0x00EE);

    chip.stack_pointer = 10;
    chip.stack[chip.stack_pointer as usize] = 0x1234;

    handler.ret(&mut chip);

    assert_eq!(chip.program_counter, 0x1234);
    assert_eq!(chip.stack_pointer, 9);
}

#[test]
fn jp_test() {
    let (mut chip, mut handler) = prepare(0x1DEA);
    chip.program_counter.set(0xABC);

    handler.jp(&mut chip);

    assert_eq!(chip.program_counter, 0xDEA)
}

#[test]
fn call_test() {
    let (mut chip, mut handler) = prepare(0x2DEA);
    chip.program_counter.set(12);

    let opcode = 0x2DEA;

    handler.call(&mut chip);

    assert_eq!(chip.program_counter, 0xDEA);
    assert_eq!(chip.stack_pointer, 1);
    assert_eq!(chip.stack[chip.stack_pointer as usize], 12);
}

#[test]
fn se_test() {
    let (mut chip, mut handler) = prepare(0x3412);
    chip.program_counter.set(12);
    chip.v[4] = 0x12;
    chip.v[5] = 0x10;

    handler.se(&mut chip);

    assert_eq!(chip.program_counter, 16);

    handler.current = 0x3512;

    handler.se(&mut chip);

    assert_eq!(chip.program_counter, 18);
}

#[test]
fn sne_test() {
    let (mut chip, mut handler) = prepare(0x4412);
    chip.program_counter.set(12);
    chip.v[4] = 0x12;
    chip.v[5] = 0x10;

    handler.sne(&mut chip);

    assert_eq!(chip.program_counter, 14);

    handler.current = 0x4512;

    handler.sne(&mut chip);

    assert_eq!(chip.program_counter, 18);
}

#[test]
fn sre_test() {
    let (mut chip, mut handler) = prepare(0x5450);

    chip.program_counter.set(12);

    chip.v[4] = 0x12;
    chip.v[5] = 0x12;
    chip.v[6] = 0x13;

    handler.sre(&mut chip);

    assert_eq!(chip.program_counter, 16);

    handler.current = 0x5460;

    handler.sre(&mut chip);

    assert_eq!(chip.program_counter, 18)
}

#[test]
fn ld_test() {
    let (mut chip, mut handler) = prepare(0x6EAB);

    chip.v[0xE] = 0xAB;

    handler.ld(&mut chip);

    assert_eq!(chip.v[0xE], 0xAB);
    
}

#[test]
fn add_test() {
    let (mut chip, mut handler) = prepare(0x75AB);

    chip.v[0x5] = 0x1;

    handler.add(&mut chip);

    assert_eq!(chip.v[0x5], 0xAC);
    
}

#[test]
fn add_overflow_test() {
    let (mut chip, mut handler) = prepare(0x7502);

    chip.v[0x5] = 0xFF;

    handler.add(&mut chip);

    assert_eq!(chip.v[0x5], 0x1);
    
}

#[test]
fn ldr_test() {
    let (mut chip, mut handler) = prepare(0x8120);
    chip.v[1] = 111;
    chip.v[2] = 222;

    handler.ldr(&mut chip);

    assert_eq!(chip.v[1], 222);
    
}

#[test]
fn or_test() {
    let (mut chip, mut handler) = prepare(0x8121);
    chip.v[1] = 0x12;
    chip.v[2] = 0x34;

    handler.or(&mut chip);
    assert_eq!(chip.v[1], 0x36);
    assert_eq!(chip.v[2], 0x34);

    
}

#[test]
fn and_test() {
    let (mut chip, mut handler) = prepare(0x8122);

    chip.v[1] = 0x12;
    chip.v[2] = 0x34;

    handler.and(&mut chip);

    assert_eq!(chip.v[1], 0x10);
    assert_eq!(chip.v[2], 0x34);
    
}

#[test]
fn xor_test() {
    let (mut chip, mut handler) = prepare(0x8123);

    chip.v[1] = 0x12;
    chip.v[2] = 0x34;

    handler.xor(&mut chip);

    assert_eq!(chip.v[1], 0x26);
    assert_eq!(chip.v[2], 0x34);
    
}

#[test]
fn addreg_test() {
    let (mut chip, mut handler) = prepare(0x8014);

    chip.v[0] = 0x20;
    chip.v[1] = 0x01;
    chip.v[0xF] = 1;

    handler.addreg(&mut chip);

    assert_eq!(chip.v[0], 0x21);
    assert_eq!(chip.v[1], 0x01);
    assert_eq!(chip.v[0xF], 0);
    
}

#[test]
fn addreg_carry_test() {
    let (mut chip, mut handler) = prepare(0x8014);

    chip.v[0] = 0xFF;
    chip.v[1] = 0x02;
    chip.v[0xF] = 0;

    handler.addreg(&mut chip);

    assert_eq!(chip.v[0], 0x01);
    assert_eq!(chip.v[1], 0x02);
    assert_eq!(chip.v[0xF], 1);
    
}

#[test]
fn subreg_test() {
    let (mut chip, mut handler) = prepare(0x8015);

    chip.v[0] = 0x20;
    chip.v[1] = 0x01;
    chip.v[0xF] = 1;

    handler.subreg(&mut chip);

    assert_eq!(chip.v[0], 0x1F);
    assert_eq!(chip.v[1], 0x01);
    assert_eq!(chip.v[0xF], 0);
    
}

#[test]
fn subreg_carry_test() {
    let (mut chip, mut handler) = prepare(0x8015);

    chip.v[0] = 0x00;
    chip.v[1] = 0x01;
    chip.v[0xF] = 0;

    handler.subreg(&mut chip);

    assert_eq!(chip.v[0], 0xFF);
    assert_eq!(chip.v[1], 0x01);
    assert_eq!(chip.v[0xF], 1);
    
}

#[test]
fn shiftr_test() {
    let (mut chip, mut handler) = prepare(0x8016);

    chip.v[0] = 0x10;
    chip.v[0xF] = 1;

    handler.shiftr(&mut chip);

    assert_eq!(chip.v[0], 0x8);
    assert_eq!(chip.v[0xF], 0);
    
}

#[test]
fn sub_test() {
    let (mut chip, mut handler) = prepare(0x8107);

    chip.v[0] = 0;
    chip.v[1] = 1;
    chip.v[0xF] = 1;

    handler.sub(&mut chip);

    assert_eq!(chip.v[1], 255);
    assert_eq!(chip.v[0xF], 1);
}

#[test]
fn sub_test_overflow() {
    let (mut chip, mut handler) = prepare(0x8017);

    chip.v[0] = 0;
    chip.v[1] = 1;
    chip.v[0xF] = 1;

    handler.sub(&mut chip);

    assert_eq!(chip.v[0], 1);
    assert_eq!(chip.v[0xF], 0);
}

#[test]
fn shiftl_test_significant_one() {
    let (mut chip, mut handler) = prepare(0x801E);
    chip.v[0] = 0b100_0100_1;
    chip.v[0xf] = 0;

    handler.shiftl(&mut chip);

    assert_eq!(chip.v[0], 0b0001_001_0);
    assert_eq!(chip.v[0xf], 1);
}
