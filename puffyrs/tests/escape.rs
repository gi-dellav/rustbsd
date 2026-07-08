use puffyrs::escape;

fn write_str(s: &str) -> (String, bool) {
    let mut buf = Vec::new();
    let complete = escape::write_escaped(&mut buf, s).unwrap();
    (String::from_utf8(buf).unwrap(), complete)
}

#[test]
fn plain_text() {
    let (out, complete) = write_str("hello");
    assert_eq!(out, "hello");
    assert!(complete);
}

#[test]
fn backslash_backslash() {
    let (out, complete) = write_str("\\\\");
    assert_eq!(out, "\\");
    assert!(complete);
}

#[test]
fn escape_newline() {
    let (out, complete) = write_str("a\\nb");
    assert_eq!(out, "a\nb");
    assert!(complete);
}

#[test]
fn escape_tab() {
    let (out, complete) = write_str("a\\tb");
    assert_eq!(out, "a\tb");
    assert!(complete);
}

#[test]
fn escape_carriage_return() {
    let (out, complete) = write_str("a\\rb");
    assert_eq!(out, "a\rb");
    assert!(complete);
}

#[test]
fn escape_bell() {
    let (out, complete) = write_str("\\a");
    assert_eq!(out, "\x07");
    assert!(complete);
}

#[test]
fn escape_backspace() {
    let (out, complete) = write_str("\\b");
    assert_eq!(out, "\x08");
    assert!(complete);
}

#[test]
fn escape_stop() {
    let (out, complete) = write_str("hello\\cworld");
    assert_eq!(out, "hello");
    assert!(!complete);
}

#[test]
fn escape_escape() {
    let (out, complete) = write_str("\\e");
    assert_eq!(out, "\x1b");
    assert!(complete);
}

#[test]
fn escape_form_feed() {
    let (out, complete) = write_str("\\f");
    assert_eq!(out, "\x0c");
    assert!(complete);
}

#[test]
fn escape_vertical_tab() {
    let (out, complete) = write_str("\\v");
    assert_eq!(out, "\x0b");
    assert!(complete);
}

#[test]
fn escape_octal() {
    let (out, complete) = write_str("\\0141");
    assert_eq!(out, "a");
    assert!(complete);
}

#[test]
fn escape_octal_zero_only() {
    let (out, complete) = write_str("\\0");
    assert_eq!(out, "\0");
    assert!(complete);
}

#[test]
fn escape_octal_partial() {
    let (out, complete) = write_str("\\07x");
    assert_eq!(out, "\x07x");
    assert!(complete);
}

#[test]
fn escape_hex() {
    let (out, complete) = write_str("\\x61");
    assert_eq!(out, "a");
    assert!(complete);
}

#[test]
fn escape_hex_incomplete() {
    let (out, complete) = write_str("\\x");
    assert_eq!(out, "\\x");
    assert!(complete);
}

#[test]
fn escape_hex_partial() {
    let (out, complete) = write_str("\\x6g");
    assert_eq!(out, "\x06g");
    assert!(complete);
}

#[test]
fn unknown_escape() {
    let (out, complete) = write_str("\\q");
    assert_eq!(out, "\\q");
    assert!(complete);
}

#[test]
fn trailing_backslash() {
    let (out, complete) = write_str("test\\");
    assert_eq!(out, "test\\");
    assert!(complete);
}

#[test]
fn unescape_fn() {
    let (out, complete) = escape::unescape("a\\nb");
    assert_eq!(out, "a\nb");
    assert!(complete);
}

#[test]
fn unescape_stop() {
    let (out, complete) = escape::unescape("hello\\cworld");
    assert_eq!(out, "hello");
    assert!(!complete);
}
