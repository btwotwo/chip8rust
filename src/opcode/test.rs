use super::*;

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
