use crate::buffer::{CharBuffer, BUFFER_HEIGHT, BUFFER_WIDTH};
use crate::screen_char::ScreenChar;

pub(crate) struct MockBuffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl MockBuffer {
    pub const fn default() -> Self {
        Self {
            chars: [[ScreenChar::default(); BUFFER_WIDTH]; BUFFER_HEIGHT]
        }
    }
    
    pub(crate) fn get_char_at(&self, row: usize, col: usize) -> ScreenChar {
        self.chars[row][col]
    }
}

impl CharBuffer for MockBuffer {
    fn set_char_at(&mut self, row: usize, col: usize, char: ScreenChar)  {
        self.chars[row][col] = char;
    }
}

impl Default for MockBuffer {
    fn default() -> Self {
        Self::default()
    }
}