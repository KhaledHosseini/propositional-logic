use std::collections::{HashSet, VecDeque};
use indexmap::IndexMap;
use crate::tokenizer::{Token, Tokens};
use prettytable::Table;
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
pub struct EvaluatorResult {
    result: Vec<IndexMap<String, bool>>
}

impl fmt::Display for EvaluatorResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut table = Table::new();
        if let Some(header) = self.result.first() {
            let header = header.iter().map(|x| x.0).collect();
            table.add_row(header);
        }
        for row in self.result.iter() {
            let values = row.iter().map(|x| x.1.to_string()).collect();
            table.add_row(values);
        }
        table.printstd();
        write!(f, "")
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
            return  Err(EvaluatorError { message: "mis-match parentheses".into() });
        }
        Ok(Evaluator { tokens })
    }
}

impl Evaluator {
    pub fn evaluate(&self, values: &IndexMap<char,bool>) -> Result<IndexMap<String, bool>, EvaluatorError> {
        //validate values

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
            Token::Not => {
                ev_result = Token::from(!opnd1);
                op_symbol = "¬".into();
            },
            Token::Implication =>  {
                let opnd = if !opnd2 {
                    true
                }else {
                    opnd1
                };
                ev_result = Token::from(opnd);
                op_symbol = "→".into();
            },
            Token::ReciprocalImplication =>  {
                let opnd = if (opnd1 && opnd2) || (!opnd1 && !opnd2) {
                    true
                }else {
                    false
                };
                ev_result = Token::from(opnd);
                op_symbol = "⟷".into();
            },
            Token::And => {
                let opnd = opnd1 && opnd2;
                ev_result = Token::from(opnd);
                op_symbol = "∧".into();
            },
            Token::Or => {
                let opnd = opnd1 || opnd2;
                ev_result = Token::from(opnd);
                op_symbol = "∨".into();
            },
            Token::Equals => {
                let opnd = (opnd1 && opnd2) || (!opnd1 && !opnd2);
                ev_result = Token::from(opnd);
                op_symbol = "=".into();
            },
            Token::NotEquals => {
                let opnd = (!opnd1 && opnd2) || (opnd1 && !opnd2);
                ev_result = Token::from(opnd);
                op_symbol = "≠".into();
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
        }).collect::<HashSet<_>>()
        .into_iter()
        .collect();
        let n = operands.len();
        let rows: u64 = 1 << n;//2^n
        for i in 0..rows {
            let mut row: IndexMap<char,bool> = IndexMap::new();
            for j in (0..n).rev() {
                let v = ((i >> j) & 1) != 0;
                row.insert(operands[n - j - 1], v);
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
        let mut values = IndexMap::<char,bool>::new();
        values.insert('a', true);
        let result = evaluator.evaluate(&values).unwrap();
        assert_eq!(
            result.get("¬ a").unwrap(),&false
        );
        assert_eq!(1,1);
    }
}