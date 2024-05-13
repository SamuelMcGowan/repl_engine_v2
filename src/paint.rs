use std::fmt;
use std::io::{self, Stdout, Write};

use crossterm::style::Stylize;
use crossterm::{cursor, execute, queue, style, terminal};

use crate::vec2::Vec2;

pub struct PaintBuffer {
    stdout: Stdout,

    term_size: Vec2,

    buffer_start: u16,
    buffer_reserved: u16,

    cursor_line: u16,

    indent: u16,
}

impl PaintBuffer {
    pub fn new() -> io::Result<Self> {
        let mut paint_buffer = Self {
            stdout: io::stdout(),

            term_size: Vec2::new(0, 0),

            buffer_start: 0,
            buffer_reserved: 0,
            cursor_line: 0,

            indent: cursor::position()?.0,
        };

        let (width, height) = terminal::size()?;
        paint_buffer.set_size(width, height)?;

        Ok(paint_buffer)
    }

    pub fn set_size(&mut self, width: u16, height: u16) -> io::Result<()> {
        let width = if width == 0 { 1 } else { width };
        let height = if height == 0 { 1 } else { height };

        self.term_size = Vec2::new(width, height);

        let cursor_pos = cursor::position()?.1.min(height - 1);
        self.buffer_start = cursor_pos.saturating_sub(self.cursor_line);

        Ok(())
    }

    /// Expects the terminal to be in raw mode.
    pub fn paint(&mut self, input: &str, cursor: Vec2, note: Option<&str>) -> io::Result<()> {
        let total_lines = 1 + input.chars().filter(|&ch| ch == '\n').count();

        self.reserve_lines(total_lines as u16)?;

        // setup
        queue!(
            self.stdout,
            cursor::MoveTo(self.indent, self.buffer_start),
            terminal::Clear(terminal::ClearType::FromCursorDown)
        )?;

        // paint
        queue!(
            self.stdout,
            style::Print(Displayer {
                s: input,
                indent: self.indent as usize
            }),
        )?;

        self.cursor_line = cursor.y;

        let cursor_column = self.indent + cursor.x;
        let cursor_line = self.buffer_start + cursor.y;

        if let Some(note) = note {
            write!(
                self.stdout,
                "{}{}",
                if input.is_empty() { "" } else { " " },
                note.black().on_white()
            )?;
        }

        queue!(self.stdout, cursor::MoveTo(cursor_column, cursor_line))?;

        self.stdout.flush()?;

        Ok(())
    }

    /// NOTE: Moves the cursor.
    fn reserve_lines(&mut self, lines: u16) -> io::Result<()> {
        let new_lines = lines.saturating_sub(self.lines_available());

        self.add_lines(new_lines)?;

        self.buffer_start = self.buffer_start.saturating_sub(new_lines);

        self.buffer_reserved = lines;

        Ok(())
    }

    fn add_lines(&mut self, lines: u16) -> io::Result<()> {
        // Go to end of screen before adding new lines.
        queue!(self.stdout, cursor::MoveTo(0, self.term_size.y - 1))?;

        for _ in 0..lines {
            queue!(self.stdout, style::Print("\r\n"))?;
        }

        Ok(())
    }

    fn lines_available(&self) -> u16 {
        self.term_size.y - self.buffer_start
    }

    fn finalize(&mut self) -> io::Result<()> {
        let last_line = (self.buffer_start + self.buffer_reserved).saturating_sub(1);
        execute!(
            self.stdout,
            cursor::MoveTo(0, last_line),
            style::Print("\r\n"),
        )
    }
}

impl Drop for PaintBuffer {
    fn drop(&mut self) {
        self.finalize().expect("failed to finalize buffer");
    }
}

struct Displayer<'a> {
    s: &'a str,
    indent: usize,
}

impl fmt::Display for Displayer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for ch in self.s.chars() {
            match ch {
                '\r' => {}
                '\n' => {
                    write!(f, "\r\n")?;

                    for _ in 0..self.indent {
                        write!(f, " ")?;
                    }
                }
                ch => write!(f, "{ch}")?,
            }
        }

        Ok(())
    }
}
