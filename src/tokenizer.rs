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
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("[")]
    OpenBracket,
    #[token("]")]
    CloseBracket,
    #[token("{")]
    OpenCurlyBrace,
    #[token("}")]
    CloseCurlyBrace,
    #[token("not", |_| '¬')]
    #[token("!", |_| '¬')]
    #[token("¬", |_| '¬')]
    #[token("∼", |_| '¬')]
    #[token("~", |_| '¬')]
    Not(char),
    #[token("->", |_| '→')]
    #[token("=>", |_| '→')]
    #[token("⇒", |_| '→')]
    #[token("→", |_| '→')]
    #[token("⊃", |_| '→')]
    Implication(char),
    #[token("<->", |_| '↔')]
    #[token("<=>", |_| '↔')]
    #[token("⇔", |_| '↔')]
    #[token("↔", |_| '↔')]
    #[token("iff", |_| '↔')]
    #[token("xnor", |_| '↔')]
    Biconditional(char),
    #[token("and", |_| '∧')]
    #[token("&", |_| '∧')]
    #[token("&&", |_| '∧')]
    #[token("∧", |_| '∧')]
    And(char),
    #[token("or", |_| '∨')]
    #[token("|", |_| '∨')]
    #[token("||", |_| '∨')]
    #[token("∨", |_| '∨')]
    Or(char),
    #[token("xor", |_| '⊕')]
    #[token("⊕", |_| '⊕')]
    XOr(char),
    #[token("=", |_| '≡')]
    #[token("==", |_| '≡')]
    #[token("eq", |_| '≡')]
    #[token("≡", |_| '≡')]
    Equals(char),
    #[token("!=", |_| '≠')]
    #[token("≠", |_| '≠')]
    NotEquals(char),
    #[token("0")]
    #[token("false")]
    #[token("False")]
    False,
    #[token("1")]
    #[token("true")]
    #[token("True")]
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
    
    pub fn enclose(&mut self, left: Token,right: Token) {
        self.tokens.insert(0, left);
        self.tokens.push(right);
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
                Token::OpenBracket => f.write_char('['),
                Token::CloseBracket => f.write_char(']'),
                Token::OpenCurlyBrace => f.write_char('{'),
                Token::CloseCurlyBrace => f.write_char('}'),
                Token::Not(symb) |
                Token::Implication(symb) |
                Token::Biconditional(symb) |
                Token::And(symb) |
                Token::Or(symb) |
                Token::XOr(symb) | 
                Token::Equals(symb) | 
                Token::NotEquals(symb) => f.write_str(&format!(" {} ",symb.to_string())),
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
            vec![Token::Ident('a'), Token::Implication('→'), Token::Ident('b')]
        );
    }

    #[test]
    fn equals() {
        let s = "a == b";
        assert_eq!(
            Tokens::from_text(s).tokens,
            vec![Token::Ident('a'), Token::Equals('≡'), Token::Ident('b')]
        );
    }

    #[test]
    fn not_equals() {
        let s = "a != b";
        assert_eq!(
            Tokens::from_text(s).tokens,
            vec![Token::Ident('a'), Token::NotEquals('≠'), Token::Ident('b')]
        );
    }

    #[test]
    fn not() {
        let s = "(not a)";
        let tokens = Tokens::from_text(s).tokens;
        assert_eq!(
            tokens,
            vec![Token::OpenParen,Token::Not('¬'), Token::Ident('a'),Token::CloseParen]
        );
    }

    #[test]
    fn and() {
        let symbols = ["and", "&", "&&", "∧"];
        for s in symbols {
            let expr = format!("a {} b",s);
            let tokens = Tokens::from_text(&expr).tokens;
            assert_eq!(
                tokens,
                vec![Token::Ident('a'),Token::And('∧'), Token::Ident('b')]
            );
        }
    }

    #[test]
    fn or() {
        let symbols = ["or", "|", "||", "∨"];
        for s in symbols {
            let expr = format!("a {} b",s);
            let tokens = Tokens::from_text(&expr).tokens;
            assert_eq!(
                tokens,
                vec![Token::Ident('a'),Token::Or('∨'), Token::Ident('b')]
            );
        }
    }
}