#[path = "./character_stream.rs"]
mod character_stream;
use character_stream::*;

#[path = "./token.rs"]
mod token;
use token::*;

use std::collections::HashMap;

pub struct Scanner {
    text: CharStream,
    ending_char: Vec<char>,
    operator_char: Vec<char>,
    id_map: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(f: &str) -> Scanner {
        Scanner {
            text: CharStream::new(f),
            ending_char: vec!['(', ')', '{', '}', ' ', '\n', ';', '\t'],
            operator_char: vec!['=', '+', '-', '*', '/', '<', '>'],
            id_map: HashMap::from([
                (String::from("unsigned"), TokenType::KEYWORD),
                (String::from("char"), TokenType::KEYWORD),
                (String::from("short"), TokenType::KEYWORD),
                (String::from("int"), TokenType::KEYWORD),
                (String::from("long"), TokenType::KEYWORD),
                (String::from("float"), TokenType::KEYWORD),
                (String::from("double"), TokenType::KEYWORD),
                (String::from("void"), TokenType::KEYWORD),
                (String::from("main"), TokenType::KEYWORD),
                (String::from("while"), TokenType::KEYWORD),
                (String::from("for"), TokenType::KEYWORD),
                (String::from("if"), TokenType::KEYWORD),
                (String::from("return"), TokenType::KEYWORD),
            ]),
        }
    }

    // skip any white space or next line
    fn trim(&mut self) {
        while self.text.more_available() {
            match self.text.peek_next_char() {
                None => break,
                Some(ch) => {
                    if self.ending_char.contains(&ch) {
                        self.text.get_next_char();
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn lookup(&self, word: String) -> TokenType {
        match self.id_map.get(&word) {
            None => TokenType::VARIABLE,
            Some(&t) => t,
        }
    }

    // check is the following operator is an operator
    fn is_next_operator(&self) -> bool {
        match self.text.peek_next_char() {
            None => false,
            Some(ch) => self.operator_char.contains(&ch),
        }
    }

    // check is the following operator is an open parenthesis
    fn is_next_parenthesis(&self) -> bool {
        match self.text.peek_next_char() {
            Some('(') => true,
            _ => false,
        }
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        let mut curr_word = String::from("");
        let mut curr_type: TokenType = TokenType::NONE;
        let mut line_num: i32 = 0;
        let mut char_pos: i32 = 0;
        let mut has_found: bool = false;

        self.trim();

        while self.text.more_available() {
            line_num += 1;
            match self.text.get_next_char() {
                None => break,
                Some(ch) => match self.ending_char.contains(&ch) {
                    true => match curr_type {
                        TokenType::VARIABLE => {
                            if ch == '(' {
                                match self.id_map.get(&curr_word) {
                                    None => {
                                        self.id_map.insert(curr_word.clone(), TokenType::FUNCTION);
                                    },
                                    _ => {}
                                }
                            }
                            break;
                        }
                        _ => break,
                    },
                    false => {
                        match curr_type {
                            TokenType::INVALID => break,
                            TokenType::INTCONSTANT => {
                                if ch.is_digit(10) {
                                    curr_word.push(ch);
                                } else if ch == '.' {
                                    curr_type = TokenType::FLOATCONSTANT;
                                    curr_word.push(ch);
                                } else {
                                    curr_type = TokenType::INVALID;
                                    break;
                                }
                            }
                            TokenType::FLOATCONSTANT => {
                                if ch.is_digit(10) {
                                    curr_word.push(ch);
                                } else {
                                    curr_type = TokenType::INVALID;
                                    break;
                                }
                            }
                            _ => {
                                has_found = true;
                                curr_word.push(ch);
                                char_pos += 1;
                                if self.operator_char.contains(&ch) {
                                    curr_type = TokenType::OPERATOR;
                                    match self.text.peek_next_char() {
                                        None => break,
                                        Some(ch) => {
                                            if ch == '=' {
                                                curr_word.push(ch);
                                                self.text.get_next_char();
                                            }
                                        }
                                    }
                                    break;
                                } else if ch.is_digit(10) {
                                    curr_type = TokenType::INTCONSTANT;
                                } else if ch.is_ascii_alphabetic() {
                                    curr_type = TokenType::VARIABLE;
                                } else {
                                    curr_type = TokenType::INVALID;
                                    break;
                                }
                            }
                        }
                        if self.is_next_operator() {
                            break;
                        }
                    }
                },
            }
        }

        match has_found {
            false => None,
            true => {
                match curr_type {
                    TokenType::VARIABLE => curr_type = self.lookup(curr_word.clone()),
                    _ => {}
                }
                let res = Token::new(curr_word, curr_type, line_num, char_pos);
                Some(res)
            }
        }
    }
}
