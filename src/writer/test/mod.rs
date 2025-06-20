use crate::buffer::{CharBuffer, BUFFER_HEIGHT};
use crate::buffer::mock::MockBuffer;
use crate::color::ColorCode;
use crate::screen_char::ScreenChar;
use crate::writer::VGAWriter;

#[test]
fn write_char() {
    static mut MOCK_BUFFER: MockBuffer = MockBuffer::default();

    #[allow(unused_unsafe)]
    let mock_buffer= unsafe { &raw mut MOCK_BUFFER };

    let mut writer = VGAWriter::new(mock_buffer);
    writer.write_char('H');

    assert_eq!(
        unsafe { (*mock_buffer).get_char_at(0,0) },
        ScreenChar::new(b'H', ColorCode::default())
    )
}

#[test]
fn write_some_chars() {
    static mut MOCK_BUFFER: MockBuffer = MockBuffer::default();

    #[allow(unused_unsafe)]
    let mock_buffer= unsafe { &raw mut MOCK_BUFFER };

    let mut writer = VGAWriter::new(mock_buffer);
    writer.write_char('H');
    writer.write_char('e');
    writer.write_char('y');

    assert_eq!(
        unsafe { (*mock_buffer).get_char_at(0,0) },
        ScreenChar::new(b'H', ColorCode::default())
    );

    assert_eq!(
        unsafe { (*mock_buffer).get_char_at(0,1) },
        ScreenChar::new(b'e', ColorCode::default())
    );

    assert_eq!(
        unsafe { (*mock_buffer).get_char_at(0,2) },
        ScreenChar::new(b'y', ColorCode::default())
    );
}

#[test]
fn chars_overflow() {
    static mut MOCK_BUFFER: MockBuffer = MockBuffer::default();

    #[allow(unused_unsafe)]
    let mock_buffer= unsafe { &raw mut MOCK_BUFFER };

    let mut writer = VGAWriter::new(mock_buffer);

    for _ in 0..BUFFER_HEIGHT {
        writer.write_string("Hey\n");
    }

    for row in 0..BUFFER_HEIGHT-1 {
        assert_eq!(
            unsafe { (*mock_buffer).get_char_at(row,0) },
            ScreenChar::new(b'H', ColorCode::default())
        );

        assert_eq!(
            unsafe { (*mock_buffer).get_char_at(row,1) },
            ScreenChar::new(b'e', ColorCode::default())
        );

        assert_eq!(
            unsafe { (*mock_buffer).get_char_at(row,2) },
            ScreenChar::new(b'y', ColorCode::default())
        );
    }
    
    writer.write_char('!');
    
    assert_eq!(
        unsafe { (*mock_buffer).get_char_at(BUFFER_HEIGHT-1,0) },
        ScreenChar::new(b'!', ColorCode::default())
    );

    assert_eq!(
        unsafe { (*mock_buffer).get_char_at(BUFFER_HEIGHT-1,1) },
        ScreenChar::new(b' ', ColorCode::default())
    );
}