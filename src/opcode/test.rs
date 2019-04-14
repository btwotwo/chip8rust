use super::*;

fn assert_pc_increment(chip: &mut Chip) {
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
    chip.v.set(4, 0x12);
    chip.v.set(5, 0x10);

    se(0x3412, &mut chip);

    assert_eq!(chip.program_counter, 16);

    se(0x3512, &mut chip);

    assert_eq!(chip.program_counter, 18);
}

#[test]
fn sne_test() {
    let mut chip = Chip::new();
    chip.program_counter.set(12);
    chip.v.set(4, 0x12);
    chip.v.set(5, 0x10);

    sne(0x4412, &mut chip);

    assert_eq!(chip.program_counter, 14);

    sne(0x4512, &mut chip);

    assert_eq!(chip.program_counter, 18);
}

#[test]
fn sre_test() {
    let mut chip = Chip::new();

    chip.program_counter.set(12);

    chip.v.set(4, 0x12);
    chip.v.set(5, 0x12);
    chip.v.set(6, 0x13);

    sre(0x5450, &mut chip);

    assert_eq!(chip.program_counter, 16);

    sre(0x5460, &mut chip);

    assert_eq!(chip.program_counter, 18)
}

#[test]
fn ld_test() {
    let mut chip = Chip::new();

    chip.v.set(0xE, 0xAB);

    ld(0x6EAB, &mut chip);

    assert_eq!(chip.v.get(0xE), 0xAB);
    assert_pc_increment(&mut chip);
}

#[test]
fn add_test() {
    let mut chip = Chip::new();

    chip.v.set(0x5, 0x1);

    add(0x75AB, &mut chip);

    assert_eq!(chip.v.get(0x5), 0xAC);
    assert_pc_increment(&mut chip);
}

#[test]
fn add_overflow_test() {
    let mut chip = Chip::new();

    chip.v.set(0x5, 0xFF);

    add(0x7502, &mut chip);

    assert_eq!(chip.v.get(0x5), 0x1);
    assert_pc_increment(&mut chip);
}
