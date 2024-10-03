use evaluator::Evaluator;
use tokenizer::Tokens;

mod tokenizer;
mod evaluator;

fn main() {
    // let s = "((p <-> q) = (p -> q) and (q -> p))";
    // let s = "((p and q) -> p)";
    // let s = "((p->q) and ((not p) -> q) -> q)";
    // let s = "((p->q) = ((not q) -> (not p)))";
    let s = "((p->q) = (not p) or q)";

    let tokens = Tokens::from_text(s);
    if let Ok(evaluator) = Evaluator::new(tokens) {
        if let Ok(result) = evaluator.evaluate_all() {
            print!("{}\n",result);
        }
    }
}
