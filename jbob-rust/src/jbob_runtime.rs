use sexpr_parser::SexprFactory;
use std::cell::{Cell, RefCell};

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
    pairs: SegmentStorage<P<'a>>,
    symbols: RefCell<Vec<String>>,
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Context {
            pairs: SegmentStorage::new(),
            symbols: RefCell::new(vec![]),
        }
    }

    pub fn cons(&self, a: S<'a>, b: S<'a>) -> S<'a> {
        let p = self.pairs.alloc();
        p.0 = a;
        p.1 = b;
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

const SEGMENT_SIZE: usize = 4000;

struct SegmentStorage<T> {
    segments: RefCell<Vec<Box<[T; SEGMENT_SIZE]>>>,
    next_free_item: Cell<usize>,
}

impl<T: Default + Copy> SegmentStorage<T> {
    pub fn new() -> Self {
        SegmentStorage {
            segments: RefCell::new(vec![]),
            next_free_item: Cell::new(SEGMENT_SIZE + 1),
        }
    }

    pub fn alloc(&self) -> &mut T {
        if self.next_free_item.get() >= SEGMENT_SIZE {
            println!("Allocating new segment");
            self.segments
                .borrow_mut()
                .push(Box::new([T::default(); SEGMENT_SIZE]));
            self.next_free_item.set(0);
        }

        let k = self.next_free_item.get();
        self.next_free_item.set(k + 1);

        let mut segments = self.segments.borrow_mut();

        unsafe { &mut *segments.last_mut().unwrap().as_mut_ptr().add(k) }
    }
}

impl<'a> SexprFactory for Context<'a> {
    type Sexpr = S<'a>;
    type Integer = i64;
    type Float = f64;

    fn int(&mut self, x: i64) -> Self::Sexpr {
        S::Num(x)
    }

    fn float(&mut self, x: f64) -> Self::Sexpr {
        unimplemented!()
    }

    fn symbol(&mut self, x: &str) -> Self::Sexpr {
        self.intern_symbol(x)
    }

    fn string(&mut self, x: &str) -> Self::Sexpr {
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

pub fn num<'a>(_: &'a Context<'a>, x: S<'a>) -> S<'a> {
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

pub fn atom<'a>(_: &'a Context<'a>, x: S<'a>) -> S<'a> {
    match x {
        S::Pair(_) => C_NIL,
        _ => C_T,
    }
}

pub fn cons<'a>(context: &'a Context<'a>, a: S<'a>, b: S<'a>) -> S<'a> {
    context.cons(a, b)
}

pub fn car<'a>(_: &'a Context<'a>, p: S<'a>) -> S<'a> {
    match p {
        S::Pair(&(a, _)) => a,
        _ => S::Empty,
    }
}

pub fn cdr<'a>(_: &'a Context<'a>, p: S<'a>) -> S<'a> {
    match p {
        S::Pair(&(_, d)) => d,
        _ => S::Empty,
    }
}

pub fn equal<'a>(_: &'a Context<'a>, a: S<'a>, b: S<'a>) -> S<'a> {
    if a == b {
        C_T
    } else {
        C_NIL
    }
}

pub fn natp<'a>(_: &'a Context<'a>, x: S<'a>) -> S<'a> {
    match x {
        S::Num(n) if n >= 0 => C_T,
        _ => C_NIL,
    }
}

pub fn _plus<'a>(context: &'a Context<'a>, a: S<'a>, b: S<'a>) -> S<'a> {
    return S::Num(raw_num(a) + raw_num(b));
}

pub fn _lt<'a>(context: &'a Context<'a>, a: S<'a>, b: S<'a>) -> S<'a> {
    if raw_num(a) < raw_num(b) {
        C_T
    } else {
        C_NIL
    }
}

pub fn size<'a>(context: &'a Context<'a>, x: S<'a>) -> S<'a> {
    todo!()
}
