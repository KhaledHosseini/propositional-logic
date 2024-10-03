use std::{collections::{HashMap, VecDeque}, fmt::Error};

use crate::tokenizer::{Token, Tokens};
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
   tokens: Tokens
}

impl Evaluator {
    pub fn new(tokens: Tokens)-> Result<Self,EvaluatorError> {
        //do some validation
        let tkns: &[Token] = &tokens;
        let mut p_count = 0;
        for t in tkns {
            if let Token::OpenParen = t {p_count +=1;}
            else if let Token::CloseParen = t {
                p_count -=1;
            }
        }
        if p_count != 0 {
            return  Err(EvaluatorError { message: "mis match parentheses".into() });
        }
        Ok(Evaluator { tokens })
    }
}

impl Evaluator {
    pub fn evaluate(&self, values: HashMap<char,char>) -> Result<HashMap<String, bool>, Error> {
        
        let mut operators_stack = VecDeque::<Token>::new();
        let mut operands_stack = VecDeque::<(Option<String>,Token)>::new();
        let mut result: HashMap<String,bool> = HashMap::new();
        let tokens: &[Token] = &self.tokens;
        for token in tokens {
            match token {
                Token::Ident(chr) => {
                     let v = values.get(&chr).unwrap();
                     let v = if v.eq_ignore_ascii_case(&'T') {
                        Token::True
                    }else {
                        Token::False
                    };
                    operands_stack.push_front((Some(chr.to_string()),v));
                },
                Token::OpenParen => {
                    operators_stack.push_front(token.clone());
                },
                Token::CloseParen => {
                  while let Some(op) = operators_stack.pop_front() {
                      if op == Token::OpenParen {break;}
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
                    }
                    operators_stack.push_front(token.clone());
                }
            }
        }
        Ok(result)
    }
    fn evaluate_operator(operator: Token, operands_stack: &mut VecDeque::<(Option<String>,Token)>){
        
        let mut ev_name = String::new();

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
            Token::Not => {
                let ev_result = Token::from(!opnd1);
                if let Some(nm) = opnd1_.0 {
                    ev_name = format!("¬{}",nm);
                }
                operands_stack.push_front((Some(ev_name),ev_result));
            },
            Token::Implication =>  {
                let opnd = if !opnd2 {
                    true
                }else {
                    opnd1
                };
                let ev_result = Token::from(opnd);
                if let (Some(nm1),Some(nm2)) = (opnd1_.0,opnd2_.0) {
                    ev_name = format!("{} → {}",nm2,nm1);
                }
                operands_stack.push_front((Some(ev_name),ev_result));
            },
            Token::ReciprocalImplication =>  {
                let opnd = if (opnd1 && opnd2) || (!opnd1 && !opnd2) {
                    true
                }else {
                    false
                };
                let ev_result = Token::from(opnd);
                if let (Some(nm1),Some(nm2)) = (opnd1_.0,opnd2_.0) {
                    ev_name = format!("{} ⟷ {}",nm2,nm1);
                }
                operands_stack.push_front((Some(ev_name),ev_result));
            },
            Token::And => {
                let opnd = opnd1 && opnd2;
                let ev_result = Token::from(opnd);
                if let (Some(nm1),Some(nm2)) = (opnd1_.0,opnd2_.0) {
                    ev_name = format!("{} ∧ {}",nm2,nm1);
                }
                operands_stack.push_front((Some(ev_name),ev_result));
            },
            Token::Or => {
                let opnd = opnd1 || opnd2;
                let ev_result = Token::from(opnd);
                if let (Some(nm1),Some(nm2)) = (opnd1_.0,opnd2_.0) {
                    ev_name = format!("{} ∨ {}",nm2,nm1);
                }
                operands_stack.push_front((Some(ev_name),ev_result));
            },
            Token::Equals => {
                let opnd = (opnd1 && opnd2) || (!opnd1 && !opnd2);
                let ev_result = Token::from(opnd);
                if let (Some(nm1),Some(nm2)) = (opnd1_.0,opnd2_.0) {
                    ev_name = format!("{} = {}",nm2,nm1);
                }
                operands_stack.push_front((Some(ev_name),ev_result));
            },
            Token::NotEquals => {
                let opnd = (!opnd1 && opnd2) || (opnd1 && !opnd2);
                let ev_result = Token::from(opnd);
                if let (Some(nm1),Some(nm2)) = (opnd1_.0,opnd2_.0) {
                    ev_name = format!("{} ≠ {}",nm2,nm1);
                }
                operands_stack.push_front((Some(ev_name),ev_result));
            },
            _ => todo!(),
        }
    }
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


fn get_priority(op_token: &Token)-> usize {
    match op_token {
        Token::Ident(_) => return usize::MAX,
        Token::OpenParen => return usize::MAX,
        Token::CloseParen => return usize::MAX,
        Token::Not => return  0,
        Token::Implication => 3,
        Token::ReciprocalImplication => 3,
        Token::And => return  1,
        Token::Or => return  2,
        Token::Equals => 4,
        Token::NotEquals => 5,
        Token::False => return usize::MAX,
        Token::True => return usize::MAX,
    }
}
fn get_operands_count(op_token: &Token)-> usize {
    match op_token {
        Token::Ident(_) => return 0,
        Token::OpenParen => return 0,
        Token::CloseParen => return 0,
        Token::Not => return  1,
        Token::Implication => 2,
        Token::ReciprocalImplication => 2,
        Token::And => return  2,
        Token::Or => return  2,
        Token::Equals => 2,
        Token::NotEquals => 2,
        Token::False => return 0,
        Token::True => return 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::Tokens;

    use super::*;

    #[test]
    fn test_not_with_parenthesis() {
        let s = "(not a)";
        let tokens = Tokens::from_text(s);
        let evaluator = Evaluator::new(tokens).unwrap();
        let mut values = HashMap::<char,char>::new();
        values.insert('a', 'T');
        let result = evaluator.evaluate(values).unwrap();
        assert_eq!(
            result.get("¬ a").unwrap(),&false
        );
        assert_eq!(1,1);
    }
}