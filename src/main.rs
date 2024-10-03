use std::collections::HashMap;

use evaluator::Evaluator;
use tokenizer::Tokens;

mod tokenizer;
mod evaluator;

fn main() {
    let s = "((not a) -> b)";
    let tokens = Tokens::from_text(s);
    if let Ok(evaluator) = Evaluator::new(tokens) {
        let mut values = HashMap::<char,char>::new();
        values.insert('a', 'T');
        values.insert('b', 'T');
        let result = evaluator.evaluate(values).unwrap();
        for r in result.iter() {
            print!("{}: {}\n",r.0,r.1);
        }
    }
}
