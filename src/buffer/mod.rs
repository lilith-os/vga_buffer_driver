use volatile::{VolatilePtr};
use crate::screen_char::ScreenChar;

pub(crate) mod mock;

pub(crate) const BUFFER_HEIGHT: usize = 25;
pub(crate) const BUFFER_WIDTH: usize = 80;

pub(crate) trait CharBuffer {
    fn set_char_at(&mut self, row: usize, col: usize, char: ScreenChar);
    fn get_char_at(&self, row: usize, col: usize) -> ScreenChar;
}

#[repr(transparent)]
pub(crate) struct VGABuffer<'a> {
    chars: [[VolatilePtr<'a, ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl CharBuffer for VGABuffer<'_> {
    fn set_char_at(&mut self, row: usize, col: usize, char: ScreenChar)  {
        self.chars[row][col].write(char);
    }

    fn get_char_at(&self, row: usize, col: usize) -> ScreenChar {
        self.chars[row][col].read()
    }
}