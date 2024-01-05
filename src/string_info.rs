use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use crate::Vec2;

pub trait StringInfo {
    fn byte_to_position(&self, byte: usize) -> Vec2;
    fn position_to_byte(&self, position: Vec2) -> usize;
}

impl StringInfo for str {
    fn byte_to_position(&self, byte: usize) -> Vec2 {
        let byte = byte.min(self.len());
        let before = &self[..byte];

        match before.rsplit_once('\n') {
            Some((before, last_line)) => {
                let line = before.split('\n').count() as u16;
                let col = last_line.width() as u16;
                Vec2::new(col, line)
            }
            None => Vec2::new(before.width() as u16, 0),
        }
    }

    fn position_to_byte(&self, position: Vec2) -> usize {
        let mut s = self;

        // skip `position.y` lines
        for _ in 0..position.y {
            s = match s.split_once('\n') {
                Some((_, rest)) => rest,
                None => "",
            }
        }

        // skip `position.x` columns
        let mut chars = s.chars();
        let mut offset_x = 0;

        while offset_x < position.x as usize && !chars.as_str().starts_with('\n') {
            let Some(ch) = chars.next() else {
                break;
            };

            let len = ch.width().unwrap_or(0);
            offset_x += len;
        }

        self.len() - chars.as_str().len()
    }
}

#[test]
fn test_byte_to_position() {
    assert_eq!("hello\nworld".byte_to_position(0), Vec2::new(0, 0));
    assert_eq!("hello\nworld".byte_to_position(5), Vec2::new(5, 0));
    assert_eq!("hello\nworld".byte_to_position(6), Vec2::new(0, 1));
    assert_eq!("hello\nworld".byte_to_position(11), Vec2::new(5, 1));
    assert_eq!("hello\nworld".byte_to_position(12), Vec2::new(5, 1)); // overrunning
}

#[test]
fn test_position_to_byte() {
    // lines
    assert_eq!("hello\nworld".position_to_byte(Vec2::new(5, 0)), 5);
    assert_eq!("hello\nworld".position_to_byte(Vec2::new(6, 0)), 5); // overrunning line
    assert_eq!("hello\nworld".position_to_byte(Vec2::new(0, 1)), 6);
    assert_eq!("hello\nworld".position_to_byte(Vec2::new(5, 1)), 11);
    assert_eq!("hello\nworld".position_to_byte(Vec2::new(6, 1)), 11); // overrunning
    assert_eq!("hello\nworld".position_to_byte(Vec2::new(0, 2)), 11); // overrunning

    // character widths
    assert_eq!("hëy".position_to_byte(Vec2::new(0, 0)), 0);
    assert_eq!("hëy".position_to_byte(Vec2::new(1, 0)), 1);
    assert_eq!("hëy".position_to_byte(Vec2::new(2, 0)), 3);
    assert_eq!("hëy".position_to_byte(Vec2::new(3, 0)), 4);
    assert_eq!("hëy".position_to_byte(Vec2::new(4, 0)), 4); // overrunning
}
