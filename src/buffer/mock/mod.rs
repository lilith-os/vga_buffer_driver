use crate::buffer::{CharBuffer, BUFFER_HEIGHT, BUFFER_WIDTH};
use crate::screen_char::ScreenChar;

#[derive(Debug)]
pub(crate) struct MockBuffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl MockBuffer {
    pub(crate) const fn default() -> Self {
        Self {
            chars: [[ScreenChar::default(); BUFFER_WIDTH]; BUFFER_HEIGHT]
        }
    }
    
    #[allow(unused)]
    pub(crate) fn print(&self) {
        println!("{:?}", self)
    }
}

impl CharBuffer for MockBuffer {
    fn set_char_at(&mut self, row: usize, col: usize, char: ScreenChar)  {
        self.chars[row][col] = char;
    }
    fn get_char_at(&self, row: usize, col: usize) -> ScreenChar {
        self.chars[row][col]
    }
}

impl Default for MockBuffer {
    fn default() -> Self {
        Self::default()
    }
}