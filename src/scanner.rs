#[path = "./character_stream.rs"]
mod character_stream;
use character_stream::*;

#[path = "./token.rs"]
mod token;
use token::*;

pub struct Scanner {
    text: CharStream,
    ending_char: Vec<char>,
    operator_char: Vec<char>,
}

impl Scanner {
    pub fn new(f: &str) -> Scanner {
        Scanner {
            text: CharStream::new(f),
            ending_char: vec!['(', ')', '{', '}', ' ', '\n', ';', '\t'],
            operator_char: vec!['=', '+', '-', '*', '/', '<', '>'],
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

    fn lookup(&self, word: &String) -> TokenType {
        TokenType::KEYWORD
    }

    // check is the following operator is an operator
    fn is_next_operator(&self) -> bool {
        match self.text.peek_next_char() {
            None => false,
            Some(ch) => self.operator_char.contains(&ch),
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
                    true => break,
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
                    TokenType::VARIABLE => curr_type = self.lookup(&curr_word),
                    _ => {}
                }
                let res = Token::new(curr_word, curr_type, line_num, char_pos);
                Some(res)
            }
        }
    }
}
