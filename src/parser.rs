use crate::prettifier::Prettifier;
use crate::scanner::Scanner;
use crate::token::Token;
use crate::token::TokenType;

#[derive(Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    idx: usize,
    result: String,
    prettifier: Prettifier,
}

impl Parser {
    /// Create a new Parser for X-lang
    /// 
    /// # Arguments
    /// 
    /// * `f` - path to the source file
    /// * `format_f` - path to the config csv file, `format.csv` in this project
    /// 
    pub fn new(f: &str, format_f: &str) -> Parser {
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
            prettifier: Prettifier::new(format_f),
        }
    }

    /// print all all the tokens after lexer
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

    /// start the recursive descent parser based on EBNF rules
    pub fn parse(&mut self) {
        self.program();
    }

    pub fn to_xhtml(&self) -> String {
        let settings = String::from("<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">\n");
        let head = format!("<head>\n<title>{}</title>\n</head>", "X Formatted File");
        let xhtml = format!(
            "{setting}\n{html_open}\n{head}\n{body_open}\n{font_open}\n{body}\n{font_close}\n{body_close}\n{html_close}",
            setting = settings,
            head = head,
            body_open = self.prettifier.get_body_open(),
            body_close = self.prettifier.get_body_close(),
            body = self.result,
            font_open = self.prettifier.get_font_open(),
            font_close = self.prettifier.get_font_close(),
            html_open="<html xmlns=\"http://www.w3.org/1999/xhtml\" xml:lang=\"en\">",
            html_close="</html>"
        );
        xhtml
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

    fn indent(&mut self, len: i32) {
        for _ in 0..len {
            self.result.push_str("&nbsp;&nbsp;");
        }
    }

    fn program(&mut self) {
        // {Declaration}
        while self.idx < self.tokens.len() && self.tokens[self.idx].get_text() != "void" {
            self.declaration(0);
            self.idx += 1;
        }

        if self.idx >= self.tokens.len() {
            panic!("Missing Main Declaration!");
        }
        self.main_declaration();

        // {Function Definition}
        while self.idx < self.tokens.len() {
            self.function_definition();
            self.idx += 1;
        }
    }

    fn declaration(&mut self, indent_len: i32) {
        self.declaration_type(indent_len);
        self.idx += 1;
        match self.tokens[self.idx - 1].get_type() {
            TokenType::VARIABLE => self.variable_declaration(),
            TokenType::FUNCTION => self.function_declaration(),
            _ => self.panic_with_error("Invalid declaration"),
        }
        self.result
            .push_str("<font color=\"white\"><b>;</b></font><br />");
        // self.result.push_str(";\n");
    }

    fn main_declaration(&mut self) {
        // skip two key words: void main
        if self.tokens[self.idx].get_text() == "void" {
            // self.result.push_str(self.tokens[self.idx].get_text());
            self.result
                .push_str(&self.prettifier.prettify(self.tokens[self.idx].clone()));
            self.result.push(' ');
            self.idx += 1;
        } else {
            self.panic_with_error("invalid main declaration: void");
        }
        if self.tokens[self.idx].get_text() == "main" {
            // self.result.push_str("main() ");
            self.result
                .push_str(&self.prettifier.prettify(self.tokens[self.idx].clone()));
            self.result.push_str(
                "<font color=\"white\"><b>(</b></font><font color=\"white\"><b>)</b></font> ",
            );
            self.idx += 1;
        } else {
            self.panic_with_error("invalid main declaration: main");
        }

        self.block(0);
    }

    fn function_definition(&mut self) {
        self.declaration_type(0);
        self.idx += 1;
        self.parameter_block();
        self.idx += 1;
        self.block(0);
    }

    fn declaration_type(&mut self, indent_len: i32) {
        self.data_type(indent_len);
        self.idx += 1;
        // Identifier
        self.indent(0);
        // self.result.push_str(self.tokens[self.idx].get_text());
        self.result
            .push_str(&self.prettifier.prettify(self.tokens[self.idx].clone()));
    }

    fn variable_declaration(&mut self) {
        if self.idx >= self.tokens.len() || self.tokens[self.idx].get_text() != "=" {
            self.idx -= 1;
            return;
        }
        self.result.push_str(" ");
        self.result
            .push_str(&self.prettifier.prettify(self.tokens[self.idx].clone()));
        self.result.push_str(" ");
        self.idx += 1;
        self.constant();
    }

    fn function_declaration(&mut self) {
        self.parameter_block();
    }

    fn block(&mut self, indent_len: i32) {
        // self.indent(indent_len);
        // self.result.push_str("{\n");
        self.result
            .push_str("<font color=\"white\"><b>{</b></font><br />");

        // {Declaration}
        while self.idx < self.tokens.len() && self.tokens[self.idx].is_type() {
            self.declaration(indent_len + 1);
            self.idx += 1;
        }

        // {Statement}
        let statement_keywords = vec!["while", "if", "return"];
        while self.idx < self.tokens.len() {
            let token: Token = self.tokens[self.idx].clone();
            if token.get_type() != &TokenType::VARIABLE
                && !statement_keywords.contains(&token.get_text())
            {
                break;
            }
            self.statement(indent_len + 1);
            if self.tokens[self.idx - 1].get_type() != &TokenType::NONE {
                self.idx += 1;
            }
            if self.tokens[self.idx].get_type() == &TokenType::NONE {
                self.idx += 1;
                break;
            }
        }

        // {Function Definition}

        self.indent(indent_len);
        // self.result.push_str("}\n");
        self.result
            .push_str("<font color=\"white\"><b>}</b></font><br />");
    }

    fn parameter_block(&mut self) {
        // self.result.push('(');
        self.result
            .push_str("<font color=\"white\"><b>(</b></font>");

        // [Parameter {, Parameter}]
        if self.tokens[self.idx].is_type() {
            self.parameter();
            self.idx += 1;
        }

        while self.idx < self.tokens.len()
            && self.tokens[self.idx].is_type()
            && !self.is_line_changed()
        {
            // self.result.push_str(", ");
            self.result
                .push_str("<font color=\"white\"><b>,</b></font>");
            self.result.push(' ');
            self.parameter();
            self.idx += 1;
        }

        if self.idx >= self.tokens.len() {
            return;
        }

        // if break out of loop by line change, an extra 1 is added
        if self.is_line_changed() {
            self.idx -= 1;
        }

        // self.result.push(')');
        self.result
            .push_str("<font color=\"white\"><b>)</b></font>");
    }

    fn data_type(&mut self, indent_len: i32) {
        let token: Token = self.tokens[self.idx].clone();
        let int_types = vec!["char", "short", "int", "long"];
        let float_types = vec!["float", "double"];
        if &*token.get_text() == "unsigned" {
            self.indent(indent_len);
            // self.result.push_str("unsigned ");
            self.result
                .push_str(&self.prettifier.prettify(self.tokens[self.idx].clone()));
            self.result.push(' ');
            self.idx += 1;
            self.data_type(indent_len);
        } else if int_types.contains(&token.get_text()) {
            self.integer_type(indent_len);
        } else if float_types.contains(&token.get_text()) {
            self.float_type(indent_len);
        } else {
            self.panic_with_error("invalid data type");
        }
    }

    fn constant(&mut self) {
        // self.result.push_str(self.tokens[self.idx].get_text());
        match self.tokens[self.idx].get_type() {
            TokenType::OPERATOR => {
                if self.tokens[self.idx].get_text() == "-" {
                    self.result
                        .push_str(&self.prettifier.prettify(self.tokens[self.idx].clone()));
                    self.idx += 1;
                    self.constant();
                } else {
                    self.panic_with_error("unsupported unary operator");
                }
            }
            TokenType::INTCONSTANT => self
                .result
                .push_str(&self.prettifier.prettify(self.tokens[self.idx].clone())),
            TokenType::FLOATCONSTANT => self
                .result
                .push_str(&self.prettifier.prettify(self.tokens[self.idx].clone())),
            _ => self.panic_with_error("unsupported constant"),
        }
    }

    fn statement(&mut self, indent_len: i32) {
        let token = self.tokens[self.idx].clone();
        if token.get_type() == &TokenType::VARIABLE {
            self.assignment(indent_len);
            // self.result.push_str(";\n");
            self.result
                .push_str("<font color=\"white\"><b>;</b></font><br />");
        } else {
            match token.get_text() {
                "while" => self.while_loop(indent_len),
                "if" => self.if_statement(indent_len),
                "return" => {
                    self.return_statement(indent_len);
                    // self.result.push_str(";\n");
                    self.result
                        .push_str("<font color=\"white\"><b>;</b></font><br />");
                }
                _ => {}
            }
        }
    }

    fn parameter(&mut self) {
        self.declaration_type(0);
    }

    fn integer_type(&mut self, indent_len: i32) {
        self.indent(indent_len);
        // self.result.push_str(self.tokens[self.idx].get_text());
        self.result
            .push_str(&self.prettifier.prettify(self.tokens[self.idx].clone()));
        self.result.push(' ');
    }

    fn float_type(&mut self, indent_len: i32) {
        self.indent(indent_len);
        // self.result.push_str(self.tokens[self.idx].get_text());
        self.result
            .push_str(&self.prettifier.prettify(self.tokens[self.idx].clone()));
        self.result.push(' ');
    }

    fn assignment(&mut self, indent_len: i32) {
        self.indent(indent_len);
        // self.result.push_str(self.tokens[self.idx].get_text());
        self.result
            .push_str(&self.prettifier.prettify(self.tokens[self.idx].clone()));
        if self.tokens[self.idx + 1].get_text() != "=" {
            self.panic_with_error("invalid assignment");
        }
        self.idx += 1;
        // self.result.push_str(" = ");
        self.result.push(' ');
        self.result
            .push_str(&self.prettifier.prettify(self.tokens[self.idx].clone()));
        self.result.push(' ');
        self.idx += 1;

        // {Identifier =}
        while self.idx < self.tokens.len()
            && self.tokens[self.idx].get_type() == &TokenType::VARIABLE
            && self.tokens[self.idx + 1].get_text() == "="
        {
            self.result.push_str(self.tokens[self.idx].get_text());
            self.idx += 1;
            // self.result.push_str(" = ");
            self.result.push(' ');
            self.result
                .push_str(&self.prettifier.prettify(self.tokens[self.idx].clone()));
            self.result.push(' ');
            self.idx += 1;
        }

        self.expression();
    }

    fn while_loop(&mut self, indent_len: i32) {
        self.indent(indent_len);
        // self.result.push_str("while (");
        self.result
            .push_str(&self.prettifier.prettify(self.tokens[self.idx].clone()));
        self.result.push(' ');
        self.result
            .push_str("<font color=\"white\"><b>(</b></font>");
        self.idx += 1;
        self.expression();
        // self.result.push_str(") ");
        self.result
            .push_str("<font color=\"white\"><b>)</b></font>");
        self.idx += 1;
        if self.idx == self.tokens.len() {
            self.idx -= 1;
        }
        self.block(indent_len);
    }

    fn if_statement(&mut self, indent_len: i32) {
        self.indent(indent_len);
        // self.result.push_str("if (");
        self.result
            .push_str(&self.prettifier.prettify(self.tokens[self.idx].clone()));
        self.result.push(' ');
        self.result
            .push_str("<font color=\"white\"><b>(</b></font>");
        self.idx += 1;
        self.expression();
        // self.result.push_str(") ");
        self.result
            .push_str("<font color=\"white\"><b>)</b></font>");
        self.idx += 1;
        if self.idx == self.tokens.len() {
            self.idx -= 1;
        }
        self.block(indent_len);
    }

    fn return_statement(&mut self, indent_len: i32) {
        self.indent(indent_len);
        // self.result.push_str(self.tokens[self.idx].get_text());
        self.result
            .push_str(&self.prettifier.prettify(self.tokens[self.idx].clone()));
        self.result.push(' ');
        self.idx += 1;
        self.expression();
    }

    fn expression(&mut self) {
        self.simple_expression();
        // [ RelationalOperator SimpleExpression ]
        if self.idx + 1 < self.tokens.len() && self.tokens[self.idx + 1].is_relational_op() {
            self.idx += 1;
            self.relational_operator();
            self.idx += 1;
            self.simple_expression();
        }
    }

    fn simple_expression(&mut self) {
        self.term();

        // { AddOperator Term }
        while self.idx + 1 < self.tokens.len() && self.tokens[self.idx + 1].is_add_op() {
            self.idx += 1;
            self.add_operator();
            self.idx += 1;
            self.term();
        }
    }

    fn term(&mut self) {
        self.factor();
        // { MultOperator Factor }
        while self.idx + 1 < self.tokens.len() && self.tokens[self.idx + 1].is_mult_op() {
            self.idx += 1;
            self.mult_operator();
            self.idx += 1;
            self.factor();
        }
    }

    fn factor(&mut self) {
        let token: Token = self.tokens[self.idx].clone();
        match token.get_type() {
            TokenType::INTCONSTANT => self.constant(),
            TokenType::OPERATOR => self.constant(),
            TokenType::FLOATCONSTANT => self.constant(),
            TokenType::VARIABLE => {
                // self.result.push_str(token.get_text());
                self.result
                    .push_str(&self.prettifier.prettify(self.tokens[self.idx].clone()));
            }
            TokenType::FUNCTION => {
                // self.result.push_str(token.get_text());
                self.result
                    .push_str(&self.prettifier.prettify(self.tokens[self.idx].clone()));
                self.idx += 1;
                // self.result.push('(');
                self.result
                    .push_str("<font color=\"white\"><b>(</b></font>");
                self.expression();

                // {, Expression}
                let token: Token = self.tokens[self.idx + 1].clone();
                let parameter_types = vec![
                    TokenType::FLOATCONSTANT,
                    TokenType::INTCONSTANT,
                    TokenType::VARIABLE,
                ];
                if parameter_types.contains(token.get_type())
                    && self.tokens[self.idx].get_line_number() == token.get_line_number()
                {
                    self.idx += 1;
                    // self.result.push_str(", "); // !
                    self.result
                        .push_str("<font color=\"white\"><b>,</b></font>");
                    self.result.push(' ');
                    self.expression();
                }
                self.result.push(')');
            }
            _ => {
                // Todo: ( ( Expression ) )
            }
        }
    }

    fn relational_operator(&mut self) {
        if !self.tokens[self.idx].is_relational_op() {
            self.panic_with_error("Invalid relational operator");
        }
        self.result.push(' ');
        // self.result.push_str(self.tokens[self.idx].get_text());
        self.result
            .push_str(&self.prettifier.prettify(self.tokens[self.idx].clone()));
        self.result.push(' ');
    }

    fn add_operator(&mut self) {
        if !self.tokens[self.idx].is_add_op() {
            self.panic_with_error("invalid add operator");
        }
        self.result.push(' ');
        // self.result.push_str(self.tokens[self.idx].get_text());
        self.result
            .push_str(&self.prettifier.prettify(self.tokens[self.idx].clone()));
        self.result.push(' ');
    }

    fn mult_operator(&mut self) {
        if !self.tokens[self.idx].is_mult_op() {
            self.panic_with_error("invalid add operator");
        }
        self.result.push(' ');
        // self.result.push_str(self.tokens[self.idx].get_text());
        self.result
            .push_str(&self.prettifier.prettify(self.tokens[self.idx].clone()));
        self.result.push(' ');
    }
}
