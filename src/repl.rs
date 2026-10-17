use std::io::{self, BufRead, Write};

use crate::lexer::Lexer;

const PROMPT: &str = ">> ";

/// Runs the REPL: reads lines from `input`, lexes each one, and writes
/// the tokens to `output`, until `input` runs out.
///
/// # Errors
///
/// Returns an error if reading from `input` or writing to `output` fails.
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
