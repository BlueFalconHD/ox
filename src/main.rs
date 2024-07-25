// main.rs

mod scanner;

use scanner::{Scanner, Token};
use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        return Ok(());
    }

    let filename = &args[1];
    let mut file = File::open(filename)?;
    let mut source = String::new();
    file.read_to_string(&mut source)?;

    let mut scanner = Scanner::new(source);
    match scanner.scan_tokens() {
        Ok(tokens) => {
            for token in tokens {
                println!("{}", token.to_string());
            }
        }
        Err(e) => eprintln!("Error scanning tokens: {}", e),
    }

    Ok(())
}
