use crate::buffer::{CharBuffer, VGABuffer, BUFFER_WIDTH};
use crate::color::{Color, ColorCode};
use crate::screen_char::ScreenChar;

#[cfg(test)]
#[cfg(not(feature = "no_std"))]
mod test;

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: *mut dyn CharBuffer
}

impl Default for Writer {
    fn default() -> Self {
        Self::new(0xb8000 as *mut VGABuffer)
    }
}

impl Writer {
    pub(crate) fn new(buffer: *mut dyn CharBuffer) -> Self {
        Self {
            column_position: 0,
            row_position: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
            buffer
        }
    }
    
    pub fn write_char(&mut self, ch: char) {
        match ch as u8 { 
            b'\n' => self.new_line(),
            ch => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                
                let row = self.row_position;
                let col = self.column_position;
                let color_code = self.color_code;
                
                self.write_char_at(row, col, ScreenChar::new(ch, color_code));
                self.column_position += 1;
            }
        }
    }
    
    pub fn write_char_at(&mut self, row: usize, col: usize, ch: ScreenChar) {
        unsafe { (*self.buffer).set_char_at(row, col, ch) };
    }
    
    pub fn new_line(&mut self) {
        self.column_position = 0;
        self.row_position += 1;
    }
    
    pub fn write_string(&mut self, string: &str) {
        for ch in string.chars() {
            match ch.is_ascii() {
                true => self.write_char(ch),
                false => self.write_char(0xfe as char)
            }
        }
    }
}