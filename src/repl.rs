use std::io::{self, BufRead, Write};

use crate::lexer::Lexer;

const PROMPT: &str = ">> ";

pub fn start<R: BufRead, W: Write>(input: R, mut output: W) -> io::Result<()> {
    let mut lines = input.lines();
    loop {
        write!(output, "{PROMPT}")?;
        output.flush()?;

        let Some(line) = lines.next() else {
            return Ok(());
        };

        let line = line?;

        for token in Lexer::new(&line) {
            writeln!(output, "{token:?}")?;
        }
    }
}
