use std::{
    fmt::{self, Display, Write},
    ops::Deref,
};

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex("[a-zA-Z]+", |lex| lex.slice().chars().next().unwrap())]
    Ident(char),
    #[token("(")]
    #[token("[")]
    #[token("{")]
    OpenParen,
    #[token(")")]
    #[token("]")]
    #[token("}")]
    CloseParen,
    #[token("not")]
    #[token("!")]
    Not,
    #[token("->")]
    Implication,
    #[token("<->")]
    ReciprocalImplication,
    #[token("and")]
    #[token("&")]
    #[token("&&")]
    And,
    #[token("or")]
    #[token("|")]
    #[token("||")]
    Or,
    #[token("=")]
    #[token("==")]
    #[token("eq")]
    Equals,
    #[token("!=")]
    NotEquals,
    #[token("0")]
    False,
    #[token("1")]
    True,
}

impl Into<bool> for Token {
    fn into(self) -> bool{
        if let Token::True = self {
            return  true;
        }
        false
    }
}

impl From<bool> for Token {
    fn from(value: bool) -> Self {
        if value {
            return Token::True;
        }
        Token::False
    }
}

#[derive(Debug)]
pub struct Tokens {
    tokens: Vec<Token>,
}

impl Tokens {
    pub fn from_text(text: &str) -> Self {
        Self {
            tokens: Token::lexer(text).filter_map(|result| result.ok()).collect(),
        }
    }
}

impl Deref for Tokens {
    type Target = [Token];
    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}

impl Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for token in self.tokens.iter() {
            match token {
                Token::Ident(c) => f.write_char(*c),
                Token::OpenParen => f.write_char('('),
                Token::CloseParen => f.write_char(')'),
                Token::Not => f.write_char('¬'),
                Token::Implication => f.write_str(" → "),
                Token::ReciprocalImplication => f.write_str(" ⟷ "),
                Token::And => f.write_str(" ∧ "),
                Token::Or => f.write_str(" ∨ "),
                Token::Equals => f.write_str(" = "),
                Token::NotEquals => f.write_str(" ≠ "),
                Token::False => f.write_char('0'),
                Token::True => f.write_char('1'),
            }?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn implication() {
        let s = "a -> b";
        assert_eq!(
            Tokens::from_text(s).tokens,
            vec![Token::Ident('a'), Token::Implication, Token::Ident('b')]
        );
    }

    #[test]
    fn equals() {
        let s = "a == b";
        assert_eq!(
            Tokens::from_text(s).tokens,
            vec![Token::Ident('a'), Token::Equals, Token::Ident('b')]
        );
    }

    #[test]
    fn not_equals() {
        let s = "a != b";
        assert_eq!(
            Tokens::from_text(s).tokens,
            vec![Token::Ident('a'), Token::NotEquals, Token::Ident('b')]
        );
    }

    #[test]
    fn not() {
        let s = "(not a)";
        let tokens = Tokens::from_text(s).tokens;
        assert_eq!(
            tokens,
            vec![Token::OpenParen,Token::Not, Token::Ident('a'),Token::CloseParen]
        );
    }
    // #[test]
    // fn complex() {
    //     let s = "((p <-> q) = (p -> q) and (q -> p))";
    //     let tokens = Tokens::from_text(s).tokens;
    //     assert_eq!(
    //         tokens,
    //         vec![Token::OpenParen,Token::Not, Token::Ident('a'),Token::CloseParen]
    //     );
    // }
}