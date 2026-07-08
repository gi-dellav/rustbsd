use std::ffi::CString;

/// Converts a Rust `&str` to a null-terminated `CString`. Panics if `s` contains
/// interior NUL bytes (`\0`), since those cannot be represented in a C string.
pub fn cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}
