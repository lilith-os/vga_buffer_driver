use core::fmt;
use crate::buffer::{CharBuffer, VGABuffer, BUFFER_HEIGHT, BUFFER_WIDTH};
use crate::color::{Color, ColorCode};
use crate::screen_char::ScreenChar;

#[cfg(test)]
#[cfg(not(feature = "no_std"))]
mod test;

pub struct VGAWriter {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: *mut dyn CharBuffer
}

impl Default for VGAWriter {
    fn default() -> Self {
        Self::new(0xb8000 as *mut VGABuffer)
    }
}

impl VGAWriter {
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
    
    pub fn read_char_at(&mut self, row: usize, col: usize) -> ScreenChar {
        unsafe { (*self.buffer).get_char_at(row, col) }
    }
    
    pub fn new_line(&mut self) {
        self.column_position = 0;
        if self.row_position >= BUFFER_HEIGHT-1 {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let ch = self.read_char_at(row, col);
                    self.write_char_at(row-1, col, ch);
                }
            }
            self.clear_row(self.row_position);
            return;
        }
        self.row_position += 1;
    }
    
    pub fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.write_char_at(row, col, ScreenChar::new(b' ', self.color_code));
        }
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

impl fmt::Write for VGAWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}