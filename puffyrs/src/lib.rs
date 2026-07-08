/// Backslash escape sequence processing (C-style escapes).
pub mod escape;
/// Process execution via `execvp(3)`.
pub mod exec;
/// FFI helpers for C string conversion.
pub mod ffi;
/// Custom flag parser with GNU-style short option support.
pub mod flags;
/// Convenient I/O helpers for stdout/stderr output.
pub mod io;
