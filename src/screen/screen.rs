use super::display::Display;
use crossterm::{Crossterm, RawScreen};

const pixel: &'static str = "█";

pub fn redraw(display: &Display, term: &mut Crossterm) -> crossterm::Result<()> {
    term.terminal().clear(crossterm::ClearType::All)?;
    let cursor = term.cursor();
    for (row_idx, row) in display.contents.iter().enumerate() {
        const MASK: u64 = 0x8000_0000_0000_0000;

        for i in 0..64 {
            let pxl = (MASK >> i) & row;
            if pxl != 0 {
                cursor.goto(i, row_idx as u16)?;
                println!("{}", pixel);
            }
        }
    };
    
    Ok(())
}

pub fn init() -> crossterm::Result<Crossterm> {
    let term = Crossterm::new();
    RawScreen::into_raw_mode()?.disable_drop();
    term.cursor().hide()?;
    term.terminal().set_size(32, 64)?;

    match term.terminal().terminal_size() {
        size if (size.0 >= 64 && size.1 >= 32) => Ok(term),
        _ => Err(crossterm::ErrorKind::ResizingTerminalFailure("Could not set expected size! Please set terminal to 64x32".to_string()))
    }
}