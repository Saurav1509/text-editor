use crossterm::cursor::MoveTo;
use crossterm::cursor::{Hide, Show};
use crossterm::style::Print;
// use crossterm::execute;
use core::fmt::Display;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{queue, Command};
use std::io::Error;
use std::io::{stdout, Write};

#[derive(Copy, Clone)]
pub struct Size {
    pub w: u16,
    pub h: u16,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Terminal;

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?; // What '?' does here. It unwraps the Result of enable_raw_mode for us.
                            //If it's an error, it returns the error immediately. If not, it continues.
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }
    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn move_cursor_to(pos: Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(pos.x, pos.y))?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (w, h) = size()?;
        Ok(Size { w, h })
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn crossterm_print<T: Display>(s: T) -> Result<(), Error> {
        Self::queue_command(Print(s))?;
        Self::execute()?;
        Ok(())
    }

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
}
