#[path = "./token.rs"]
mod token;
use token::*;

#[path = "./scanner.rs"]
mod scanner;
use scanner::*;

pub struct Parser {
    scanner: Scanner,
}

impl Parser {
    pub fn new(f: &str) -> Parser {
        Parser {
            scanner: Scanner::new(f),
        }
    }

    pub fn print_lex_results(&mut self) {
        loop {
            match self.scanner.get_next_token() {
                Some(token) => {
                    println!("text: {}", token.get_text());
                    println!("token type: {}", token.get_type().as_str());
                    println!("line number: {}", token.get_line_number());
                    println!("char position: {}", token.get_char_pos());
                    println!("=======================================");
                }
                None => break,
            }
        }
    }

    pub fn parse(&self) {
        println!("parsing!");
    }
}
