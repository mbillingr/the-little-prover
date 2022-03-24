#[derive(Debug, Copy, Clone)]
pub enum Event {
    Unknown,
    Edit(char),
    EditBackspace,
    EditDelete,
    EditInsert,
    EditWrap,
    EditUnwrap,
    NavPrev,
    NavNext,
    NavPrevFast,
    NavNextFast,
}
