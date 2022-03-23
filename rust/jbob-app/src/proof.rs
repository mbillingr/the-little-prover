use crate::sexpr_adapter::pretty_to_s;
use crate::PrettyExpr;
use jbob::j_bob::{j_bob_slash_define, j_bob_slash_prove};
use jbob::jbob_runtime::{Context, S};

pub fn proof<T>(
    defs: &PrettyExpr<T>,
    statement: &PrettyExpr<T>,
    seed: &PrettyExpr<T>,
    steps: &[(PrettyExpr<T>, PrettyExpr<T>)],
) -> PrettyExpr<T> {
    let ctx = &mut Context::new();
    let (defs, pfs) = prepare_args(ctx, defs, statement, seed, steps);
    let result = j_bob_slash_prove(ctx, defs, pfs);
    result.into()
}

pub fn define<T>(
    defs: &PrettyExpr<T>,
    statement: &PrettyExpr<T>,
    seed: &PrettyExpr<T>,
    steps: &[(PrettyExpr<T>, PrettyExpr<T>)],
) -> (PrettyExpr<T>, PrettyExpr<T>) {
    let ctx = &mut Context::new();
    let (defs, pfs) = prepare_args(ctx, defs, statement, seed, steps);

    let result = j_bob_slash_define(ctx, defs, pfs);
    (result.into(), pfs.into())
}

fn prepare_args<'a, T>(
    ctx: &mut Context<'a>,
    defs: &PrettyExpr<T>,
    statement: &PrettyExpr<T>,
    seed: &PrettyExpr<T>,
    steps: &[(PrettyExpr<T>, PrettyExpr<T>)],
) -> (S<'a>, S<'a>) {
    let statement = pretty_to_s(ctx, statement);
    let seed = pretty_to_s(ctx, seed);

    let mut pf = S::Empty;
    for (path, rewrite) in steps.iter().rev() {
        let path = pretty_to_s(ctx, path);
        let rewrite = pretty_to_s(ctx, rewrite);
        let step = ctx.cons(path, ctx.cons(rewrite, S::Empty));
        pf = ctx.cons(step, pf);
    }

    let pf = ctx.cons(statement, ctx.cons(seed, pf));
    let pfs = ctx.cons(pf, S::Empty);

    let defs = pretty_to_s(ctx, defs);

    (defs, pfs)
}
