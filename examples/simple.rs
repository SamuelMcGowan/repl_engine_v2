use std::io;

use repl_engine2::{Repl, Signal};

fn main() -> io::Result<()> {
    let mut repl = Repl::new();

    loop {
        match repl.read_line("\n|\n|-> ").unwrap() {
            Signal::Submit(output) => {
                println!("{output:?}");
            }

            Signal::Interrupted => {}
            Signal::EOF => {
                return Ok(());
            }
        }
    }
}
