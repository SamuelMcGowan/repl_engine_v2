use std::io::{self, Write};

use repl_engine2::{Repl, Signal};

fn main() -> io::Result<()> {
    let mut repl = Repl::new();

    loop {
        print!("\n╭ ~/username\n╰ ");
        std::io::stdout().flush()?;

        match repl.read_line().unwrap() {
            Signal::Submit(output) => {
                println!("\n{output:?}");
            }

            Signal::Clear => {
                repl.clear_screen()?;
            }

            Signal::Interrupted => {}
            Signal::Eof => {
                return Ok(());
            }
        }
    }
}
