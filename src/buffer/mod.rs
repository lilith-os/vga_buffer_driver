use crate::screen_char::ScreenChar;

pub(crate) const BUFFER_HEIGHT: usize = 25;
pub(crate) const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct VGABuffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl VGABuffer {
    pub(crate) fn set_char_at(&mut self, row: usize, col: usize, char: ScreenChar) {
        self.chars[row][col] = char;
    }
}