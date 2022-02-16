pub mod j_bob;
pub mod jbob_runtime;

use crate::jbob_runtime::S;
use jbob_runtime::Context;
use sexpr_parser::Parser;
use std::fs;

fn main() {
    let src = fs::read_to_string("../test_recess.scm").unwrap();
    test_runner(&src);
}

fn test_runner(src: &str) {
    let mut ctx = Context::new();

    let testsuite = ctx.parse_seq(src).unwrap();
    let executor = analyze_testsuite(testsuite);
    executor(&ctx)
}

fn analyze_testsuite<'a>(mut exprs: S<'a>) -> impl Fn(&'a Context<'a>) + 'a {
    let mut execs = vec![];
    while let S::Pair(&(expr, rest)) = exprs {
        execs.push(match expr {
            S::Pair((S::Symbol("testcase"), S::Pair(&(name, S::Pair(&(body, S::Empty)))))) => {
                analyze_testcase(name, body)
            }
            _ => todo!("{:?}", expr),
        });
        exprs = rest;
    }
    move |ctx: &'a Context<'a>| {
        for ex in &execs {
            ex(ctx)
        }
    }
}

fn analyze_testcase<'a>(name: S<'a>, body: S<'a>) -> impl Fn(&'a Context<'a>) + 'a {
    match body {
        S::Pair((S::Symbol("assert-equal"), S::Pair(&(expr, S::Pair(&(expect, S::Empty)))))) => {
            let actual_exec = analyze_expr(expr);
            let expect_exec = analyze_expr(expect);
            move |ctx: &'a Context<'a>| {
                print!("{:?} ... ", name);
                assert_eq!(actual_exec(ctx), expect_exec(ctx));
                println!("OK");
            }
        }
        _ => todo!("{:?}", body),
    }
}

fn analyze_expr<'a>(expr: S<'a>) -> Box<dyn 'a + Fn(&'a Context<'a>) -> S<'a>> {
    match expr {
        S::Pair((S::Symbol("prelude"), S::Empty)) => {
            Box::new(|ctx: &'a Context<'a>| j_bob::prelude(ctx))
        }
        S::Pair((
            S::Symbol("J-Bob/step"),
            S::Pair(&(defs, S::Pair(&(e, S::Pair(&(steps, S::Empty)))))),
        )) => {
            let defs = analyze_expr(defs);
            let e = analyze_expr(e);
            let steps = analyze_expr(steps);
            Box::new(move |ctx: &'a Context<'a>| {
                j_bob::j_bob_slash_step(ctx, defs(ctx), e(ctx), steps(ctx))
            })
        }
        S::Pair(&(S::Symbol("quote"), S::Pair(&(value, S::Empty)))) => {
            Box::new(move |_: &'a Context<'a>| value)
        }
        _ => todo!("{:?}", expr),
    }
}
