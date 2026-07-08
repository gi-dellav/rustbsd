use std::ffi::CString;

pub fn cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}
