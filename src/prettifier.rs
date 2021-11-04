use crate::token::Token;
use crate::token::TokenType;

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
}
