use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::stdout;

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?; // What '?' does here. It unwraps the Result of enable_raw_mode for us.
                            //If it's an error, it returns the error immediately. If not, it continues.
        Self::clear_screen()?;
        Self::move_cursor_to(0, 0)?;
        Ok(())
    }
    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), std::io::Error> {
        execute!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn move_cursor_to(col: u16, row: u16) -> Result<(), std::io::Error> {
        execute!(stdout(), MoveTo(col, row))?;
        Ok(())
    }

    pub fn size() -> Result<(u16, u16), std::io::Error> {
        size()
    }
}
