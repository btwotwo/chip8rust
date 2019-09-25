use super::display::Display;
use crossterm::{queue, Goto, Terminal, PrintStyledFont, Colorize, Crossterm};
use std::io::Write;

const pixel: &'static str = "█";

pub fn redraw(display: &Display, term: &mut Crossterm) {
    term.terminal().clear(crossterm::ClearType::All);
    let cursor = term.cursor();
    for (row_idx, row) in display.contents.iter().enumerate() {
        const MASK: u64 = 0x8000_0000_0000_0000;

        for i in 0..64 {
            let pxl = (MASK >> i) & row;
            if pxl != 0 {
                cursor.goto(i, row_idx as u16);
                println!("{}", pixel);
            }
        }
    }

    cursor.goto(77, 87);
    println!("{:?}", display.contents);
}