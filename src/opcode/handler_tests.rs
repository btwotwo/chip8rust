use super::*;

fn prepare() -> (OpcodeHandler, Chip) {
    (OpcodeHandler::new(), Chip::new())
}

#[test]
fn next_increases_program_counter() {
    let (mut handler, mut chip) = prepare();

    chip.program_counter.set(0x2);
    handler.next(0x6512, &mut chip);

    assert_eq!(chip.program_counter, 0x4);
}

#[test]
fn ret_does_not_increase_pc() {
    let (mut handler, mut chip) = prepare();
    let sp = 5;
    let new_pc = 0x6;

    chip.stack_pointer = sp;
    chip.stack[sp as usize] = new_pc;
    chip.program_counter.set(0x2);
    handler.next(0x00EE, &mut chip);

    assert_eq!(chip.program_counter, new_pc)
}

#[test]
fn jp_does_not_increase_pc() {
    let (mut handler, mut chip) = prepare();

    chip.program_counter.set(2);

    handler.next(0x1034, &mut chip);

    assert_eq!(chip.program_counter, 0x0034);
}

#[test]
fn call_does_not_increase_pc() {
    let (mut handler, mut chip) = prepare();

    chip.program_counter.set(2);

    handler.next(0x2012, &mut chip);

    assert_eq!(chip.program_counter, 0x0012)
}

#[test]
fn jmpv0_does_not_increase_pc() {
    let (mut handler, mut chip) = prepare();

    chip.program_counter.set(2);

    handler.next(0xB123, &mut chip);

    assert_eq!(chip.program_counter, 0x0123);
}