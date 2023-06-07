
use std::slice;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MESSAGEBOX_STYLE};
use windows::core::PCWSTR;
use encoding::{Encoding, EncoderTrap};
use encoding::all::UTF_16LE;

fn to_utf16(s: &str) -> Vec<u8> {
    let mut result = UTF_16LE.encode(s, EncoderTrap::Ignore).unwrap_or(Vec::new());
    result.push(0);
    result.push(0);
    result
}

unsafe fn as_pcwstr(v: &Vec<u8>) -> PCWSTR {
    let s : &[u16] = slice::from_raw_parts(v.as_ptr() as *const _, 
        v.len()/2);
    PCWSTR { 0: s.as_ptr() }
}

pub fn error_popup(title: &str, text: &str) {
    unsafe {
        MessageBoxW(None, as_pcwstr(&to_utf16(text)), as_pcwstr(&to_utf16(title)), MESSAGEBOX_STYLE { 0: 0x00000010 });
    }
}