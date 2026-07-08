use std::io::{self, Write};
use std::process;

pub fn println_stdout(s: &str) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{s}").ok();
}

pub fn eprintln_stderr(s: &str) {
    let stderr = io::stderr();
    let mut handle = stderr.lock();
    writeln!(handle, "{s}").ok();
}

pub fn die(status: i32, msg: &str) -> ! {
    let stderr = io::stderr();
    let mut handle = stderr.lock();
    let _ = writeln!(handle, "{msg}");
    process::exit(status);
}

pub fn warn(msg: &str) {
    let stderr = io::stderr();
    let mut handle = stderr.lock();
    let _ = writeln!(handle, "{msg}");
}
