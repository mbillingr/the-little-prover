use crate::PrettyExpr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Style {
    Default,
    Background,
    Frame,
    Highlight,
    Keyword,
    Quote,
    True,
    False,
}

impl Default for Style {
    fn default() -> Style {
        Style::Default
    }
}

impl PrettyExpr<Style> {
    pub fn highlight(self) -> Self {
        match self {
            PrettyExpr::Atom(ref s) if is_keyword(&s) => {
                Self::Style(Style::Keyword, Box::new(self))
            }
            PrettyExpr::Stat(s) if is_keyword(s) => Self::Style(Style::Keyword, Box::new(self)),
            PrettyExpr::Quote(ref x) if x.get_text() == Some("t") => Self::Style(Style::True, Box::new(self)),
            PrettyExpr::Quote(ref x) if x.get_text() == Some("nil") => Self::Style(Style::False, Box::new(self)),
            PrettyExpr::Quote(_) => Self::Style(Style::Quote, Box::new(self)),
            PrettyExpr::Inline(xs) => PrettyExpr::Inline(highlight_vec(xs)),
            PrettyExpr::Expand(xs) => PrettyExpr::Expand(highlight_vec(xs)),
            PrettyExpr::SemiExpand(n, xs) => PrettyExpr::SemiExpand(n, highlight_vec(xs)),
            PrettyExpr::Style(s, x) => Self::Style(s, Box::new(x.highlight())),
            _ => self,
        }
    }
}

fn highlight_vec(xs: Vec<PrettyExpr<Style>>) -> Vec<PrettyExpr<Style>> {
    xs.into_iter().map(|x| x.highlight()).collect()
}

fn is_keyword(s: &str) -> bool {
    match s {
        "defun" => true,
        "dethm" => true,
        "if" => true,
        _ => false,
    }
}
