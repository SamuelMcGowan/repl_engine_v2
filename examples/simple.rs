use std::io;

use repl_engine2::{PaintBuffer, Vec2};

fn main() -> io::Result<()> {
    let mut paintbuf = PaintBuffer::new()?;

    crossterm::terminal::enable_raw_mode()?;
    paintbuf.paint(">> ", "hello\nworld", Vec2::new(0, 1))?;
    crossterm::terminal::disable_raw_mode()?;

    loop {}

    Ok(())
}
