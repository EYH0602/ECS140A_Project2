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
            Some(t) => match t.get_type() {
                TokenType::INVALID => panic!("Invalid Token: TokenType::INVALID"),
                _ => {
                    self.token = t;
                    true
                }
            },
        }
    }

    fn program(&mut self) {
        if !self.update_token() {
            return;
        }

        // {Declaration}
        loop {
            if !self.declaration() {
                break;
            }
            println!("{}", self.result);
            println!("======================");
        }

        self.main_declaration();
        println!("{}", self.result);
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

    fn main_declaration(&mut self) -> bool {
        let mut text = self.token.get_text();
        if &*text != "void" {
            panic!("Invalid Main Declaration type (void)");
        }
        if !self.update_token() {
            return false;
        }
        text = self.token.get_text();
        if &*text != "main" {
            panic!("Invalid Main Declaration (main)");
        }
        self.result.push_str("\nvoid main()");
        self.block()
    }

    fn block(&mut self) -> bool {
        self.result.push_str(" {\n");
        // {Declaration}
        if !self.update_token() {
            return false;
        }
        while self.declaration() {}
        // {Statement}
        while self.statement() {}

        self.result.push_str("}\n");
        true
    }

    fn statement(&mut self) -> bool {
        match self.token.get_type() {
            TokenType::VARIABLE => return self.assignment(),
            TokenType::KEYWORD => match self.token.get_text() {
                "while" => return self.while_loop(),
                "if" => return self.if_statement(),
                "return" => return self.return_statement(),
                _ => return false,
            },
            _ => {}
        }
        false
    }

    fn assignment(&mut self) -> bool {
        self.identifier();
        self.result.push_str(" = ");
        if !self.update_token() {
            return false;
        }
        if &*self.token.get_text() != "=" {
            panic!("invalid assignment");
        }
        if !self.update_token() {
            return false;
        }
        println!(
            "!!! {} {}",
            self.token.get_text(),
            self.token.get_type().as_str()
        );
        loop {
            match self.token.get_type() {
                TokenType::VARIABLE => {
                    self.identifier();
                    self.result.push_str(" = ");
                    if !self.update_token() {
                        return false;
                    }
                }
                _ => break,
            }
        }
        self.expression()
    }

    fn expression(&mut self) -> bool {
        self.simple_expression()
    }

    fn simple_expression(&mut self) -> bool {
        self.term()
    }

    fn term(&mut self) -> bool {
        self.factor()
    }

    fn factor(&mut self) -> bool {
        match self.token.get_type() {
            TokenType::INTCONSTANT => self.constant(),
            TokenType::FLOATCONSTANT => self.constant(),
            TokenType::FUNCTION => {
                println!("--- {}", self.token.get_text());
                self.identifier();
                self.result.push('(');
                if !self.update_token() {
                    return false;
                }
                let mut res = self.expression();
                while res {
                    self.result.push_str(", ");
                    if !self.update_token() {
                        return false;
                    }
                    res = res && self.expression();
                }
                self.result.push(')');
                return res;
            }
            _ => false,
        }
    }

    fn while_loop(&mut self) -> bool {
        false
    }
    fn if_statement(&mut self) -> bool {
        false
    }
    fn return_statement(&mut self) -> bool {
        false
    }

    fn function_definition(&mut self) {}

    fn declaration_type(&mut self) -> bool {
        if !self.data_type() {
            return false;
        }
        if !self.update_token() {
            return false;
        }
        self.identifier();
        true
    }

    fn variable_declaration(&mut self) -> bool {
        if !self.update_token() {
            self.result.push_str(";\n");
            return false;
        }

        let text = self.token.get_text();
        if &*text == "=" {
            self.result.push_str(" = ");
            if !self.update_token() {
                return false;
            }
            let res = self.constant();
            if !self.update_token() {
                return false;
            }
            res
        } else {
            self.result.push_str(";\n");
            true
        }
    }

    fn function_declaration(&mut self) -> bool {
        if !self.update_token() {
            return false;
        }
        let res = self.parameter_block();
        self.result.push_str(";\n");
        if !self.update_token() {
            return false;
        }
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

    fn data_type(&mut self) -> bool {
        if !self.float_type() && !self.integer_type() {
            return false;
        }
        true
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
