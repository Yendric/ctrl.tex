use crate::token::Token;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.input.peek() {
            Some(&'\\') => {
                self.input.next();
                self.read_command()
            }
            Some(&'{') => {
                self.input.next();
                Token::LBrace
            }
            Some(&'}') => {
                self.input.next();
                Token::RBrace
            }
            Some(&'[') => {
                self.input.next();
                Token::LBracket
            }
            Some(&']') => {
                self.input.next();
                Token::RBracket
            }
            Some(&'(') => {
                self.input.next();
                Token::LParen
            }
            Some(&')') => {
                self.input.next();
                Token::RParen
            }
            Some(&'^') => {
                self.input.next();
                Token::Superscript
            }
            Some(&'_') => {
                self.input.next();
                Token::Subscript
            }
            Some(&'%') => {
                self.skip_comment();
                self.next_token()
            }
            Some(_) => {
                let c = self.input.next().unwrap();
                Token::Char(c)
            }
            None => Token::EOF,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        self.input.next();
        while let Some(&c) = self.input.peek() {
            if c == '\n' || c == '\r' {
                break;
            }
            self.input.next();
        }
    }

    fn read_command(&mut self) -> Token {
        if let Some(&c) = self.input.peek() {
            if c.is_alphabetic() {
                let mut command = String::new();
                while let Some(&c) = self.input.peek() {
                    if c.is_alphabetic() {
                        command.push(c);
                        self.input.next();
                    } else {
                        break;
                    }
                }
                Token::Command(command)
            } else {
                let c = self.input.next().unwrap();
                Token::Command(c.to_string())
            }
        } else {
            Token::Command("".to_string())
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        if token == Token::EOF {
            None
        } else {
            Some(token)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn test_simple_tokens() {
        let input = "{ } ^ _ [ ] ( )";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::LBrace);
        assert_eq!(lexer.next_token(), Token::RBrace);
        assert_eq!(lexer.next_token(), Token::Superscript);
        assert_eq!(lexer.next_token(), Token::Subscript);
        assert_eq!(lexer.next_token(), Token::LBracket);
        assert_eq!(lexer.next_token(), Token::RBracket);
        assert_eq!(lexer.next_token(), Token::LParen);
        assert_eq!(lexer.next_token(), Token::RParen);
        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_commands() {
        let input = r"\alpha \beta \gamma";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::Command("alpha".to_string()));
        assert_eq!(lexer.next_token(), Token::Command("beta".to_string()));
        assert_eq!(lexer.next_token(), Token::Command("gamma".to_string()));
        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_escaped_chars() {
        let input = r"\{ \} \% \\";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::Command("{".to_string()));
        assert_eq!(lexer.next_token(), Token::Command("}".to_string()));
        assert_eq!(lexer.next_token(), Token::Command("%".to_string()));
        assert_eq!(lexer.next_token(), Token::Command("\\".to_string()));
        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_mixed() {
        let input = r"\frac{1}{2} + x^2";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::Command("frac".to_string()));
        assert_eq!(lexer.next_token(), Token::LBrace);
        assert_eq!(lexer.next_token(), Token::Char('1'));
        assert_eq!(lexer.next_token(), Token::RBrace);
        assert_eq!(lexer.next_token(), Token::LBrace);
        assert_eq!(lexer.next_token(), Token::Char('2'));
        assert_eq!(lexer.next_token(), Token::RBrace);
        assert_eq!(lexer.next_token(), Token::Char('+'));
        assert_eq!(lexer.next_token(), Token::Char('x'));
        assert_eq!(lexer.next_token(), Token::Superscript);
        assert_eq!(lexer.next_token(), Token::Char('2'));
        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_comments() {
        let input = "x % this is a comment\n y";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::Char('x'));
        assert_eq!(lexer.next_token(), Token::Char('y'));
        assert_eq!(lexer.next_token(), Token::EOF);
    }
}
