use crate::scanner::Scanner;
use crate::token::Token;
use crate::token::TokenType;

pub struct Parser {
    scanner: Scanner,
    token: Token,
    result: String,
}

impl Parser {
    pub fn new(f: &str) -> Parser {
        Parser {
            scanner: Scanner::new(f),
            token: Token::new(String::from(""), TokenType::NONE, 0, 0),
            result: String::from(""),
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

    pub fn get_result(&self) -> &str {
        &self.result
    }

    pub fn parse(&mut self) {
        self.program();
    }

    fn update_token(&mut self) {
        match self.scanner.get_next_token() {
            None => return,
            Some(t) => {
                self.token = t;
            }
        }
    }

    fn program(&mut self) {
        match self.scanner.get_next_token() {
            None => return,
            Some(t) => {
                self.token = t;
            }
        }

        while self.declaration() {}
    }

    fn declaration(&mut self) -> bool {
        self.declaration_type();
        match self.token.get_type() {
            TokenType::VARIABLE => return self.variable_declaration(),
            TokenType::FUNCTION => return self.function_declaration(),
            _ => panic!("Invalid declaration"),
        }
    }

    fn function_definition(&mut self) {}

    fn declaration_type(&mut self) {
        self.data_type();
        self.update_token();
        self.identifier();
    }

    fn variable_declaration(&mut self) -> bool {
        self.result.push_str("= ");
        self.update_token();
        let text = self.token.get_text();
        if &*text != "=" {
            panic!("invalid variable declaration");
        }
        self.update_token();
        self.constant()
    }

    fn function_declaration(&mut self) -> bool {
        true
    }

    fn constant(&mut self) -> bool {
        match self.token.get_type() {
            TokenType::INTCONSTANT => return self.int_constant(),
            TokenType::FLOATCONSTANT => return self.float_constant(),
            _ => panic!("invalid constant type"),
        }
    }

    fn int_constant(&mut self) -> bool {
        self.result.push_str(self.token.get_text());
        self.result.push_str(";\n");
        false
    }

    fn float_constant(&mut self) -> bool {
        self.result.push_str(self.token.get_text());
        self.result.push_str(";\n");
        false
    }

    fn data_type(&mut self) {
        if !self.float_type() && !self.integer_type() {
            panic!("invalid type");
        }
    }

    fn integer_type(&mut self) -> bool {
        let text = self.token.get_text();
        let int_types = vec!["char", "short", "int", "long"];
        if &*text == "unsigned" {
            self.result.push_str(text);
            self.result.push(' ');
            match self.scanner.get_next_token() {
                None => panic!("Syntax Error: incomplete type."),
                Some(token) => match token.get_type() {
                    TokenType::KEYWORD => {
                        let new_text = token.get_text();
                        if int_types.contains(&new_text) {
                            self.result.push_str(text);
                            self.result.push(' ');
                        } else {
                            return false;
                        }
                    }
                    _ => panic!("aaa!"),
                },
            }
        } else if int_types.contains(&text) {
            self.result.push_str(text);
            self.result.push(' ');
        } else {
            return false;
        }
        true
    }

    fn float_type(&mut self) -> bool {
        let text = self.token.get_text();
        if &*text == "float" || &*text == "double" {
            self.result.push_str(text);
            self.result.push(' ');
            return true;
        }
        false
    }

    fn identifier(&mut self) {
        let text = self.token.get_text();
        match self.token.get_type() {
            TokenType::VARIABLE => {
                self.result.push_str(text);
                self.result.push(' ');
            }
            TokenType::FUNCTION => {
                self.result.push_str(text);
                self.result.push(' ');
            }
            _ => panic!("invalid identifier"),
        }
    }
}
