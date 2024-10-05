use std::collections::{HashSet, VecDeque};
use indexmap::{IndexMap, IndexSet};
use crate::{evaluator_result::EvaluatorResult, tokenizer::{Token, Tokens}};
use std::fmt;


#[derive(Debug)]
pub struct EvaluatorError {
    message: String,
}
impl fmt::Display for EvaluatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub struct Evaluator {
   tokens: Tokens,
   idents: HashSet<char>
}

impl Evaluator {
    pub fn new(tokens: Tokens)-> Result<Self,EvaluatorError> {
        //do some validation
        let mut tokens = tokens;
        fn check_enclosing(tkns: &[Token], left: Token, right: Token)-> Result<(),EvaluatorError> {
            let mut p_count = 0;
            for t in tkns {
                if t == &left {
                    p_count +=1;
                }else if t == &right {
                    p_count -=1;
                }
            }
            if p_count != 0 {
                return  Err(EvaluatorError { message: "mis-match parentheses".into() });
            }
            Ok(())
        }
        let tkns: &[Token] = &tokens;
        if tkns.len() == 0 {
            return  Err(EvaluatorError { message: "Empty expression.".into() });
        }

        check_enclosing(tkns, Token::OpenParen, Token::CloseParen)?;
        check_enclosing(tkns, Token::OpenBracket, Token::CloseBracket)?;
        check_enclosing(tkns, Token::OpenCurlyBrace, Token::CloseCurlyBrace)?;
        
        tokens.enclose(Token::OpenParen, Token::CloseParen);

        let idents: HashSet<char> = tokens.iter().filter_map(|t| {
            match t {
                Token::Ident(chr)=> {
                    return Some(*chr);
                }
                _ => None
            }
        }).collect();
        Ok(Evaluator { tokens , idents})
    }
}

impl Evaluator {
    pub fn evaluate(&self, values: &IndexMap<char,bool>) -> Result<IndexMap<String, bool>, EvaluatorError> {
        //validate values
        if !self.idents.iter().all(|x| values.contains_key(x)) {
            return Err(EvaluatorError{message: "Please provide value for all idents.".into()});
        }
        let mut operators_stack = VecDeque::<Token>::new();
        let mut operands_stack = VecDeque::<(Option<String>,Token)>::new();
        let mut result: IndexMap<String,bool> = IndexMap::new();
        let tokens: &[Token] = &self.tokens;
        for token in tokens {
            match token {
                Token::Ident(chr) => {
                     let v = values.get(chr).unwrap();
                     let v = if *v {
                        Token::True
                    }else {
                        Token::False
                    };
                    operands_stack.push_front((Some(chr.to_string()),v));
                },
                Token::OpenParen | Token::OpenBracket | Token::OpenCurlyBrace => {
                    operators_stack.push_front(token.clone());
                },
                Token::CloseParen | Token::CloseBracket | Token::CloseCurlyBrace => {
                  while let Some(op) = operators_stack.pop_front() {
                      if op == Token::OpenParen || 
                         op == Token::OpenBracket || 
                         op == Token::OpenCurlyBrace {break;}
                      Self::evaluate_operator(op, &mut operands_stack);
                      let res = operands_stack.front().unwrap().clone();
                      if let Some(name) = res.0 {
                        result.insert(name, res.1.into());
                      }
                  }  
                },
                Token::False | Token::True => operands_stack.push_front((None,token.clone())),
                _ => {
                    while operators_stack.len() > 0 && (get_priority(&operators_stack.front().unwrap())<=get_priority(&token)) {
                        Self::evaluate_operator(operators_stack.pop_front().unwrap(),&mut operands_stack);
                        let res = operands_stack.front().unwrap().clone();
                        if let Some(name) = res.0 {
                            result.insert(name, res.1.into());
                        }
                    }
                    operators_stack.push_front(token.clone());
                }
            }
        }
        Ok(result)
    }
    fn evaluate_operator(operator: Token, operands_stack: &mut VecDeque::<(Option<String>,Token)>){
        
        let op_symbol: String;
        let ev_result: Token;
        let opnd_count = get_operands_count(&operator);
        let opnd1_ =  operands_stack.pop_front().unwrap();
        let opnd1: bool = opnd1_.1.into();
        let mut opnd2_: (Option<String>, Token) = (None,Token::False);
        let mut opnd2 = false;
        if opnd_count == 2 {
            opnd2_ = operands_stack.pop_front().unwrap();
            opnd2 = opnd2_.1.into();
        }
        
        match operator {
            Token::Not(symbol) => {
                ev_result = Token::from(!opnd1);
                op_symbol = symbol.into();
            },
            Token::Implication(symbol) =>  {
                let opnd = if !opnd2 {
                    true
                }else {
                    opnd1
                };
                ev_result = Token::from(opnd);
                op_symbol = symbol.into();
            },
            Token::Biconditional(symbol) =>  {
                let opnd = if (opnd1 && opnd2) || (!opnd1 && !opnd2) {
                    true
                }else {
                    false
                };
                ev_result = Token::from(opnd);
                op_symbol = symbol.into();
            },
            Token::And(symbol) => {
                let opnd = opnd1 && opnd2;
                ev_result = Token::from(opnd);
                op_symbol = symbol.into();
            },
            Token::Or(symbol) => {
                let opnd = opnd1 || opnd2;
                ev_result = Token::from(opnd);
                op_symbol = symbol.into();
            },
            Token::XOr(symbol) => {
                let opnd = (opnd1 && !opnd2) || (!opnd1 && opnd2);
                ev_result = Token::from(opnd);
                op_symbol = symbol.into();
            },
            Token::Equals(symbol) => {
                let opnd = (opnd1 && opnd2) || (!opnd1 && !opnd2);
                ev_result = Token::from(opnd);
                op_symbol = symbol.to_string();
            },
            Token::NotEquals(symbol) => {
                let opnd = (!opnd1 && opnd2) || (opnd1 && !opnd2);
                ev_result = Token::from(opnd);
                op_symbol = symbol.into();
            },
            _ => todo!(),
        }
        
        if opnd_count == 2 {
            if let (Some(nm1),Some(nm2)) = (opnd1_.0,opnd2_.0) {
                let name = format!("({} {} {})",nm2,op_symbol,nm1);
                operands_stack.push_front((Some(name),ev_result));
            }
        }else {
            if let Some(nm) = opnd1_.0 {
                let name = format!("{}{}",op_symbol,nm);
                operands_stack.push_front((Some(name),ev_result));
            }
        }
    }

    pub fn evaluate_all(&self)-> Result<EvaluatorResult,EvaluatorError> {
        let mut result: Vec<IndexMap<String, bool>> = Vec::new();
        let tokens: &[Token] = &self.tokens;
        let operands: Vec<char> = tokens.iter().filter_map(|t| {
            match t {
                Token::Ident(chr)=> {
                    return Some(*chr);
                }
                _ => None
            }
        }).collect::<IndexSet<_>>()
        .into_iter()
        .collect();
        let n = operands.len();
        let rows: u64 = 1 << n;//2^n
        for i in 0..rows {
            let mut row: IndexMap<char,bool> = IndexMap::new();
            for j in (0..n).rev() {
                let v = ((i >> j) & 1) != 0;
                row.insert(operands[n - j - 1], !v);
            }
            if let Ok(mut eval) = self.evaluate(&row){
                for r in row.iter().rev() {
                    eval.insert_before(0, r.0.to_string(), *r.1);
                }
                result.push(eval);
            }
        }
        Ok(EvaluatorResult { result })
    }
}


fn get_priority(op_token: &Token)-> usize {
    match op_token {
        Token::Ident(_) => return usize::MAX,
        Token::OpenParen | Token::CloseParen |
        Token::OpenBracket | Token::CloseBracket |
        Token::OpenCurlyBrace | Token::CloseCurlyBrace => return usize::MAX,
        Token::Not(_) => return  0,
        Token::Implication(_) => 3,
        Token::Biconditional(_) => 3,
        Token::And(_) => return  1,
        Token::Or(_) => return  2,
        Token::XOr(_) => return  2,
        Token::Equals(_) => 4,
        Token::NotEquals(_) => 5,
        Token::False => return usize::MAX,
        Token::True => return usize::MAX,
    }
}
fn get_operands_count(op_token: &Token)-> usize {
    match op_token {
        Token::Ident(_) => return 0,
        Token::OpenParen | Token::CloseParen |
        Token::OpenBracket | Token::CloseBracket |
        Token::OpenCurlyBrace | Token::CloseCurlyBrace => return 0,
        Token::Not(_) => return  1,
        Token::Implication(_) => 2,
        Token::Biconditional(_) => 2,
        Token::And(_) => return  2,
        Token::Or(_) => return  2,
        Token::XOr(_) => return  2,
        Token::Equals(_) => 2,
        Token::NotEquals(_) => 2,
        Token::False => return 0,
        Token::True => return 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::Tokens;

    use super::*;

    #[test]
    fn not() {
        let symbols = ["not", "¬", "!", "∼"];
        for s in symbols {
            let expr = format!("{} a",s);
            let tokens = Tokens::from_text(&expr);
            let evaluator = Evaluator::new(tokens).unwrap();
            let mut values = IndexMap::<char,bool>::new();
            values.insert('a', true);
            let result = evaluator.evaluate(&values).unwrap();
            assert_eq!(
                result.get("¬a").unwrap(),&false
            );
        }
        
    }

    fn check(evaluator: &Evaluator, a: bool, b:bool, expect: bool, expr: &str) {
        let mut values = IndexMap::<char,bool>::new();
        values.insert('a', a);
        values.insert('b', b);
        let result = evaluator.evaluate(&values).unwrap();
        assert_eq!(
            result.get(expr).unwrap(),&expect
        );
    }

    #[test]
    fn and() {
        let symbols = ["and", "&", "&&", "∧"];
        for s in symbols {
            let expr = format!("a {} b",s);
            let tokens = Tokens::from_text(&expr);
            let evaluator = Evaluator::new(tokens).unwrap();
            let expr = "(a ∧ b)";
            check(&evaluator, true, true, true,&expr);
            check(&evaluator, true, false, false,&expr);
            check(&evaluator, false, true, false,&expr);
            check(&evaluator, false, false, false,&expr);
        }
    }

    #[test]
    fn or() {
        let symbols = ["or", "|", "||", "∨"];
        for s in symbols {
            let expr = format!("a {} b",s);
            let tokens = Tokens::from_text(&expr);
            let evaluator = Evaluator::new(tokens).unwrap();
            let expr = "(a ∨ b)";
            check(&evaluator, true, true, true,&expr);
            check(&evaluator, true, false, true,&expr);
            check(&evaluator, false, true, true,&expr);
            check(&evaluator, false, false, false,&expr);
        }
    }
    #[test]
    fn xor() {
        let symbols = ["xor", "⊕"];
        for s in symbols {
            let expr = format!("a {} b",s);
            let tokens = Tokens::from_text(&expr);
            let evaluator = Evaluator::new(tokens).unwrap();
            let expr = "(a ⊕ b)";
            check(&evaluator, true, true, false,&expr);
            check(&evaluator, true, false, true,&expr);
            check(&evaluator, false, true, true,&expr);
            check(&evaluator, false, false, false,&expr);
        }
    }
    #[test]
    fn implication() {
        let symbols = ["->", "=>", "⇒", "→", "⊃"];
        for s in symbols {
            let expr = format!("a {} b",s);
            let tokens = Tokens::from_text(&expr);
            let evaluator = Evaluator::new(tokens).unwrap();
            let expr = "(a → b)";
            check(&evaluator, true, true, true,&expr);
            check(&evaluator, true, false, false,&expr);
            check(&evaluator, false, true, true,&expr);
            check(&evaluator, false, false, true,&expr);
        }
        
    }

    #[test]
    fn biconditional() {
        let symbols = ["<->", "<=>", "⇔", "↔", "iff", "xnor"];
        for s in symbols {
            let expr = format!("a {} b",s);
            let tokens = Tokens::from_text(&expr);
            let evaluator = Evaluator::new(tokens).unwrap();
            let expr = "(a ↔ b)";
            check(&evaluator, true, true, true,&expr);
            check(&evaluator, true, false, false,&expr);
            check(&evaluator, false, true, false,&expr);
            check(&evaluator, false, false, true,&expr);
        }
    }
}