use crate::PrettyExpr;

#[derive(Clone)]
pub struct ProofView {
    statement: PrettyExpr,
    seed: PrettyExpr,
}
