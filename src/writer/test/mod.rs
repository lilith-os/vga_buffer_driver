use crate::buffer::BUFFER_HEIGHT;
use crate::buffer::mock::MockBuffer;
use crate::color::ColorCode;
use crate::screen_char::ScreenChar;
use crate::writer::Writer;

#[test]
fn write_char() {
    static mut MOCK_BUFFER: MockBuffer = MockBuffer::default();
    
    #[allow(unused_unsafe)]
    let mock_buffer= unsafe { &raw mut MOCK_BUFFER };
    
    let mut writer = Writer::new(mock_buffer);
    writer.write_char('H');
    
    assert_eq!(
        unsafe { (*mock_buffer).get_char_at(BUFFER_HEIGHT-1,0) }, 
        ScreenChar::new(b'H', ColorCode::default())
    )
}