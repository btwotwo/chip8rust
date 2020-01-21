use super::display::Display;
use crossterm::{
    execute,
    queue
};
use std::io::{Write, stdout};


const pixel: &'static str = "█";

pub fn redraw(display: &Display) -> crossterm::Result<()> {
    let mut stdout = stdout();

    execute!(
        stdout,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )?;

    for (row_idx, row) in display.contents.iter().enumerate() {
        const MASK: u64 = 0x8000_0000_0000_0000;

        for i in 0..64 {
            let pxl = (MASK >> i) & row;
            if pxl != 0 {
                queue!(
                    stdout,
                    crossterm::cursor::MoveTo(i, row_idx as u16),
                    crossterm::style::Print(pixel)
                )?;
            }
        }
    }

    stdout.flush()?;

    Ok(())
}

pub fn init() -> crossterm::Result<()> {

    use crossterm::terminal::{EnterAlternateScreen};

    let mut stdout = stdout();

    execute!(
        stdout,
        crossterm::cursor::Hide,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        crossterm::terminal::SetSize(64,32),
    )

    // match term.terminal().terminal_size() {
    //     size if (size.0 >= 64 && size.1 >= 32) => Ok((term, alternate)),
    //     _ => Err(crossterm::ErrorKind::ResizingTerminalFailure(
    //         "Could not set expected size! Please set terminal to 64x32".to_string(),
    //     )),
    // }
}
