# Propositional logic evalutaor written in rust.
Supported operators and symbols:
- Not: `not`, `¬`, `!`, `∼`
- And: `and`, `&`, `&&`, `∧`
- Or: `or`, `|`, `||`, `∨`
- XOr: `xor`, `⊕`
- Implication: `->`, `=>`, `⇒`, `→`, `⊃`
- Biconditional: `<->`, `<=>`, `⇔`, `↔`, `iff`, `xnor`
- Logical equivalence: `=`, `==`, `eq`, `≡`
- Not equal: `!=`, `≠`
- parentheses: `()`, `{}`, `[]`

Note: `Logical equivalence` means there is a proof for `Biconditional` expression. for example:</br>
$(P↔Q) ≡ (P→Q)∧(Q→P)$ means you can prove $(P↔Q) ↔ (P→Q)∧(Q→P)$ or prove both $(P↔Q)→(P→Q)∧(Q→P)$ and $(P→Q)∧(Q→P)→(P↔Q)$. Therefore, the truth table for `≡` results in all true. but for `↔`, all result to true if it is proofable.</br>

Note: the order of evaluation of operators are: `not`, `and`, `or`, `xor`, `→`, `↔`, `≡` and `≠`. Please use parentheses inorder to adjust the operator priorities. incorrect expressions result in wrong tables.</br>

Note: Characters are case sensetive. for example `q` is not the same as `Q`.

Example: </br>
- Truth table for `P ∨ (Q ∧ R) ≡ (P ∨ Q) ∧ (P ∨ R)` or `P or (Q and R) == (P or Q) and (P or R)`
![truth table 1](files/tt1.png)

- Truth table for `(P ∧ (∼ Q)) ⇔ (P ⇒ Q)`
![truth table 2](files/tt2.png)

# How to run
1. install `rust`. see [here](https://www.rust-lang.org/tools/install) for the instructions.
2. run `cargo run` to run the app.
3. run `cargo test` to run the tests.