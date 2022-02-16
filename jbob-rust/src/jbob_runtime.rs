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
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Context {
            pairs: SegmentStorage::new(),
        }
    }

    pub fn cons(&'a self, a: S<'a>, b: S<'a>) -> S<'a> {
        let p = self.pairs.alloc();
        p.0 = a;
        p.1 = b;
        S::Pair(p)
    }
}

const SEGMENT_SIZE: usize = 4000;

struct SegmentStorage<T> {
    segments: RefCell<Vec<[T; SEGMENT_SIZE]>>,
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
                .push([T::default(); SEGMENT_SIZE]);
            self.next_free_item.set(0);
        }

        let k = self.next_free_item.get();
        self.next_free_item.set(k + 1);

        let mut segments = self.segments.borrow_mut();

        unsafe { &mut *segments.last_mut().unwrap().as_mut_ptr().add(k) }
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
