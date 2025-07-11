use volatile::{Volatile};
use crate::screen_char::ScreenChar;

#[cfg(not(feature = "no_std"))]
pub(crate) mod mock;

pub(crate) const BUFFER_HEIGHT: usize = 25;
pub(crate) const BUFFER_WIDTH: usize = 80;

pub(crate) trait CharBuffer: Send {
    fn set_char_at(&mut self, row: usize, col: usize, char: ScreenChar);
    fn get_char_at(&self, row: usize, col: usize) -> ScreenChar;
}

#[repr(transparent)]
pub(crate) struct VGABuffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl CharBuffer for VGABuffer {
    fn set_char_at(&mut self, row: usize, col: usize, char: ScreenChar)  {
        self.chars[row][col].write(char);
    }

    fn get_char_at(&self, row: usize, col: usize) -> ScreenChar {
        self.chars[row][col].read()
    }
}