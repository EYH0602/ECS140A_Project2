use crate::token::Token;
use crate::token::TokenType;

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

#[derive(Clone)]
pub struct Prettifier {
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
    pub fn new(f: &str) -> Prettifier {
        let mut field_map: HashMap<String, usize> = HashMap::new();
        let mut val_map: HashMap<String, Vec<String>> = HashMap::new();
        match File::open(f) {
            Ok(file) => {
                let reader = BufReader::new(file);
                for (file_idx, line) in reader.lines().enumerate() {
                    let line = line.unwrap();
                    if file_idx == 0 {
                        for (i, key) in line.split(',').enumerate() {
                            field_map.insert(String::from(key), i);
                        }
                    } else {
                        let mut key: &str = "";
                        for (i, s) in line.split(',').enumerate() {
                            if i == 0 {
                                key = s;
                                val_map.insert(String::from(s), Vec::new());
                            }
                            match val_map.get_mut(&String::from(key)) {
                                Some(vector) => vector.push(String::from(s)),
                                _ => {}
                            }
                        }
                    }
                }
            }
            Err(_) => {
                panic!("Error opening file {}", f);
            }
        }

        // extract a String value from maps, category indicating row, field indicating column
        let get_value = |field: &str, category: &str| {
            val_map.get(category).unwrap()[*field_map.get(field).unwrap()].clone()
        };

        Prettifier {
            background: get_value("BACKGROUND", "DEFAULT"),
            foreground: get_value("FOREGROUND", "DEFAULT"),
            font: get_value("FONT", "DEFAULT"),
            function: get_value("FOREGROUND", "FUNCTION"),
            variable: get_value("FOREGROUND", "VARIABLE"),
            float: get_value("FOREGROUND", "FLOAT_CONSTANT"),
            int: get_value("FOREGROUND", "INT_CONSTANT"),
            operator: get_value("FOREGROUND", "OPERATOR"),
            keyword: get_value("FOREGROUND", "KEYWORD"),
        }
    }

    pub fn get_body_open(&self) -> String {
        let body_open = String::from(format!(
            "<body bgcolor=\"{}\" text=\"{}\" link=\"{}\" vlink=\"{}\">",
            self.background, self.foreground, self.foreground, self.foreground
        ));
        body_open
    }

    pub fn get_body_close(&self) -> String {
        let body_close = String::from("</body>");
        body_close
    }

    pub fn get_font_open(&self) -> String {
        let font_open = String::from(format!("<font face={}>", self.font));
        font_open
    }

    pub fn get_font_close(&self) -> String {
        let font_close = String::from("</font>");
        font_close
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
        res.push_str("\">");
        res.push_str(style_open);
        res.push_str(text);
        res.push_str(style_close);
        res.push_str("</font>");
        res
    }
}
