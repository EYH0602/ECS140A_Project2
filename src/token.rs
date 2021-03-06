#[derive(Clone, Copy)]
pub enum TokenType {
    NONE,
    INTCONSTANT,
    FLOATCONSTANT,
    OPERATOR,
    KEYWORD,
    VARIABLE,
    FUNCTION,
    INVALID,
}

impl TokenType {
    pub fn as_str(&self) -> &'static str {
        match &self {
            TokenType::NONE => "None",
            TokenType::INTCONSTANT => "IntConstant",
            TokenType::FLOATCONSTANT => "FloatConstant",
            TokenType::OPERATOR => "Operator",
            TokenType::KEYWORD => "Keyword",
            TokenType::VARIABLE => "Variable",
            TokenType::FUNCTION => "Function",
            TokenType::INVALID => "Invalid",
        }
    }
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

#[derive(Clone)]
pub struct Token {
    text: String,
    token_type: TokenType,
    line_number: i32,
    char_position: i32,
}

impl Token {
    pub fn new(s: String, t: TokenType, linenum: i32, charpos: i32) -> Token {
        Token {
            text: s,
            token_type: t,
            line_number: linenum,
            char_position: charpos,
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn get_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn get_line_number(&self) -> i32 {
        self.line_number
    }

    pub fn get_char_pos(&self) -> i32 {
        self.char_position
    }

    pub fn is_type(&self) -> bool {
        let types = vec![
            "unsigned", "char", "short", "int", "long", "float", "double",
        ];
        types.contains(&self.get_text())
    }

    pub fn is_relational_op(&self) -> bool {
        let relational_ops = vec!["==", "<", ">", "<=", ">=", "!="];
        self.token_type == TokenType::OPERATOR && relational_ops.contains(&self.get_text())
    }

    pub fn is_add_op(&self) -> bool {
        let add_ops = vec!["+", "-"];
        self.token_type == TokenType::OPERATOR && add_ops.contains(&self.get_text())
    }

    pub fn is_mult_op(&self) -> bool {
        let mult_ops = vec!["*", "/"];
        self.token_type == TokenType::OPERATOR && mult_ops.contains(&self.get_text())
    }
}
