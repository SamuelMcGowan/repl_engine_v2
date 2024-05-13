use std::ops::ControlFlow;

use crate::event::{EditorCommand, Signal};
use crate::string_info::StringInfo;
use crate::Vec2;

pub struct Editor {
    s: String,
    num_lines: usize,

    cursor_byte: usize,
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

impl Editor {
    pub fn new() -> Self {
        Self {
            s: String::new(),
            num_lines: 0,
            cursor_byte: 0,
        }
    }

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

    pub fn delete_word(&mut self) {
        let prev_cursor_byte = self.cursor_byte;
        self.move_right_word();

        let word = &self.s[prev_cursor_byte..self.cursor_byte];
        self.num_lines -= word.chars().filter(|&ch| ch == '\n').count();

        self.s.replace_range(prev_cursor_byte..self.cursor_byte, "");

        self.cursor_byte = prev_cursor_byte;
    }

    pub fn backspace_word(&mut self) {
        let prev_cursor_byte = self.cursor_byte;
        self.move_left_word();

        let word = &self.s[self.cursor_byte..prev_cursor_byte];
        self.num_lines -= word.chars().filter(|&ch| ch == '\n').count();

        self.s.replace_range(self.cursor_byte..prev_cursor_byte, "");
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

    pub fn move_up(&mut self) {
        let mut pos = self.s.byte_to_position(self.cursor_byte);

        if pos.y == 0 {
            self.cursor_byte = 0;
        } else {
            pos.y -= 1;
            self.cursor_byte = self.s.position_to_byte(pos);
        }
    }

    pub fn move_down(&mut self) {
        let mut pos = self.s.byte_to_position(self.cursor_byte);
        pos.y += 1;

        if pos.y as usize == self.num_lines {
            self.cursor_byte = self.s.len();
        } else {
            self.cursor_byte = self.s.position_to_byte(pos);
        }
    }

    pub fn move_left_word(&mut self) -> usize {
        self.move_left();

        let new_cursor_byte = self.before_cursor().trim_end_matches(is_word).len();

        let diff = self.cursor_byte - new_cursor_byte;
        self.cursor_byte = new_cursor_byte;

        diff
    }

    pub fn move_right_word(&mut self) -> usize {
        self.move_right();

        let new_cursor_byte = self.s.len() - self.after_cursor().trim_start_matches(is_word).len();

        let diff = new_cursor_byte - self.cursor_byte;
        self.cursor_byte = new_cursor_byte;

        diff
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

    pub fn cursor_pos(&self) -> Vec2 {
        self.s.byte_to_position(self.cursor_byte)
    }

    fn before_cursor(&self) -> &str {
        &self.s[..self.cursor_byte]
    }

    fn after_cursor(&self) -> &str {
        &self.s[self.cursor_byte..]
    }

    pub fn take(&mut self) -> String {
        let s = self.s.clone();
        self.clear();
        s
    }

    pub fn clear(&mut self) {
        self.s.clear();
        self.cursor_byte = 0;
        self.num_lines = 1;
    }
}

pub trait Handler<Command> {
    fn handle(&mut self, command: Command) -> Option<ControlFlow<Signal>>;
}

impl Handler<EditorCommand> for Editor {
    fn handle(&mut self, command: EditorCommand) -> Option<ControlFlow<Signal>> {
        let handled = match command {
            EditorCommand::InsertChar(ch) => {
                self.insert_char(ch);
                true
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
            EditorCommand::MoveUp => {
                self.move_up();
                true
            }
            EditorCommand::MoveDown => {
                self.move_down();
                true
            }

            EditorCommand::MoveLeftWord => {
                self.move_left_word();
                true
            }
            EditorCommand::MoveRightWord => {
                self.move_right_word();
                true
            }

            EditorCommand::MoveHome => {
                self.move_home();
                true
            }
            EditorCommand::MoveEnd => {
                self.move_end();
                true
            }

            EditorCommand::Submit => return Some(ControlFlow::Break(Signal::Submit(self.take()))),
        };

        handled.then_some(ControlFlow::Continue(()))
    }
}

fn is_word(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '-' || ch == '_'
}
