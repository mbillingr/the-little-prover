mod events;
mod proof;
pub mod sexpr_adapter;
pub mod sexpr_editor;
mod styles;
mod sxfmt;

pub use events::Event;
pub use proof::{define, proof};
pub use styles::Style;
pub use sxfmt::{Formatter, PrettyExpr, PrettyFormatter};

pub type Sexpr = PrettyExpr<Style>;
