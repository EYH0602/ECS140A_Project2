use crate::scanner::Scanner;
use crate::token::Token;
use crate::token::TokenType;

#[derive(Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    idx: usize,
    result: String,
}

impl Parser {
    pub fn new(f: &str) -> Parser {
        let mut token_acc = Vec::new();
        let mut scanner = Scanner::new(f);

        // read in all tokens from scanner
        loop {
            match scanner.get_next_token() {
                Some(token) => token_acc.push(token),
                None => break,
            }
        }

        Parser {
            tokens: token_acc,
            idx: 0,
            result: String::from(""),
        }
    }

    pub fn print_lex_results(&self) {
        for token in &self.tokens {
            println!("text: {}", token.get_text());
            println!("token type: {}", token.get_type().as_str());
            println!("line number: {}", token.get_line_number());
            println!("char position: {}", token.get_char_pos());
            println!("=======================================");
        }
    }

    pub fn get_result(&self) -> &str {
        &self.result
    }

    pub fn parse(&mut self) {
        self.program();
    }

    fn panic_with_error(&self, msg: &str) {
        panic!(
            "{}\n  line number: {}\n  char pos: {}",
            msg,
            self.tokens[self.idx].get_line_number(),
            self.tokens[self.idx].get_char_pos()
        );
    }

    fn is_line_changed(&self) -> bool {
        self.tokens[self.idx].get_line_number() != self.tokens[self.idx - 1].get_line_number()
    }

    fn show(&self) {
        println!(
            "!!! {} {} \n{}",
            self.tokens[self.idx].get_text(),
            self.tokens[self.idx].get_type().as_str(),
            self.result
        );
    }

    fn program(&mut self) {
        // {Declaration}
        while self.idx < self.tokens.len() && self.tokens[self.idx].get_text() != "void" {
            self.declaration();
            self.idx += 1;
        }

        self.main_declaration();

        // {Function Definition}
        while self.idx < self.tokens.len() {
            self.function_definition();
            self.idx += 1;
        }
    }

    fn declaration(&mut self) {
        self.declaration_type();
        self.idx += 1;
        match self.tokens[self.idx - 1].get_type() {
            TokenType::VARIABLE => self.variable_declaration(),
            TokenType::FUNCTION => self.function_declaration(),
            _ => self.panic_with_error("Invalid declaration"),
        }
        self.result.push_str(";\n");
    }

    fn main_declaration(&mut self) {
        // skip two key words: void main
        if self.tokens[self.idx].get_text() == "void" {
            self.result.push_str("void ");
            self.idx += 1;
        } else {
            self.panic_with_error("invalid main declaration: void");
        }
        if self.tokens[self.idx].get_text() == "main" {
            self.result.push_str("main() ");
            self.idx += 1;
        } else {
            self.panic_with_error("invalid main declaration: main");
        }

        self.block();
    }

    fn function_definition(&mut self) {}

    fn declaration_type(&mut self) {
        self.data_type();
        self.idx += 1;
        // Identifier
        self.result.push_str(self.tokens[self.idx].get_text());
    }

    fn variable_declaration(&mut self) {
        if self.idx >= self.tokens.len() || self.tokens[self.idx].get_text() != "=" {
            self.idx -= 1;
            return;
        }
        self.result.push_str(" = ");
        self.idx += 1;
        self.constant();
    }

    fn function_declaration(&mut self) {
        self.parameter_block();
    }

    fn block(&mut self) {
        self.result.push_str("{\n");

        // {Declaration}
        while self.idx < self.tokens.len() && self.tokens[self.idx].is_type() {
            self.declaration();
            self.idx += 1;
        }
        // if !self.tokens[self.idx].is_type() {
        //     self.idx -= 1;
        // }

        // {Statement}
        let statement_keywords = vec!["while", "if", "return"];
        while self.idx < self.tokens.len() {
            let token: Token = self.tokens[self.idx].clone();
            if token.get_type() != &TokenType::VARIABLE
                && statement_keywords.contains(&token.get_text())
            {
                break;
            }
            self.statement();
            self.idx += 1;
        }

        // {Function Definition}

        self.result.push_str("\n}\n");
    }

    fn parameter_block(&mut self) {
        self.result.push('(');

        // [Parameter {, Parameter}]
        if self.tokens[self.idx].is_type() {
            self.parameter();
            self.idx += 1;
        }

        while self.idx < self.tokens.len()
            && self.tokens[self.idx].is_type()
            && !self.is_line_changed()
        {
            self.result.push_str(", ");
            self.parameter();
            self.idx += 1;
        }
        // if break out of loop by line change, an extra 1 is added
        if self.is_line_changed() {
            self.idx -= 1;
        }

        self.result.push(')');
    }

    fn data_type(&mut self) {
        let token: Token = self.tokens[self.idx].clone();
        let int_types = vec!["char", "short", "int", "long"];
        let float_types = vec!["float", "double"];
        if &*token.get_text() == "unsigned" {
            self.result.push_str("unsigned ");
            self.idx += 1;
            self.data_type();
        } else if int_types.contains(&token.get_text()) {
            self.integer_type();
        } else if float_types.contains(&token.get_text()) {
            self.float_type();
        } else {
            self.panic_with_error("invalid data type");
        }
    }

    fn constant(&mut self) {
        self.result.push_str(self.tokens[self.idx].get_text());
    }

    fn statement(&mut self) {
        let token = self.tokens[self.idx].clone();
        if token.get_type() == &TokenType::VARIABLE {
            self.assignment();
        } else {
            match token.get_text() {
                "while" => self.while_loop(),
                "if" => self.if_statement(),
                "return" => self.return_statement(),
                _ => {}
            }
        }
        self.result.push_str(";\n");
    }

    fn parameter(&mut self) {
        self.declaration_type();
    }

    fn integer_type(&mut self) {
        self.result.push_str(self.tokens[self.idx].get_text());
        self.result.push(' ');
    }

    fn float_type(&mut self) {
        self.result.push_str(self.tokens[self.idx].get_text());
        self.result.push(' ');
    }

    fn assignment(&mut self) {
        self.result.push_str(self.tokens[self.idx].get_text());
        self.idx += 1;
        if self.tokens[self.idx].get_text() != "=" {
            self.panic_with_error("invalid assignment");
        }
        self.result.push_str(" = ");
        self.idx += 1;

        // {Identifier =}
        while self.idx < self.tokens.len()
            && self.tokens[self.idx].get_type() == &TokenType::VARIABLE
        {
            self.result.push_str(self.tokens[self.idx].get_text());
            self.idx += 1;
            if self.tokens[self.idx].get_text() != "=" {
                break;
            }
            self.result.push_str(" = ");
            self.idx += 1;
        }

        self.expression();
    }

    fn while_loop(&mut self) {}
    fn if_statement(&mut self) {}
    fn return_statement(&mut self) {}

    fn expression(&mut self) {
        self.simple_expression();
    }

    fn simple_expression(&mut self) {
        self.term();
    }

    fn term(&mut self) {
        self.factor();
    }

    fn factor(&mut self) {
        let token: Token = self.tokens[self.idx].clone();
        if token.get_type() == &TokenType::INTCONSTANT
            || token.get_type() == &TokenType::FLOATCONSTANT
        {
            self.constant();
        }
        // self.show();
    }

    fn relational_operator(&mut self) {}
    fn add_operator(&mut self) {}
    fn mult_operator(&mut self) {}
    fn identifier(&mut self) {}
}
