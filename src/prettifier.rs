use crate::token::Token;
use crate::token::TokenType;

use std::fmt;

#[derive(Clone)]
pub struct Prettifier {
    title: String,
    background: String,
    foreground: String,
    font: String,
    function: String,
    variable: String,
    float: String,
    int: String,
    operator: String,
    keyword: String,
}

impl Prettifier {
    pub fn new() -> Prettifier {
        Prettifier {
            title: String::from("ABC"),
            background: String::from("navy"),
            foreground: String::from("yellow"),
            font: String::from("Courier New"),
            function: String::from("orange"),
            variable: String::from("yellow"),
            float: String::from("aqua"),
            int: String::from("aqua"),
            operator: String::from("white"),
            keyword: String::from("white"),
        }
    }

    pub fn get_header(&self) -> String {
        String::from("<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">\n<html xmlns=\"http://www.w3.org/1999/xhtml\" xml:lang=\"en\">\n<head>\n<title>X Formatted file</title>\n</head>")
    }

    pub fn prettify(&self, token: Token) -> String {
        match token.get_type() {
            &TokenType::FUNCTION => self.format(&self.function, "", token.get_text(), ""),
            &TokenType::VARIABLE => self.format(&self.variable, "", token.get_text(), ""),
            &TokenType::FLOATCONSTANT => self.format(&self.float, "<b>", token.get_text(), "</b>"),
            &TokenType::INTCONSTANT => self.format(&self.int, "<b>", token.get_text(), "</b>"),
            &TokenType::OPERATOR => self.format(&self.operator, "<b>", token.get_text(), "</b>"),
            &TokenType::KEYWORD => self.format(&self.keyword, "<b>", token.get_text(), "</b>"),
            _ => self.format(&self.foreground, "", token.get_text(), ""),
        }
    }

    fn format(&self, color: &str, style_open: &str, text: &str, style_close: &str) -> String {
        let mut res = String::from("<font color=\"");
        res.push_str(color);
        res.push('\"');
        res.push_str(style_open);
        res.push_str(text);
        res.push_str(style_close);
        res.push_str("</font>");
        res
    }
}
