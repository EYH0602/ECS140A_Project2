use crate::scanner::Scanner;
use crate::token::Token;
use crate::token::TokenType;

pub struct Parser {
    scanner: Scanner,
    token: Token,
}

impl Parser {
    pub fn new(f: &str) -> Parser {
        Parser {
            scanner: Scanner::new(f),
            token: Token::new(String::from(""), TokenType::NONE, 0, 0),
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

    fn program(&mut self) {
        match self.scanner.get_next_token() {
            None => {}
            Some(t) => {
                self.token = t;
            }
        }
    }
}
