use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
mod terminal;
mod view;
use std::io::Error;
use std::panic::{set_hook, take_hook};
use std::{cmp::min, env};
use terminal::{Position, Size, Terminal};
use view::View;

#[derive(Clone, Copy, Default)]
pub struct Location {
    x: usize,
    y: usize,
}

pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut view = View::default();
        let args: Vec<String> = env::args().collect();

        if let Some(file_name) = args.get(1) {
            view.load(file_name);
        }

        Ok(Self {
            should_quit: false,
            location: Location::default(),
            view,
        })

        // let result = self.repl();
        // Terminal::terminate().unwrap();
        // result.unwrap();
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();

            if self.should_quit {
                break;
            }
            // let event = read()?;
            // self.evaluate_event(event)?;
            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
            }
        }
    }

    // needless_pass_by_value: Event is not huge, so there is not a
    // performance overhead in passing by value, and pattern matching in this
    // function would be needlessly complicated if we pass by reference here.
    #[allow(clippy::needless_pass_by_value)]
    fn evaluate_event(&mut self, event: Event) {
        match event {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                }
                (
                    KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::PageDown
                    | KeyCode::PageUp
                    | KeyCode::End
                    | KeyCode::Home,
                    _,
                ) => {
                    let _ = self.move_point(code);
                }
                _ => {}
            },
            Event::Resize(w_16, h_16) => {
                #[allow(clippy::as_conversions)]
                let h = h_16 as usize;
                #[allow(clippy::as_conversions)]
                let w = w_16 as usize;
                self.view.resize(Size { w, h });
            }
            _ => {}
        }
    }

    fn move_point(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.location;
        let Size { w, h } = Terminal::size().unwrap_or_default();

        match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = min(h.saturating_sub(1), y.saturating_add(1));
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(w.saturating_sub(1), x.saturating_add(1));
            }
            KeyCode::PageDown => {
                y = h.saturating_sub(1);
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = w.saturating_sub(1);
            }
            _ => (),
        }

        self.location = Location { x, y };

        Ok(())
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();
        let _ = Terminal::move_caret_to(Position {
            col: self.location.x,
            row: self.location.y,
        });

        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::crossterm_print("GoodBye. \r\n");
        }
    }
}
