#[macro_use]
pub mod jbob_runtime;
pub mod j_bob;

use crate::jbob_runtime::S;
use jbob_runtime::Context;
use sexpr_parser::Parser;
use std::collections::HashMap;
use std::fs;

fn main() {}

#[test]
fn test_recess() {
    let src = fs::read_to_string("../test_recess.scm").unwrap();
    test_runner(&src);
}

fn test_runner(src: &str) {
    let mut ctx = Context::new();

    let mut env = HashMap::new();

    let testsuite = ctx.parse_seq(src).unwrap();
    run_testsuite(&ctx, &mut env, testsuite);
}

fn run_testsuite<'a>(ctx: &'a Context<'a>, env: &mut HashMap<&'a str, S<'a>>, mut exprs: S<'a>) {
    while let S::Pair(&(expr, rest)) = exprs {
        match expr {
            S::Pair((S::Symbol("testcase"), S::Pair(&(name, S::Pair(&(body, S::Empty)))))) => {
                run_testcase(ctx, env, name, body)
            }
            S::Pair((
                S::Symbol("define"),
                S::Pair((S::Symbol(name), S::Pair(&(value, S::Empty)))),
            )) => {
                let value = evaluate_expr(ctx, env, value);
                assert!(env.insert(name, value).is_none());
            }
            _ => todo!("{:?}", expr),
        }
        exprs = rest;
    }
}

fn run_testcase<'a>(ctx: &'a Context<'a>, env: &HashMap<&'a str, S<'a>>, name: S<'a>, body: S<'a>) {
    match body {
        S::Pair((S::Symbol("assert-equal"), S::Pair(&(expr, S::Pair(&(expect, S::Empty)))))) => {
            let actual_value = evaluate_expr(ctx, env, expr);
            let expected_value = evaluate_expr(ctx, env, expect);
            print!("{:?} ... ", name);
            assert_eq!(actual_value, expected_value);
            println!("OK");
        }
        _ => todo!("{:?}", body),
    }
}

fn evaluate_expr<'a>(ctx: &'a Context<'a>, env: &HashMap<&'a str, S<'a>>, expr: S<'a>) -> S<'a> {
    match expr {
        S::Pair((S::Symbol("prelude"), S::Empty)) => j_bob::prelude(ctx),
        S::Pair((
            S::Symbol("J-Bob/step"),
            S::Pair(&(defs, S::Pair(&(e, S::Pair(&(steps, S::Empty)))))),
        )) => {
            let defs = evaluate_expr(ctx, env, defs);
            let e = evaluate_expr(ctx, env, e);
            let steps = evaluate_expr(ctx, env, steps);
            j_bob::j_bob_slash_step(ctx, defs, e, steps)
        }
        S::Pair((S::Symbol("J-Bob/prove"), S::Pair(&(defs, S::Pair(&(pfs, S::Empty)))))) => {
            let defs = evaluate_expr(ctx, env, defs);
            let pfs = evaluate_expr(ctx, env, pfs);
            j_bob::j_bob_slash_prove(ctx, defs, pfs)
        }
        S::Pair((S::Symbol("J-Bob/define"), S::Pair(&(defs, S::Pair(&(pfs, S::Empty)))))) => {
            let defs = evaluate_expr(ctx, env, defs);
            let pfs = evaluate_expr(ctx, env, pfs);
            j_bob::j_bob_slash_define(ctx, defs, pfs)
        }
        S::Pair(&(S::Symbol("quote"), S::Pair(&(value, S::Empty)))) => value,
        S::Symbol(name) => env[name],
        _ => todo!("{:?}", expr),
    }
}
