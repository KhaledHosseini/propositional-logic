use std::collections::HashMap;

use evaluator::Evaluator;
use indexmap::IndexMap;
use tokenizer::Tokens;

mod tokenizer;
mod evaluator;

fn main() {
    let s = "((not a) -> b)";
    let tokens = Tokens::from_text(s);
    if let Ok(evaluator) = Evaluator::new(tokens) {
        if let Ok(result) = evaluator.evaluate_all() {
            print!("{}\n",result);
        }
    }
}
