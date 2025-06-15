use crate::screen_char::ScreenChar;

pub(crate) mod mock;

pub(crate) const BUFFER_HEIGHT: usize = 25;
pub(crate) const BUFFER_WIDTH: usize = 80;

pub(crate) trait CharBuffer {
    fn set_char_at(&mut self, row: usize, col: usize, char: ScreenChar);
}

#[repr(transparent)]
pub(crate) struct VGABuffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl CharBuffer for VGABuffer {
    fn set_char_at(&mut self, row: usize, col: usize, char: ScreenChar)  {
        self.chars[row][col] = char;
    }
}