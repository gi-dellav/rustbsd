use std::path::Path;

/// Returns the basename of `argv[0]`, falling back to `"???"` if unavailable.
pub fn getprogname() -> String {
    std::env::args()
        .next()
        .as_deref()
        .and_then(|a| Path::new(a).file_name())
        .and_then(|n| n.to_str())
        .map(String::from)
        .unwrap_or_else(|| String::from("???"))
}
