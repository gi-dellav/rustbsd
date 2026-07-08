use std::env;
use std::ptr;

use puffyrs::{exec, ffi::cstr, flags::FlagParser, io::die};

const PATH_BSHELL: &str = "/bin/sh";

fn usage() -> ! {
    let prog = env::args()
        .next()
        .and_then(|a| {
            std::path::Path::new(&a)
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| "chroot".to_string());
    eprintln!(
        "usage: {} [-g group,group,...] [-u user] newroot [command]",
        prog
    );
    std::process::exit(1);
}

fn main() {
    let parsed = FlagParser::new()
        .string('g')
        .string('u')
        .parse(env::args());

    let user = parsed.string('u');
    let grouplist = parsed.string('g');
    let args = parsed.args();

    if args.is_empty() {
        usage();
    }

    if let Some(ref u) = user {
        if u.is_empty() {
            usage();
        }
    }
    if let Some(ref g) = grouplist {
        if g.is_empty() {
            usage();
        }
    }

    let mut pwd_uid: u32 = 0;
    let mut pwd_gid: u32 = 0;

    #[cfg(not(target_os = "linux"))]
    let mut pwd_name: Option<String> = None;

    if let Some(ref username) = user {
        let c_user = cstr(username);
        let pw = unsafe { libc::getpwnam(c_user.as_ptr()) };
        if pw.is_null() {
            die(1, &format!("no such user '{}'", username));
        }
        unsafe {
            pwd_uid = (*pw).pw_uid;
            pwd_gid = (*pw).pw_gid;
        }
        #[cfg(not(target_os = "linux"))]
        unsafe {
            use std::ffi::CStr;
            pwd_name = Some(
                CStr::from_ptr((*pw).pw_name)
                    .to_string_lossy()
                    .into_owned(),
            );
        }
    }

    let mut ngids: usize = 0;
    let mut gidlist: [u32; 16] = [0; 16];

    if let Some(ref glist) = grouplist {
        for group_name in glist.split(',') {
            if group_name.is_empty() {
                continue;
            }

            if ngids >= gidlist.len() {
                die(1, "too many supplementary groups provided");
            }
            let c_group = cstr(group_name);
            let grp = unsafe { libc::getgrnam(c_group.as_ptr()) };
            if grp.is_null() {
                die(1, &format!("no such group '{}'", group_name));
            }
            unsafe {
                gidlist[ngids] = (*grp).gr_gid;
            }
            ngids += 1;
        }
    }

    if ngids > 0 {
        let ret = unsafe { libc::setgid(gidlist[0]) };
        if ret != 0 {
            die(1, "setgid");
        }
        #[cfg(target_os = "linux")]
        let ret = unsafe {
            libc::setgroups(ngids, gidlist.as_ptr() as *const libc::gid_t)
        };
        #[cfg(not(target_os = "linux"))]
        let ret = unsafe {
            libc::setgroups(ngids as i32, gidlist.as_ptr() as *const libc::gid_t)
        };
        if ret != 0 {
            die(1, "setgroups");
        }
    } else if user.is_some() {
        let c_user = cstr(user.unwrap());
        #[cfg(target_os = "linux")]
        let ret = unsafe { libc::initgroups(c_user.as_ptr(), pwd_gid) };
        #[cfg(not(target_os = "linux"))]
        let ret = unsafe { libc::initgroups(c_user.as_ptr(), pwd_gid as i32) };
        if ret != 0 {
            die(1, "initgroups");
        }
    }

    let newroot = cstr(&args[0]);
    if unsafe { libc::chroot(newroot.as_ptr()) } != 0
        || unsafe { libc::chdir(b"/\0" as *const u8 as *const i8) } != 0
    {
        die(1, &format!("{}", args[0]));
    }

    if user.is_some() {
        #[cfg(not(target_os = "linux"))]
        {
            let sess_id = unsafe { libc::getsid(0) };
            let pid = unsafe { libc::getpid() };
            if sess_id == pid || unsafe { libc::setsid() } != -1 {
                let name = pwd_name.as_ref().unwrap();
                let c_name = cstr(name);
                unsafe {
                    libc::setlogin(c_name.as_ptr());
                }
            }
        }
        let ret = unsafe { libc::setuid(pwd_uid) };
        if ret != 0 {
            die(1, "setuid");
        }
    }

    if args.len() > 1 {
        exec::execvp(&args[1], &args[1..]);
    }

    let shell = env::var("SHELL").unwrap_or_default();
    let shell = if shell.is_empty() {
        PATH_BSHELL
    } else {
        &shell
    };
    let c_shell = cstr(shell);
    let c_minus_i = cstr("-i");
    unsafe {
        libc::execlp(
            c_shell.as_ptr(),
            c_shell.as_ptr(),
            c_minus_i.as_ptr(),
            ptr::null::<i8>(),
        );
    }
    die(1, &format!("{}", shell));
}
