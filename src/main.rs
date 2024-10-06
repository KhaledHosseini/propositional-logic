use evaluator::Evaluator;
use indexmap::IndexMap;
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
    // let s = "not true or not false";
    let s = "(P and not Q) <-> (P -> Q)";//"(P ∧ (∼ Q)) ⇔ (P ⇒ Q)";
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

    //evaluate for specific values only.
    let s = "(∼ P) ∨ (∼ Q)";
    let tokens = Tokens::from_text(s);
    let evaluator = Evaluator::new(tokens).unwrap();
    let mut values = IndexMap::<char,bool>::new();
    values.insert('P', false);
    values.insert('Q', true);
    let mut result = evaluator.evaluate(&values).unwrap();
    for v in values.iter().rev() {
        result.insert_before(0, (*v.0).into(), *v.1);
    }
    let result = evaluator_result::EvaluatorResult{result:vec![result]};
    println!("{}",result);

}
