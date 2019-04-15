use super::*;

fn assert_pc_increment(chip: &Chip) {
    assert_eq!(chip.program_counter, 514);
}

#[test]
fn ret_test() {
    let mut chip = Chip::new();
    chip.stack_pointer = 10;
    chip.stack[chip.stack_pointer as usize] = 0x1234;

    ret(0x00EE, &mut chip);

    assert_eq!(chip.program_counter, 0x1234);
    assert_eq!(chip.stack_pointer, 9);
}

#[test]
fn jp_test() {
    let mut chip = Chip::new();
    chip.program_counter.set(0xABC);

    jp(0x1DEA, &mut chip);

    assert_eq!(chip.program_counter, 0xDEA)
}

#[test]
fn call_test() {
    let mut chip = Chip::new();
    chip.program_counter.set(12);

    let opcode = 0x2DEA;

    call(0x2DEA, &mut chip);

    assert_eq!(chip.program_counter, 0xDEA);
    assert_eq!(chip.stack_pointer, 1);
    assert_eq!(chip.stack[chip.stack_pointer as usize], 12);
}

#[test]
fn se_test() {
    let mut chip = Chip::new();
    chip.program_counter.set(12);
    chip.v[4] = 0x12;
    chip.v[5] = 0x10;

    se(0x3412, &mut chip);

    assert_eq!(chip.program_counter, 16);

    se(0x3512, &mut chip);

    assert_eq!(chip.program_counter, 18);
}

#[test]
fn sne_test() {
    let mut chip = Chip::new();
    chip.program_counter.set(12);
    chip.v[4] = 0x12;
    chip.v[5] = 0x10;

    sne(0x4412, &mut chip);

    assert_eq!(chip.program_counter, 14);

    sne(0x4512, &mut chip);

    assert_eq!(chip.program_counter, 18);
}

#[test]
fn sre_test() {
    let mut chip = Chip::new();

    chip.program_counter.set(12);

    chip.v[4] = 0x12;
    chip.v[5] = 0x12;
    chip.v[6] = 0x13;

    sre(0x5450, &mut chip);

    assert_eq!(chip.program_counter, 16);

    sre(0x5460, &mut chip);

    assert_eq!(chip.program_counter, 18)
}

#[test]
fn ld_test() {
    let mut chip = Chip::new();

    chip.v[0xE] = 0xAB;

    ld(0x6EAB, &mut chip);

    assert_eq!(chip.v[0xE], 0xAB);
    assert_pc_increment(&mut chip);
}

#[test]
fn add_test() {
    let mut chip = Chip::new();

    chip.v[0x5] = 0x1;

    add(0x75AB, &mut chip);

    assert_eq!(chip.v[0x5], 0xAC);
    assert_pc_increment(&mut chip);
}

#[test]
fn add_overflow_test() {
    let mut chip = Chip::new();

    chip.v[0x5] = 0xFF;

    add(0x7502, &mut chip);

    assert_eq!(chip.v[0x5], 0x1);
    assert_pc_increment(&mut chip);
}

#[test]
fn ldr_test() {
    let mut chip = Chip::new();
    chip.v[1] = 111;
    chip.v[2] = 222;

    ldr(0x8120, &mut chip);

    assert_eq!(chip.v[1], 222);
    assert_pc_increment(&chip);
}

#[test]
fn or_test() {
    let mut chip = Chip::new();
    chip.v[1] = 0x12;
    chip.v[2] = 0x34;

    or(0x8121, &mut chip);
    assert_eq!(chip.v[1], 0x36);
    assert_eq!(chip.v[2], 0x34);

    assert_pc_increment(&chip);
}

#[test]
fn and_test() {
    let mut chip = Chip::new();

    chip.v[1] = 0x12;
    chip.v[2] = 0x34;

    and(0x8122, &mut chip);

    assert_eq!(chip.v[1], 0x10);
    assert_eq!(chip.v[2], 0x34);
    assert_pc_increment(&chip);
}

#[test]
fn xor_test() {
    let mut chip = Chip::new();

    chip.v[1] = 0x12;
    chip.v[2] = 0x34;

    xor(0x8123, &mut chip);

    assert_eq!(chip.v[1], 0x26);
    assert_eq!(chip.v[2], 0x34);
    assert_pc_increment(&chip);
}

#[test]
fn addreg_test() {
    let mut chip = Chip::new();

    chip.v[0] = 0x20;
    chip.v[1] = 0x01;
    chip.v[0xF] = 1;

    addreg(0x8014, &mut chip);

    assert_eq!(chip.v[0], 0x21);
    assert_eq!(chip.v[1], 0x01);
    assert_eq!(chip.v[0xF], 0);
    assert_pc_increment(&chip);
}

#[test]
fn addreg_carry_test() {
    let mut chip = Chip::new();

    chip.v[0] = 0xFF;
    chip.v[1] = 0x02;
    chip.v[0xF] = 0;

    addreg(0x8014, &mut chip);

    assert_eq!(chip.v[0], 0x01);
    assert_eq!(chip.v[1], 0x02);
    assert_eq!(chip.v[0xF], 1);
    assert_pc_increment(&chip);
}

#[test]
fn subreg_test() {
    let mut chip = Chip::new();

    chip.v[0] = 0x20;
    chip.v[1] = 0x01;
    chip.v[0xF] = 1;

    subreg(0x8015, &mut chip);

    assert_eq!(chip.v[0], 0x1F);
    assert_eq!(chip.v[1], 0x01);
    assert_eq!(chip.v[0xF], 0);
    assert_pc_increment(&chip);
}

#[test]
fn subreg_carry_test() {
    let mut chip = Chip::new();

    chip.v[0] = 0x00;
    chip.v[1] = 0x01;
    chip.v[0xF] = 0;

    subreg(0x8015, &mut chip);

    assert_eq!(chip.v[0], 0xFF);
    assert_eq!(chip.v[1], 0x01);
    assert_eq!(chip.v[0xF], 1);
    assert_pc_increment(&chip);
}
