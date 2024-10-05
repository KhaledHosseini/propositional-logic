use evaluator::Evaluator;
use tokenizer::Tokens;

mod tokenizer;
mod evaluator;
mod evaluator_result;

fn main() {
    // let s = "(p <-> q) = (p -> q) and (q -> p)";
    // let s = "(p and q) -> p";
    // let s = "((p->q) and ((not p) -> q) -> q)";
    // let s = "((p->q) = ((not q) -> (not p)))";
    // let s = "([p->q] = not p or q)";
    // let s = "{((q and p) or {q or p})}";
    // let s = "(p and q & r)";
    // let s = "P ∧ (∼ Q)";
    // let s = "P ⇒ Q";
    // let s = "(P ∧ Q) ∧ (Q ⇒∼ P)";
    // let s = "(P ∧ Q) ⇒ (P ∨ Q)";
    // let s = "(P ∧ (∼ Q)) ⇔ (P ⇒ Q)";
    // let s = "P ⇒ Q ≡ ∼ P ∨ Q";
    // let s = "P ∧ Q ≡ Q ∧ P";
    // let s= "(P ∨ Q) ∧ (P ∨ R)";
    // let s = "P ∨ (Q ∧ R) ≡ (P ∨ Q) ∧ (P ∨ R)";
    // let s = "[(p -> q) and (r -> s)] eq [(not q or not s) -> (not p or not r)]";
    let s = "(P ∧ (∼ Q)) ⇔ (P ⇒ Q)";
    
    let tokens = Tokens::from_text(s);
    match Evaluator::new(tokens) {
        Ok(evaluator) => {
            match evaluator.evaluate_all() {
                Ok(result) => {
                    print!("{}\n",result);
                    // result.save_to_html("test.html");
                    // result.save_to_csv("test.csv");
                    // result.save_to_pdf("test.pdf");
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
