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

    fn update_token(&mut self) -> bool {
        match self.scanner.get_next_token() {
            None => return false,
            Some(t) => {
                self.token = t;
                true
            }
        }
    }

    fn program(&mut self) {
        if !self.update_token() {
            return;
        }

        loop {
            let flag1 = self.declaration();
            println!("{}", self.result);
            let flag2 = flag1 && self.update_token();
            if !flag2 {
                break;
            }
        }
    }

    fn declaration(&mut self) -> bool {
        if !self.declaration_type() {
            return false;
        }
        match self.token.get_type() {
            TokenType::VARIABLE => return self.variable_declaration(),
            TokenType::FUNCTION => return self.function_declaration(),
            _ => panic!("Invalid declaration"),
        }
    }

    fn function_definition(&mut self) {}

    fn declaration_type(&mut self) -> bool {
        self.data_type();
        if !self.update_token() {
            return false;
        }
        self.identifier();
        true
    }

    fn variable_declaration(&mut self) -> bool {
        self.result.push_str(" = ");
        if !self.update_token() {
            return false;
        }
        let text = self.token.get_text();
        if &*text != "=" {
            panic!("invalid variable declaration");
        }
        if !self.update_token() {
            return false;
        }
        self.constant()
    }

    fn function_declaration(&mut self) -> bool {
        if !self.update_token() {
            return false;
        }
        let res = self.parameter_block();
        self.result.push_str(";\n");
        res
    }

    fn parameter_block(&mut self) -> bool {
        self.result.push('(');
        while self.parameter() {
            if !self.update_token() {
                return false;
            }
        }
        self.result.push(')');
        true
    }

    fn parameter(&mut self) -> bool {
        self.data_type();
        if !self.update_token() {
            return false;
        }
        self.identifier();
        false
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
        true
    }

    fn float_constant(&mut self) -> bool {
        self.result.push_str(self.token.get_text());
        self.result.push_str(";\n");
        true
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
            }
            TokenType::FUNCTION => {
                self.result.push_str(text);
            }
            _ => panic!("invalid identifier"),
        }
    }
}
