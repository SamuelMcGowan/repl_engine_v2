use crate::event::{ControlFlow, EditorCommand};

#[derive(Default)]
pub struct Editor {
    s: String,
    num_lines: usize,

    cursor_byte: usize,
}

impl Editor {
    pub fn insert_char(&mut self, ch: char) {
        self.s.insert(self.cursor_byte, ch);
        self.cursor_byte += ch.len_utf8();

        if ch == '\n' {
            self.num_lines += 1;
        }
    }

    pub fn insert_str(&mut self, s: &str) {
        self.s.insert_str(self.cursor_byte, s);
        self.cursor_byte += s.len();
        self.num_lines += s.chars().filter(|&ch| ch == '\n').count();
    }

    pub fn delete_char(&mut self) {
        if self.cursor_byte < self.s.len() {
            let ch = self.s.remove(self.cursor_byte);
            if ch == '\n' {
                self.num_lines -= 1;
            }
        }
    }

    pub fn backspace_char(&mut self) {
        if self.move_left() {
            self.delete_char();
        }
    }

    pub fn move_left(&mut self) -> bool {
        if let Some(ch) = self.before_cursor().chars().next_back() {
            self.cursor_byte -= ch.len_utf8();
            true
        } else {
            false
        }
    }

    pub fn move_right(&mut self) -> bool {
        if let Some(ch) = self.after_cursor().chars().next() {
            self.cursor_byte += ch.len_utf8();
            true
        } else {
            false
        }
    }

    pub fn move_home(&mut self) {
        self.cursor_byte = 0;
    }

    pub fn move_end(&mut self) {
        self.cursor_byte = self.s.len();
    }

    pub fn as_str(&self) -> &str {
        &self.s
    }

    pub fn cursor_byte(&self) -> usize {
        self.cursor_byte
    }

    fn before_cursor(&self) -> &str {
        &self.s[..self.cursor_byte]
    }

    fn after_cursor(&self) -> &str {
        &self.s[self.cursor_byte..]
    }
}

pub trait Handler<Command> {
    fn handle(&mut self, command: Command) -> Option<ControlFlow>;
}

impl Handler<EditorCommand> for Editor {
    fn handle(&mut self, command: EditorCommand) -> Option<ControlFlow> {
        let handled = match command {
            EditorCommand::InsertChar(ch) => {
                if ch == '\n' {
                    return Some(ControlFlow::Submit);
                } else {
                    self.insert_char(ch);
                    true
                }
            }

            EditorCommand::InsertString(s) => {
                self.insert_str(&s);
                true
            }

            EditorCommand::DeleteChar | EditorCommand::DeleteToken => {
                self.delete_char();
                true
            }
            EditorCommand::BackspaceChar | EditorCommand::BackspaceToken => {
                self.backspace_char();
                true
            }
            EditorCommand::MoveLeft => {
                self.move_left();
                true
            }
            EditorCommand::MoveRight => {
                self.move_right();
                true
            }
            EditorCommand::MoveUp => todo!(),
            EditorCommand::MoveDown => todo!(),
            EditorCommand::MoveLeftToken => todo!(),
            EditorCommand::MoveRightToken => todo!(),
            EditorCommand::MoveHome => todo!(),
            EditorCommand::MoveEnd => todo!(),
        };

        handled.then_some(ControlFlow::Continue)
    }
}
