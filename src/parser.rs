use crate::ast::{CommandRegistry, Expr};
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut p = Parser {
            lexer,
            current_token: Token::EOF,
        };
        p.advance();
        p
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Vec<Expr> {
        let mut exprs = Vec::new();
        while self.current_token != Token::EOF {
            if self.current_token == Token::RBrace {
                break;
            }
            if let Some(expr) = self.parse_expr() {
                exprs.push(expr);
            } else {
                self.advance();
            }
        }
        exprs
    }

    fn parse_sequence(&mut self) -> Vec<Expr> {
        let mut exprs = Vec::new();
        while self.current_token != Token::EOF && self.current_token != Token::RBrace {
            if let Some(expr) = self.parse_expr() {
                exprs.push(expr);
            } else {
                self.advance();
            }
        }
        exprs
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        let mut base = self.parse_base()?;

        loop {
            match self.current_token {
                Token::Superscript => {
                    self.advance();
                    if let Some(exponent) = self.parse_base() {
                        base = Expr::Superscript(Box::new(base), Box::new(exponent));
                    } else {
                        break;
                    }
                }
                Token::Subscript => {
                    self.advance();
                    if let Some(subscript) = self.parse_base() {
                        base = Expr::Subscript(Box::new(base), Box::new(subscript));
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        Some(base)
    }

    fn parse_base(&mut self) -> Option<Expr> {
        match &self.current_token {
            Token::Char(c) => {
                let expr = Expr::Literal(*c);
                self.advance();
                Some(expr)
            }
            Token::Command(s) => {
                let name = s.clone();
                self.advance();

                let arity = CommandRegistry::arity(&name);
                let mut args = Vec::with_capacity(arity);
                for _ in 0..arity {
                    if let Some(arg) = self.parse_base() {
                        args.push(arg);
                    } else {
                        break;
                    }
                }
                Some(Expr::Command(CommandRegistry::build(&name, args)))
            }
            Token::LBrace => {
                self.advance();
                let content = self.parse_sequence();
                if self.current_token == Token::RBrace {
                    self.advance();
                }
                Some(Expr::Group(content))
            }
            Token::LParen => {
                self.advance();
                Some(Expr::Literal('('))
            }
            Token::RParen => {
                self.advance();
                Some(Expr::Literal(')'))
            }
            Token::LBracket => {
                self.advance();
                Some(Expr::Literal('['))
            }
            Token::RBracket => {
                self.advance();
                Some(Expr::Literal(']'))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_simple_parse() {
        let input = "a + b";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse();
        assert_eq!(ast.len(), 3);
        assert_eq!(ast[0], Expr::Literal('a'));
        assert_eq!(ast[1], Expr::Literal('+'));
        assert_eq!(ast[2], Expr::Literal('b'));
    }

    #[test]
    fn test_superscript() {
        let input = "x^2";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse();
        assert_eq!(ast.len(), 1);
        match &ast[0] {
            Expr::Superscript(base, exp) => {
                assert_eq!(**base, Expr::Literal('x'));
                assert_eq!(**exp, Expr::Literal('2'));
            }
            _ => panic!("Expected Superscript"),
        }
    }

    #[test]
    fn test_group() {
        let input = "{ab}";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse();
        assert_eq!(ast.len(), 1);
        match &ast[0] {
            Expr::Group(content) => {
                assert_eq!(content.len(), 2);
                assert_eq!(content[0], Expr::Literal('a'));
                assert_eq!(content[1], Expr::Literal('b'));
            }
            _ => panic!("Expected Group"),
        }
    }

    #[test]
    fn test_complex() {
        use crate::ast::Command;

        let input = r"\alpha_1^2";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse();
        assert_eq!(ast.len(), 1);
        // Structure: ((\alpha)_1)^2
        match &ast[0] {
            Expr::Superscript(base, exp) => {
                assert_eq!(**exp, Expr::Literal('2'));
                match &**base {
                    Expr::Subscript(inner_base, sub) => {
                        assert_eq!(**sub, Expr::Literal('1'));
                        assert_eq!(
                            **inner_base,
                            Expr::Command(Command::Symbol {
                                name: "alpha".to_string()
                            })
                        );
                    }
                    _ => panic!("Expected Subscript"),
                }
            }
            _ => panic!("Expected Superscript"),
        }
    }

    #[test]
    fn test_command_with_args() {
        use crate::ast::Command;

        let input = r"\frac{a}{b}";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse();
        assert_eq!(ast.len(), 1);
        match &ast[0] {
            Expr::Command(Command::Frac { numer, denom }) => {
                assert_eq!(**numer, Expr::Group(vec![Expr::Literal('a')]));
                assert_eq!(**denom, Expr::Group(vec![Expr::Literal('b')]));
            }
            _ => panic!("Expected Command Frac"),
        }
    }

    #[test]
    fn test_sqrt_with_arg() {
        use crate::ast::Command;

        let input = r"\sqrt{x}";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse();
        assert_eq!(ast.len(), 1);
        match &ast[0] {
            Expr::Command(Command::Sqrt { content }) => {
                assert_eq!(**content, Expr::Group(vec![Expr::Literal('x')]));
            }
            _ => panic!("Expected Command Sqrt"),
        }
    }
}
