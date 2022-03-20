use crate::sexpr_adapter::pretty_to_s;
use crate::PrettyExpr;
use jbob::j_bob::j_bob_slash_prove;
use jbob::jbob_runtime::Context;

pub fn proof<T>(defs: &PrettyExpr<T>, pfs: &PrettyExpr<T>) -> PrettyExpr<T> {
    let ctx = &mut Context::new();
    let defs = pretty_to_s(ctx, defs);
    let pfs = pretty_to_s(ctx, pfs);

    let result = j_bob_slash_prove(ctx, defs, pfs);

    result.into()
}
