pub mod j_bob;
mod jbob_runtime;

use jbob_runtime::{Context, S};
use sexpr_parser::Parser;

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
    let mut ctx = Context::new();
    let expr = ctx.parse("(car (cons 'ham '(cheese)))").unwrap();
    let steps = ctx.parse("((() (car/cons 'ham '(cheese))))").unwrap();

    println!("{:?}", expr);
    println!("{:?}", steps);

    for _ in 0..10 {
        let defs = j_bob::prelude(&ctx);
        println!("{:?}", j_bob::j_bob_slash_step(&ctx, defs, expr, steps));
    }
}
