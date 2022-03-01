#[derive(Debug, Copy, Clone)]
pub enum Event {
    Unknown,
    Edit(char),
    EditBackspace,
    EditDelete,
    EditWrap,
    EditUnwrap,
    NavLeft,
    NavRight,
    NavUp,
    NavDown,
}
