pub use sexpr_parser::{Parser, SexprFactory};
use std::cell::RefCell;

macro_rules! cons {
    ($car:expr, $cdr:expr) => {
        S::Pair(&($car, $cdr))
    };

    ($car:expr, $cadr:expr, $($rest:expr),+) => {
        cons!($car, cons!($cadr, $($rest),+))
    };
}

#[derive(Copy, Clone, PartialEq)]
pub enum S<'a> {
    Empty,
    Num(i64),
    Symbol(&'a str),
    Pair(&'a P<'a>),
}

type P<'a> = (S<'a>, S<'a>);

impl Default for S<'_> {
    fn default() -> Self {
        NULL
    }
}

impl From<&'static str> for S<'static> {
    fn from(s: &'static str) -> Self {
        S::Symbol(s)
    }
}

pub const NULL: S<'static> = S::Symbol("NULL");
pub const C_NIL: S<'static> = S::Symbol("nil");
pub const C_T: S<'static> = S::Symbol("t");

impl std::fmt::Debug for S<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            S::Empty => write!(f, "()"),
            S::Num(x) => write!(f, "{}", x),
            S::Symbol(s) => write!(f, "{}", s),
            S::Pair(&(S::Symbol("quote"), S::Pair(&(x, S::Empty)))) => write!(f, "'{:?}", x),
            S::Pair(&(a, mut p)) => {
                write!(f, "({:?}", a)?;
                while let S::Pair(&(a, d)) = p {
                    write!(f, " {:?}", a)?;
                    p = d;
                }
                if let S::Empty = p {
                } else {
                    write!(f, " . {:?}", p)?;
                }
                write!(f, ")")
            }
        }
    }
}

pub struct Context<'a> {
    pairs: RefCell<SegmentStorage<P<'a>>>,
    symbols: RefCell<Vec<String>>,
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Context {
            pairs: RefCell::new(SegmentStorage::new(4000)),
            symbols: RefCell::new(vec![]),
        }
    }

    pub fn cons(&self, a: S<'a>, b: S<'a>) -> S<'a> {
        let mut pair_storage = self.pairs.borrow_mut();
        let p = pair_storage.alloc((a, b));
        let p = unsafe { &*(p as *const _) };
        S::Pair(&*p)
    }

    pub fn intern_symbol(&self, name: &str) -> S<'a> {
        for s in &*self.symbols.borrow() {
            if s == name {
                let s = unsafe { &*(s.as_str() as *const _) };
                return S::Symbol(s);
            }
        }

        let mut syms = self.symbols.borrow_mut();

        syms.push(name.to_string());
        let s = syms.last().unwrap();
        let s = unsafe { &*(s.as_str() as *const _) };
        S::Symbol(s)
    }
}

struct SegmentStorage<T> {
    current_segment: Vec<T>,
    segments: Vec<Vec<T>>,
}

impl<T: Default + Copy> SegmentStorage<T> {
    pub fn new(segment_size: usize) -> Self {
        SegmentStorage {
            current_segment: Vec::with_capacity(segment_size),
            segments: vec![],
        }
    }

    pub fn alloc(&mut self, value: T) -> &mut T {
        let segment_size = self.current_segment.capacity();
        if self.current_segment.len() >= segment_size {
            eprintln!("Allocating new segment");
            let new_segment = Vec::with_capacity(segment_size);
            let old_segment = std::mem::replace(&mut self.current_segment, new_segment);
            self.segments.push(old_segment);
        }

        self.current_segment.push(value);
        self.current_segment.last_mut().unwrap()
    }
}

impl<'a> SexprFactory for Context<'a> {
    type Sexpr = S<'a>;
    type Integer = i64;
    type Float = f64;

    fn int(&mut self, x: i64) -> Self::Sexpr {
        S::Num(x)
    }

    fn float(&mut self, _: f64) -> Self::Sexpr {
        unimplemented!()
    }

    fn symbol(&mut self, x: &str) -> Self::Sexpr {
        self.intern_symbol(x)
    }

    fn string(&mut self, _: &str) -> Self::Sexpr {
        unimplemented!()
    }

    fn list(&mut self, x: Vec<Self::Sexpr>) -> Self::Sexpr {
        let mut tail = S::Empty;
        for item in x.into_iter().rev() {
            tail = self.pair(item, tail)
        }
        tail
    }

    fn pair(&mut self, a: Self::Sexpr, b: Self::Sexpr) -> Self::Sexpr {
        self.cons(a, b)
    }
}

// predefined functions

pub fn num<'a>(_: &Context<'a>, x: S<'a>) -> S<'a> {
    match x {
        S::Num(_) => x,
        _ => S::Num(0),
    }
}

pub fn raw_num(x: S) -> i64 {
    match x {
        S::Num(n) => n,
        _ => 0,
    }
}

pub fn atom<'a>(_: &Context<'a>, x: S<'a>) -> S<'a> {
    match x {
        S::Pair(_) => C_NIL,
        _ => C_T,
    }
}

pub fn cons<'a>(context: &Context<'a>, a: S<'a>, b: S<'a>) -> S<'a> {
    context.cons(a, b)
}

pub fn car<'a>(_: &Context<'a>, p: S<'a>) -> S<'a> {
    match p {
        S::Pair(&(a, _)) => a,
        _ => S::Empty,
    }
}

pub fn cdr<'a>(_: &Context<'a>, p: S<'a>) -> S<'a> {
    match p {
        S::Pair(&(_, d)) => d,
        _ => S::Empty,
    }
}

pub fn equal<'a>(_: &Context<'a>, a: S<'a>, b: S<'a>) -> S<'a> {
    if a == b {
        C_T
    } else {
        C_NIL
    }
}

pub fn natp<'a>(_: &Context<'a>, x: S<'a>) -> S<'a> {
    match x {
        S::Num(n) if n >= 0 => C_T,
        _ => C_NIL,
    }
}

pub fn _plus<'a>(_: &Context<'a>, a: S<'a>, b: S<'a>) -> S<'a> {
    return S::Num(raw_num(a) + raw_num(b));
}

pub fn _lt<'a>(_: &Context<'a>, a: S<'a>, b: S<'a>) -> S<'a> {
    if raw_num(a) < raw_num(b) {
        C_T
    } else {
        C_NIL
    }
}

pub fn size<'a>(_: &Context<'a>, x: S<'a>) -> S<'a> {
    S::Num(raw_size(x))
}

pub fn raw_size(x: S) -> i64 {
    match x {
        S::Pair(&(car, cdr)) => 1 + raw_size(car) + raw_size(cdr),
        _ => 0,
    }
}
