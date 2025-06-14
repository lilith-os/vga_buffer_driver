use crate::color::ColorCode;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    character: char,
    color_code: ColorCode,
}

impl ScreenChar {
    pub fn new(character: char, color_code: ColorCode) -> Self {
        Self { character, color_code }
    }
}