#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Command(String),
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Superscript,
    Subscript,
    Char(char),
    EOF,
}
