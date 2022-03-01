#[derive(Debug, Copy, Clone)]
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
