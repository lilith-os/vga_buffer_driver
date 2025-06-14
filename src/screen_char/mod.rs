use crate::color::ColorCode;

pub struct ScreenChar {
    character: char,
    color_code: ColorCode,
}

impl ScreenChar {
    pub fn new() -> Self {
        Self {}
    }
}