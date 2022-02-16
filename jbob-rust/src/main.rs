pub mod j_bob;
mod jbob_runtime;

use jbob_runtime::{Context, S};

macro_rules! sx {
    (()) => { S::Empty };
    (($first:tt $($rest:tt)*)) => {{
        const A: S = sx![$first];
        const D: S = sx![($($rest)*)];
        S::Pair(&(A, D))
    }};
    ($x:expr) => { S::Symbol($x) };
}

fn main() {
    let ctx = Context::new();

    let expr = sx![("car" ("cons" ("quote" "ham") ("quote" ("cheese"))))];
    let steps = sx![((() ("car/cons" ("quote" "ham") ("quote" ("cheese")))))];

    println!("{:?}", expr);
    println!("{:?}", steps);
    for _ in 0..10 {
        println!(
            "{:?}",
            j_bob::j_bob_slash_step(&ctx, j_bob::prelude(&ctx), expr, steps)
        );
    }
}
