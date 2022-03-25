#[derive(Debug, Copy, Clone)]
pub enum Event {
    Unknown,
    Edit(char),
    EditBackspace,
    EditDelete,
    EditInsert,
    EditWrap,
    EditUnwrap,
    EditGrowLeft,
    EditGrowRight,
    EditShrinkLeft,
    EditShrinkRight,
    NavPrev,
    NavNext,
    NavPrevFast,
    NavNextFast,
}
