#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Style {
    Default,
    Background,
    Frame,
    Highlight,
}

impl Default for Style {
    fn default() -> Style {
        Style::Default
    }
}
