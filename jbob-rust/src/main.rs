pub mod j_bob;
pub mod jbob_runtime;

use crate::jbob_runtime::S;
use jbob_runtime::Context;
use sexpr_parser::Parser;
use std::fs;

fn main() {}

#[test]
fn test_recess() {
    let src = fs::read_to_string("../test_recess.scm").unwrap();
    test_runner(&src);
}

fn test_runner(src: &str) {
    let mut ctx = Context::new();

    let testsuite = ctx.parse_seq(src).unwrap();
    let executor = run_testsuite(&ctx, testsuite);
}

fn run_testsuite<'a>(ctx: &'a Context<'a>, mut exprs: S<'a>) {
    while let S::Pair(&(expr, rest)) = exprs {
        match expr {
            S::Pair((S::Symbol("testcase"), S::Pair(&(name, S::Pair(&(body, S::Empty)))))) => {
                run_testcase(ctx, name, body)
            }
            _ => todo!("{:?}", expr),
        }
        exprs = rest;
    }
}

fn run_testcase<'a>(ctx: &'a Context<'a>, name: S<'a>, body: S<'a>) {
    match body {
        S::Pair((S::Symbol("assert-equal"), S::Pair(&(expr, S::Pair(&(expect, S::Empty)))))) => {
            let actual_value = evaluate_expr(ctx, expr);
            let expected_value = evaluate_expr(ctx, expect);
            print!("{:?} ... ", name);
            assert_eq!(actual_value, expected_value);
            println!("OK");
        }
        _ => todo!("{:?}", body),
    }
}

fn evaluate_expr<'a>(ctx: &'a Context<'a>, expr: S<'a>) -> S<'a> {
    match expr {
        S::Pair((S::Symbol("prelude"), S::Empty)) => {
            j_bob::prelude(ctx)
        }
        S::Pair((
            S::Symbol("J-Bob/step"),
            S::Pair(&(defs, S::Pair(&(e, S::Pair(&(steps, S::Empty)))))),
        )) => {
            let defs = evaluate_expr(ctx,defs);
            let e = evaluate_expr(ctx,e);
            let steps = evaluate_expr(ctx,steps);
            j_bob::j_bob_slash_step(ctx, defs, e, steps)
        }
        S::Pair((S::Symbol("J-Bob/prove"), S::Pair(&(defs, S::Pair(&(pfs, S::Empty)))))) => {
            let defs = evaluate_expr(ctx,defs);
            let pfs = evaluate_expr(ctx,pfs);
            j_bob::j_bob_slash_prove(ctx, defs, pfs)
        }
        S::Pair((S::Symbol("J-Bob/define"), S::Pair(&(defs, S::Pair(&(pfs, S::Empty)))))) => {
            let defs = evaluate_expr(ctx,defs);
            let pfs = evaluate_expr(ctx,pfs);
            j_bob::j_bob_slash_define(ctx, defs, pfs)
        }
        S::Pair(&(S::Symbol("quote"), S::Pair(&(value, S::Empty)))) => {
            value
        }
        _ => todo!("{:?}", expr),
    }
}
