mod events;
mod proof;
pub mod sexpr_adapter;
pub mod sexpr_editor;
mod styles;
mod sxfmt;
mod textbuffer;
pub mod widgets;

pub use events::Event;
pub use styles::Style;
pub use sxfmt::{Formatter, PrettyExpr, PrettyFormatter};
pub use textbuffer::{RenderTarget, TextBuffer};

pub type Sexpr = PrettyExpr<Style>;
