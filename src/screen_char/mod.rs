use crate::color::ColorCode;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    character: u8,
    color_code: ColorCode,
}

impl ScreenChar {
    pub fn new(character: u8, color_code: ColorCode) -> Self {
        Self { character, color_code }
    }
    
    pub const fn default() -> Self { Self { character: b' ', color_code: ColorCode::default() } } 
}

impl Default for ScreenChar {
    fn default() -> Self {
        Self::default()
    }
}