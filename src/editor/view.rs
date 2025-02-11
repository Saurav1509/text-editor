use super::terminal::{Size, Terminal};
mod buffer;
use buffer::Buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
}

impl View {
    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }

    fn render_line(at: usize, line_text: &str) {
        let result = Terminal::print_row(at, line_text);
        debug_assert!(result.is_ok(), "Failed to render line");
    }

    fn build_welcome_message(w: usize) -> String {
        if w == 0 {
            return " ".to_string();
        }

        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();

        if w <= len {
            return "~".to_string();
        }

        #[allow(clippy::integer_division)]
        let padding = (w.saturating_sub(len).saturating_sub(1)) / 2;
        let mut full_message = format!("~{}{}", " ".repeat(padding), welcome_message);
        full_message.truncate(w);
        full_message
    }

    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }

        let Size { w, h } = self.size;

        if w == 0 || h == 0 {
            return;
        }

        #[allow(clippy::integer_division)]
        let vertical_center = h / 3;

        for current_row in 0..h {
            if let Some(line) = self.buffer.lines.get(current_row) {
                let truncated_line = if line.len() >= w { &line[0..w] } else { line };
                Self::render_line(current_row, truncated_line);
            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(w));
            } else {
                Self::render_line(current_row, "~");
            }
        }
        self.needs_redraw = false;
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}
