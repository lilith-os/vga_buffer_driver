use crate::buffer::{VGABuffer, BUFFER_HEIGHT, BUFFER_WIDTH};
use crate::color::{Color, ColorCode};
use crate::screen_char::ScreenChar;

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut VGABuffer
}

impl Default for Writer {
    fn default() -> Self {
        Self::new()
    }
}

impl Writer {
    pub fn new() -> Self {
        Self {
            column_position: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
            buffer: unsafe { &mut *(0xb8000 as *mut VGABuffer) }
        }
    }
    
    pub fn write_char(&mut self, ch: char) {
        match ch { 
            '\n' => self.new_line(),
            ch => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code;
                
                self.buffer.set_char_at(row, col, ScreenChar::new(ch, color_code));
                self.column_position += 1;
            }
        }
    }
    
    pub fn new_line(&mut self) {
        todo!()
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