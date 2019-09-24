use super::display::Display;
use crossterm::{queue, Goto, Terminal, PrintStyledFont, Colorize};
use std::io::Write;

const pixel: &'static str = "█";

pub fn redraw(display: &Display, term: &mut Terminal) {
    term.clear(crossterm::ClearType::All);
    for (row_idx, row) in display.contents.iter().enumerate() {
        const MASK: u64 = 0x8000_0000_0000_0000;

        for i in 0..63 {
            let pxl = (MASK >> i) & row;
            if pxl != 0 {
                queue!(
                    std::io::stdout(),
                    Goto(i, row_idx as u16),
                    PrintStyledFont(pixel.green())
                );
            }
        }
    }
}