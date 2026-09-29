use std::env;
use std::io;

use monkey::repl;

fn main() -> io::Result<()> {
    let user = env::var("USER").unwrap();
    println!("Hello {user}! This is the Monkey programming language!");
    println!("Feel free to type in commands");
    repl::start(io::stdin().lock(), io::stdout())
}
