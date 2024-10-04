use evaluator::Evaluator;
use tokenizer::Tokens;

mod tokenizer;
mod evaluator;

fn main() {
    // let s = "((p <-> q) = (p -> q) and (q -> p))";
    // let s = "((p and q) -> p)";
    // let s = "((p->q) and ((not p) -> q) -> q)";
    // let s = "((p->q) = ((not q) -> (not p)))";
    // let s = "([p->q] = not p or q)";
    let s = "q and p";

    let tokens = Tokens::from_text(s);
    match Evaluator::new(tokens) {
        Ok(evaluator) => {
            match evaluator.evaluate_all() {
                Ok(result) => {
                    print!("{}\n",result);
                },
                Err(er) => {
                    println!("{}",er);
                },
            }
        },
        Err(er) => {
            println!("{}",er);
        },
    }
}
