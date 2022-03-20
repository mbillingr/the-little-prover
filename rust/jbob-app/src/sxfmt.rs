//! S-Expression pretty-printing

use std::collections::VecDeque;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub enum PrettyExpr<T = ()> {
    Atom(String),
    Stat(&'static str),
    Quote(Box<PrettyExpr<T>>),
    Inline(Vec<PrettyExpr<T>>),
    Expand(Vec<PrettyExpr<T>>),
    SemiExpand(u8, Vec<PrettyExpr<T>>),
    Style(T, Box<PrettyExpr<T>>),
}

impl<T> PrettyExpr<T> {
    pub fn empty_list() -> Self {
        PrettyExpr::Inline(vec![])
    }
    pub fn list(xs: Vec<PrettyExpr<T>>) -> Self {
        PrettyExpr::Inline(xs)
    }
    pub fn quote(x: PrettyExpr<T>) -> Self {
        PrettyExpr::Quote(Box::new(x))
    }

    pub fn styled(style: impl Into<T>, exp: impl Into<PrettyExpr<T>>) -> Self {
        PrettyExpr::Style(style.into(), Box::new(exp.into()))
    }

    pub fn with_style(self, path: &[usize], style: impl Into<T>) -> Option<Self> {
        use PrettyExpr::*;
        match (path, self) {
            (_, Style(s, x)) => Some(Self::styled(s, x.with_style(path, style)?)),
            ([], x) => Some(Self::styled(style, x)),
            ([p, rest @ ..], Inline(xs)) => Self::list_with_style(xs, *p, rest, style).map(Inline),
            ([p, rest @ ..], Expand(xs)) => Self::list_with_style(xs, *p, rest, style).map(Expand),
            ([p, rest @ ..], SemiExpand(n, xs)) => {
                Self::list_with_style(xs, *p, rest, style).map(|xs| SemiExpand(n, xs))
            }
            ([_, rest @ ..], Quote(x)) => x.with_style(rest, style).map(Self::quote),
            (_, Atom(_) | Stat(_)) => None,
        }
    }

    pub fn is_valid_path(&self, path: &[usize]) -> bool {
        self.get(path).is_some()
    }

    pub fn path_to_last_element(&self) -> VecDeque<usize> {
        use PrettyExpr::*;
        match self {
            Atom(_) | Stat(_) => VecDeque::new(),
            Quote(x) => {
                let mut path = x.path_to_last_element();
                path.push_front(0);
                path
            }
            Inline(xs) | Expand(xs) | SemiExpand(_, xs) => match xs.last() {
                None => VecDeque::new(),
                Some(x) => {
                    let mut path = x.path_to_last_element();
                    path.push_front(xs.len() - 1);
                    path
                }
            },
            Style(_, x) => x.path_to_last_element(),
        }
    }

    pub fn get(&self, path: &[usize]) -> Option<&Self> {
        use PrettyExpr::*;
        match (path, self) {
            (_, Style(_, x)) => x.get(path),
            ([], x) => Some(x),
            ([0, rest @ ..], Quote(x)) => x.get(rest),
            (_, Quote(_)) => None,
            ([p, rest @ ..], Inline(xs) | Expand(xs) | SemiExpand(_, xs)) => {
                xs.get(*p).and_then(|x| x.get(rest))
            }
            (_, Atom(_) | Stat(_)) => None,
        }
    }

    pub fn get_mut(&mut self, path: &[usize]) -> Option<&mut Self> {
        use PrettyExpr::*;
        match (path, self) {
            (_, Style(_, x)) => x.get_mut(path),
            ([], x) => Some(x),
            ([0, rest @ ..], Quote(x)) => x.get_mut(rest),
            (_, Quote(_)) => None,
            ([p, rest @ ..], Inline(xs) | Expand(xs) | SemiExpand(_, xs)) => {
                xs.get_mut(*p).and_then(|x| x.get_mut(rest))
            }
            (_, Atom(_) | Stat(_)) => None,
        }
    }

    fn list_with_style(
        xs: Vec<Self>,
        p: usize,
        rest_path: &[usize],
        style: impl Into<T>,
    ) -> Option<Vec<Self>> {
        let mut out = Vec::with_capacity(xs.len());
        let mut xs = xs.into_iter();

        for _ in 0..p {
            out.push(xs.next()?);
        }

        out.push(xs.next()?.with_style(rest_path, style)?);

        out.extend(xs);
        Some(out)
    }

    pub fn is_atom(&self) -> bool {
        match self {
            PrettyExpr::Atom(_) | PrettyExpr::Stat(_) => true,
            PrettyExpr::Quote(_) => true,
            PrettyExpr::Inline(_) | PrettyExpr::Expand(_) | PrettyExpr::SemiExpand(_, _) => false,
            PrettyExpr::Style(_, x) => x.is_atom(),
        }
    }

    pub fn is_quotation(&self) -> bool {
        self.quoted_value().is_some()
    }

    pub fn is_empty_list(&self) -> bool {
        match self {
            PrettyExpr::Atom(_) | PrettyExpr::Stat(_) => false,
            PrettyExpr::Quote(_) => false,
            PrettyExpr::Inline(xs) | PrettyExpr::Expand(xs) | PrettyExpr::SemiExpand(_, xs) => {
                xs.is_empty()
            }
            PrettyExpr::Style(_, x) => x.is_empty_list(),
        }
    }

    pub fn get_text(&self) -> Option<&str> {
        match self {
            PrettyExpr::Atom(s) => Some(s),
            PrettyExpr::Stat(s) => Some(s),
            PrettyExpr::Quote(_) => None,
            PrettyExpr::Inline(xs) | PrettyExpr::Expand(xs) | PrettyExpr::SemiExpand(_, xs)
                if xs.is_empty() =>
            {
                Some("")
            }
            PrettyExpr::Inline(_) | PrettyExpr::Expand(_) | PrettyExpr::SemiExpand(_, _) => None,
            PrettyExpr::Style(_, x) => x.get_text(),
        }
    }

    pub fn quoted_value(&self) -> Option<&Self> {
        match self {
            PrettyExpr::Atom(_) | PrettyExpr::Stat(_) => None,
            PrettyExpr::Quote(x) => Some(x),
            PrettyExpr::Inline(_) | PrettyExpr::Expand(_) | PrettyExpr::SemiExpand(_, _) => None,
            PrettyExpr::Style(_, x) => x.quoted_value(),
        }
    }

    pub fn elements(&self) -> Option<&[Self]> {
        match self {
            PrettyExpr::Atom(_) | PrettyExpr::Stat(_) => None,
            PrettyExpr::Quote(_) => None,
            PrettyExpr::Inline(xs) | PrettyExpr::Expand(xs) | PrettyExpr::SemiExpand(_, xs) => {
                Some(xs.as_slice())
            }
            PrettyExpr::Style(_, x) => x.elements(),
        }
    }

    pub fn elements_mut(&mut self) -> Option<&mut Vec<Self>> {
        match self {
            PrettyExpr::Atom(_) | PrettyExpr::Stat(_) => None,
            PrettyExpr::Quote(_) => None,
            PrettyExpr::Inline(xs) | PrettyExpr::Expand(xs) | PrettyExpr::SemiExpand(_, xs) => {
                Some(xs)
            }
            PrettyExpr::Style(_, x) => x.elements_mut(),
        }
    }

    pub fn remove_item(&mut self, idx: usize) -> Option<Self> {
        match self {
            PrettyExpr::Atom(_) | PrettyExpr::Stat(_) => None,
            PrettyExpr::Quote(x) => Some(std::mem::replace(x, PrettyExpr::list(vec![]))),
            PrettyExpr::Inline(xs) | PrettyExpr::Expand(xs) | PrettyExpr::SemiExpand(_, xs) => {
                Some(xs.remove(idx))
            }
            PrettyExpr::Style(_, x) => x.remove_item(idx),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            PrettyExpr::Atom(_) | PrettyExpr::Stat(_) => 0,
            PrettyExpr::Quote(_) => 1,
            PrettyExpr::Inline(xs) | PrettyExpr::Expand(xs) | PrettyExpr::SemiExpand(_, xs) => {
                xs.len()
            }
            PrettyExpr::Style(_, x) => x.len(),
        }
    }

    fn inline_width(&self) -> Option<usize> {
        match self {
            PrettyExpr::Atom(x) => Some(x.len()),
            PrettyExpr::Stat(x) => Some(x.len()),
            PrettyExpr::Quote(x) => Some(1 + x.inline_width()?),
            PrettyExpr::Inline(xs) => Some(1 + Self::partial_width(xs)?),
            PrettyExpr::Style(_, x) => x.inline_width(),
            PrettyExpr::Expand(_) | PrettyExpr::SemiExpand(_, _) => None,
        }
    }

    fn partial_width(xs: &[PrettyExpr<T>]) -> Option<usize> {
        let n_spaces = if xs.len() < 2 { 0 } else { xs.len() - 1 };
        let mut w = 1 + n_spaces;
        for x in xs {
            w += x.inline_width()?;
        }
        Some(w)
    }
}

#[derive(Copy, Clone)]
pub struct PrettyFormatter {
    pub max_code_width: usize,
    pub default_indent: usize,
}

impl Default for PrettyFormatter {
    fn default() -> Self {
        PrettyFormatter {
            max_code_width: 15,
            default_indent: 2,
        }
    }
}

pub struct Pretty<T> {
    pf: PrettyFormatter,
    pub pe: PrettyExpr<T>,
}

impl<T> Pretty<T> {
    pub fn write<F: Formatter<T>>(&self, f: &mut F) -> Result<(), F::Error> {
        self.pf.write(&self.pe, 0, f)
    }

    pub fn with_style(self, path: &[usize], style: impl Into<T>) -> Option<Self> {
        self.pe
            .with_style(path, style)
            .map(|pe| Pretty { pe, ..self })
    }
}

impl PrettyFormatter {
    pub fn new(max_code_width: usize, default_indent: usize) -> Self {
        PrettyFormatter {
            max_code_width,
            default_indent,
        }
    }

    pub fn prepare<T>(&self, pe: PrettyExpr<T>) -> PrettyExpr<T> {
        self.prepare_recursively(pe, 0)
    }

    fn prepare_recursively<T>(&self, pe: PrettyExpr<T>, current_indent: usize) -> PrettyExpr<T> {
        match pe {
            PrettyExpr::Atom(x) => PrettyExpr::Atom(x),
            PrettyExpr::Stat(x) => PrettyExpr::Stat(x),
            PrettyExpr::Inline(_)
                if current_indent + pe.inline_width().unwrap() <= self.max_code_width =>
            {
                pe
            }
            PrettyExpr::Inline(xs) | PrettyExpr::Expand(xs) | PrettyExpr::SemiExpand(_, xs) => {
                let indent = current_indent
                    + xs.first()
                        .map(|x0| self.compute_expand_indent(x0))
                        .unwrap_or(0);
                match self.try_semiexpand(xs, current_indent, indent) {
                    Ok(psx) => psx,
                    Err(xs) => PrettyExpr::Expand(
                        xs.into_iter()
                            .map(|x| self.prepare_recursively(x, indent))
                            .collect(),
                    ),
                }
            }
            PrettyExpr::Quote(x) => PrettyExpr::quote(self.prepare_recursively(*x, current_indent)),
            PrettyExpr::Style(s, x) => {
                PrettyExpr::styled(s, self.prepare_recursively(*x, current_indent))
            }
        }
    }

    pub fn try_semiexpand<T>(
        &self,
        xs: Vec<PrettyExpr<T>>,
        current_indent: usize,
        new_indent: usize,
    ) -> Result<PrettyExpr<T>, Vec<PrettyExpr<T>>> {
        if xs.is_empty() {
            return Err(xs);
        }

        let n_inline = match xs.first().and_then(PrettyExpr::get_text) {
            Some("defun") | Some("dethm") => 3,
            Some("if") => 2,
            Some(s) if s.len() == 1 => 2,
            _ => 1,
        };

        if PrettyExpr::partial_width(&xs[..n_inline])
            .map(|w| w + current_indent <= self.max_code_width)
            == Some(true)
        {
            let mut xs = xs.into_iter();
            let mut ys = vec![];
            for _ in 0..n_inline {
                ys.push(xs.next().unwrap())
            }
            ys.extend(xs.map(|x| self.prepare_recursively(x, new_indent)));
            Ok(PrettyExpr::SemiExpand(n_inline as u8, ys))
        } else {
            Err(xs)
        }
    }

    pub fn pretty<T>(&self, pe: PrettyExpr<T>) -> Pretty<T> {
        Pretty {
            pf: *self,
            pe: self.prepare(pe),
        }
    }

    fn write<T, F: Formatter<T>>(
        &self,
        pe: &PrettyExpr<T>,
        indent_level: usize,
        f: &mut F,
    ) -> Result<(), F::Error> {
        match pe {
            PrettyExpr::Atom(x) => f.write(x),
            PrettyExpr::Stat(x) => f.write(x),
            PrettyExpr::Quote(x) => {
                f.write("'")?;
                self.write(x, indent_level + 1, f)
            }
            PrettyExpr::Inline(xs) => self.write_inline(xs, f),
            PrettyExpr::Expand(xs) => self.write_expanded(xs, 1, indent_level, f),
            PrettyExpr::SemiExpand(n, xs) => self.write_expanded(xs, *n as usize, indent_level, f),
            PrettyExpr::Style(s, x) => {
                f.save_style();
                f.set_style(s);
                self.write(x, indent_level, f)?;
                f.restore_style();
                Ok(())
            }
        }
    }

    fn write_inline<T, F: Formatter<T>>(
        &self,
        xs: &[PrettyExpr<T>],
        f: &mut F,
    ) -> Result<(), F::Error> {
        f.write("(")?;
        match &xs[..] {
            [] => {}
            [x] => self.write(x, 0, f)?,
            [x, ys @ ..] => {
                self.write(x, 0, f)?;
                for y in ys {
                    f.write(" ")?;
                    self.write(y, 0, f)?;
                }
            }
        }
        f.write(")")
    }

    fn write_expanded<T, F: Formatter<T>>(
        &self,
        xs: &[PrettyExpr<T>],
        n_inline: usize,
        mut indent_level: usize,
        f: &mut F,
    ) -> Result<(), F::Error> {
        f.write("(")?;
        match &xs[..] {
            [] => {}
            [x] => {
                indent_level += self.compute_expand_indent(x);
                self.write(x, indent_level, f)?
            }
            [xs @ ..] => {
                indent_level += self.compute_expand_indent(xs.first().unwrap());
                self.write(xs.first().unwrap(), indent_level, f)?;
                for x in &xs[1..n_inline] {
                    f.write(" ")?;
                    self.write(x, indent_level, f)?;
                }
                for x in &xs[n_inline..] {
                    f.write_indent(indent_level)?;
                    self.write(x, indent_level, f)?;
                }
            }
        }
        f.write(")")
    }

    fn compute_expand_indent<T>(&self, first_item: &PrettyExpr<T>) -> usize {
        match first_item.get_text() {
            None => 1,
            Some(s) if s.len() == 2 => 4,
            Some(s) if s.len() == 1 => 3,
            Some(_) => self.default_indent,
        }
    }
}

impl<T: Clone> std::fmt::Display for PrettyExpr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let pf = PrettyFormatter::default();
        let pe = pf.prepare(self.clone());
        let mut df = DisplayFormatter::new(f);
        pf.write(&pe, 0, &mut df)
    }
}

impl<T: Clone> std::fmt::Display for Pretty<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let pe = self.pf.prepare(self.pe.clone());
        let mut df = DisplayFormatter::new(f);
        self.pf.write(&pe, 0, &mut df)
    }
}

pub trait Formatter<S> {
    type Error;

    fn write(&mut self, x: impl std::fmt::Display) -> std::result::Result<(), Self::Error>;
    fn set_style(&mut self, style: &S);
    fn save_style(&mut self);
    fn restore_style(&mut self);

    fn write_newline(&mut self) -> std::result::Result<(), Self::Error> {
        self.write("\n")
    }

    fn write_indent(&mut self, level: usize) -> std::result::Result<(), Self::Error> {
        self.write_newline()?;
        self.write(" ".repeat(level))
    }
}

struct DisplayFormatter<'a, 'b, S> {
    f: &'a mut std::fmt::Formatter<'b>,
    _p: PhantomData<S>,
}

impl<'a, 'b, S> DisplayFormatter<'a, 'b, S> {
    pub fn new(f: &'a mut std::fmt::Formatter<'b>) -> Self {
        DisplayFormatter { f, _p: PhantomData }
    }
}

impl<S> Formatter<S> for DisplayFormatter<'_, '_, S> {
    type Error = std::fmt::Error;

    fn write(&mut self, x: impl std::fmt::Display) -> std::result::Result<(), Self::Error> {
        write!(self.f, "{}", x)
    }

    fn set_style(&mut self, _style: &S) {}
    fn save_style(&mut self) {}
    fn restore_style(&mut self) {}
}

#[test]
fn tests() {
    macro_rules! pe {
        (($($xs:tt)*)) => {$crate::sxfmt::PrettyExpr::list(vec![$(pe!($xs)),*])};
        ($x:ident) => {$crate::sxfmt::PrettyExpr::Stat(stringify!{$x})};
        ($x:expr) => {$crate::sxfmt::PrettyExpr::Atom($x.to_string())};
    }

    macro_rules! p {
        ($($x:tt)*) => {pe!($($x)*) as PrettyExpr::<()>};
    }

    assert_eq!(p!["abc"].inline_width(), Some(3));
    assert_eq!(p![abcde].inline_width(), Some(5));
    assert_eq!(p![()].inline_width(), Some(2));
    assert_eq!(p![(abc)].inline_width(), Some(5));
    assert_eq!(p![(a b c)].inline_width(), Some(7));
    assert_eq!(p![(let ((a 1) (b 2)) ("+" a b))].inline_width(), Some(27));

    let pf = PrettyFormatter::new(15, 2);

    assert_eq!(pf.pretty(p!["abc"]).to_string(), "abc");
    assert_eq!(pf.pretty(p![(if q a e)]).to_string(), "(if q a e)");
    assert_eq!(
        pf.pretty(p![(branchon question answer else)]).to_string(),
        "(branchon\n  question\n  answer\n  else)"
    );
    assert_eq!(
        pf.pretty(p![(branchon (a b) (c d) (e f))]).to_string(),
        "(branchon\n  (a b)\n  (c d)\n  (e f))"
    );
    assert_eq!(
        pf.pretty(p![(long_name (other_long_name (if q a e)))])
            .to_string(),
        "(long_name\n  (other_long_name\n    (if q a e)))"
    );
    assert_eq!(
        pf.pretty(p![(let ((a 1) (b 2) (c 3)) ("+" a b))])
            .to_string(),
        "(let\n  ((a 1)\n   (b 2)\n   (c 3))\n  (+ a b))"
    );

    println!("{}", p![(((((alpha (beta gamma) delta)))))]);
    assert_eq!(
        pf.pretty(p![(((((alpha (beta gamma) delta)))))])
            .to_string(),
        "(((((alpha\n      (beta\n        gamma)\n      delta)))))"
    );
}
