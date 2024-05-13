#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EditorCommand {
    InsertChar(char),
    InsertString(String),

    DeleteChar,
    DeleteToken,

    BackspaceChar,
    BackspaceToken,

    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,

    MoveLeftToken,
    MoveRightToken,

    MoveHome,
    MoveEnd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControlFlow {
    Continue,
    Submit,
}
