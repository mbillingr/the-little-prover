use jbob::jbob_runtime::S;

struct Proof<T> {
    statement: T,
    seed: T,
    steps: Vec<ProofStep<T>>,
}

struct ProofStep<T> {
    path: T,
    rewrite: T,
}