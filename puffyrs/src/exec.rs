use std::ffi::CString;
use std::ptr;

use crate::ffi;
use crate::io;

/// Replaces the current process image with `cmd` and the given `args`, using `execvp(3)`.
/// This is a diverging function: on success the caller is gone; on failure it calls `die`.
pub fn execvp(cmd: &str, args: &[String]) -> ! {
    let c_args: Vec<CString> = args.iter().map(|a| ffi::cstr(a)).collect();
    let mut c_argv: Vec<*const i8> = c_args.iter().map(|a| a.as_ptr()).collect();
    c_argv.push(ptr::null());
    let c_cmd = ffi::cstr(cmd);
    unsafe {
        libc::execvp(c_cmd.as_ptr(), c_argv.as_ptr());
    }
    io::die(1, cmd);
}
