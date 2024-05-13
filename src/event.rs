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

    MoveLeftWord,
    MoveRightWord,

    MoveHome,
    MoveEnd,

    Submit,
}

pub enum MenuCommand {
    MoveLeft,
    MoveRight,

    MoveDown,
    MoveUp,

    Submit,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Signal {
    Submit(String),

    Clear,

    Interrupted,
    Eof,
}
