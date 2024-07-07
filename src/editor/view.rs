use super::terminal::{Size, Terminal};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default, Clone, Copy)]
pub struct View {}

impl View {
    pub fn render() -> Result<(), Error> {
        let Size { h, .. } = Terminal::size()?;
        Terminal::clear_line()?;
        Terminal::crossterm_print("Hello World!")?;
        for current_row in 0..h {
            #[allow(clippy::integer_division)]
            if current_row == h / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }

            if current_row.saturating_add(1) < h {
                Terminal::crossterm_print("\r\n")?;
            }
        }

        Ok(())
    }
    fn draw_empty_row() -> Result<(), Error> {
        Terminal::crossterm_print("~")?;
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");

        let width = Terminal::size()?.w;
        let len = welcome_message.len();

        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);

        Terminal::crossterm_print(&welcome_message)?;

        Ok(())
    }
}
