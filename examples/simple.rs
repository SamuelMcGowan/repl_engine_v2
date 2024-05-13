use std::io;

use repl_engine2::{Repl, Signal};

fn main() -> io::Result<()> {
    let mut repl = Repl::new();

    loop {
        match repl.read_line(">> ").unwrap() {
            Signal::Submit(output) => {
                println!("{output:?}");
            }

            Signal::Interrupted => {
                eprintln!("Ctrl-C: Interrupted");
            }
            Signal::EOF => {
                eprintln!("Ctrl-D: EOF");
                return Ok(());
            }
        }
    }
}
