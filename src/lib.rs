mod editor;
mod event;
mod paint;
mod string_info;
mod vec2;

use std::io;

use crossterm::{cursor, terminal};
use editor::Editor;
pub use event::Signal;
pub use paint::PaintBuffer;
use vec2::Vec2;

#[derive(Default)]
pub struct Repl {
    editor: Editor,
}

impl Repl {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read_line(&mut self) -> io::Result<Signal> {
        terminal::enable_raw_mode()?;
        let res = self.read_line_inner();
        terminal::disable_raw_mode()?;
        res
    }

    fn read_line_inner(&mut self) -> io::Result<Signal> {
        use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

        let mut paint_buffer = PaintBuffer::new()?;

        macro_rules! repaint {
            () => {
                paint_buffer.paint(self.editor.as_str(), self.editor.cursor_pos())?;
            };
        }

        repaint!();

        loop {
            match event::read()? {
                Event::Key(KeyEvent {
                    code,
                    modifiers,
                    kind: KeyEventKind::Press | KeyEventKind::Repeat,
                    state: _,
                }) => match (modifiers, code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                        self.editor.clear();
                        return Ok(Signal::Interrupted);
                    }
                    (KeyModifiers::CONTROL, KeyCode::Char('d')) => {
                        return Ok(Signal::EOF);
                    }

                    (KeyModifiers::CONTROL, KeyCode::Char('l')) => {
                        return Ok(Signal::Clear);
                    }

                    (KeyModifiers::NONE, KeyCode::Backspace) => {
                        self.editor.backspace_char();
                    }

                    (KeyModifiers::NONE, KeyCode::Delete) => {
                        self.editor.delete_char();
                    }

                    (KeyModifiers::CONTROL, KeyCode::Char('w')) => {
                        self.editor.backspace_word();
                    }

                    (KeyModifiers::ALT, KeyCode::Char('d')) => {
                        self.editor.delete_word();
                    }

                    (KeyModifiers::NONE, KeyCode::Left) => {
                        self.editor.move_left();
                    }
                    (KeyModifiers::NONE, KeyCode::Right) => {
                        self.editor.move_right();
                    }
                    (KeyModifiers::NONE, KeyCode::Up) => {
                        self.editor.move_up();
                    }
                    (KeyModifiers::NONE, KeyCode::Down) => {
                        self.editor.move_down();
                    }
                    (KeyModifiers::CONTROL, KeyCode::Left) => {
                        self.editor.move_left_word();
                    }
                    (KeyModifiers::CONTROL, KeyCode::Right) => {
                        self.editor.move_right_word();
                    }
                    (KeyModifiers::NONE, KeyCode::Home) => {
                        self.editor.move_home();
                    }
                    (KeyModifiers::NONE, KeyCode::End) => {
                        self.editor.move_end();
                    }

                    (KeyModifiers::ALT, KeyCode::Enter) => {
                        self.editor.insert_char('\n');
                    }
                    (KeyModifiers::NONE, KeyCode::Enter) => {
                        return Ok(Signal::Submit(self.editor.take()));
                    }

                    (KeyModifiers::NONE, KeyCode::Char(ch)) => {
                        self.editor.insert_char(ch);
                    }

                    _ => {}
                },

                Event::Paste(s) => {
                    self.editor.insert_str(&s);
                }

                Event::Resize(width, height) => {
                    paint_buffer.set_size(width, height)?;
                }

                _ => {}
            }

            repaint!();
        }
    }

    pub fn clear_screen(&mut self) -> io::Result<()> {
        crossterm::execute!(
            std::io::stdout(),
            terminal::Clear(crossterm::terminal::ClearType::All),
            cursor::MoveTo(0, 0),
        )
    }
}
