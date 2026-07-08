use std::ffi::CString;
use std::ptr;

use crate::ffi;
use crate::io;

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
