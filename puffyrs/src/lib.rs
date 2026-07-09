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
/// Program–name extraction from argv[0].
pub mod progname;
/// Daemon signal–watching via `AtomicBool` + `signal(2)`.
pub mod sig;
/// OpenBSD `strtonum(3)`: bounded numeric parsing.
pub mod strtonum;
/// Byte-visibility rendering (`vis(3)` semantics, used by `cat -v`).
pub mod vis;
