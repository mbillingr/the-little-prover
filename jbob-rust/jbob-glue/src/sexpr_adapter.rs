use crate::PrettyExpr;
use jbob_rs::jbob_runtime::{Context, SexprFactory, S};

impl<T> From<S<'_>> for PrettyExpr<T> {
    fn from(expr: S) -> Self {
        match expr {
            S::Empty => PrettyExpr::empty_list(),
            S::Num(n) => PrettyExpr::Atom(n.to_string()),
            S::Symbol(s) => PrettyExpr::Atom(s.to_string()),
            S::Pair(&(S::Symbol("quote"), S::Pair(&(x, S::Empty)))) => {
                PrettyExpr::Quote(Box::new(x.into()))
            }
            S::Pair(_) => {
                let mut list = vec![];
                let mut p = expr;
                while let S::Pair(&(car, cdr)) = p {
                    list.push(car.into());
                    p = cdr;
                }
                assert_eq!(p, S::Empty);
                PrettyExpr::list(list)
            }
        }
    }
}

pub fn pretty_to_s<'a, T>(context: &mut Context<'a>, expr: &PrettyExpr<T>) -> S<'a> {
    match expr {
        PrettyExpr::Atom(s) => match s.parse() {
            Ok(n) => context.int(n),
            Err(_) => context.symbol(s),
        },
        PrettyExpr::Stat(s) => match s.parse() {
            Ok(n) => context.int(n),
            Err(_) => S::Symbol(s),
        },
        PrettyExpr::Quote(x) => {
            let x = pretty_to_s(context, x);
            context.cons(S::Symbol("quote"), context.cons(x, S::Empty))
        }
        PrettyExpr::Inline(xs) | PrettyExpr::Expand(xs) | PrettyExpr::SemiExpand(_, xs) => {
            let xs = xs.iter().map(|x| pretty_to_s(context, x)).collect();
            context.list(xs)
        }
        PrettyExpr::Style(_, x) => pretty_to_s(context, x),
    }
}
