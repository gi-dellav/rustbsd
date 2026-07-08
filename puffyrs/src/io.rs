use std::io::{self, Write};
use std::process;

/// Writes `s` followed by a newline to stdout. I/O errors are silently ignored.
pub fn println_stdout(s: &str) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{s}").ok();
}

/// Writes `s` followed by a newline to stderr. I/O errors are silently ignored.
pub fn eprintln_stderr(s: &str) {
    let stderr = io::stderr();
    let mut handle = stderr.lock();
    writeln!(handle, "{s}").ok();
}

/// Prints `msg` to stderr and exits the process with the given `status` code.
/// Never returns (diverging function).
pub fn die(status: i32, msg: &str) -> ! {
    let stderr = io::stderr();
    let mut handle = stderr.lock();
    let _ = writeln!(handle, "{msg}");
    process::exit(status);
}

/// Prints `msg` to stderr without exiting. I/O errors are silently ignored.
pub fn warn(msg: &str) {
    let stderr = io::stderr();
    let mut handle = stderr.lock();
    let _ = writeln!(handle, "{msg}");
}
