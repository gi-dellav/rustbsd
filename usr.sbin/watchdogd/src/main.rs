use std::path::Path;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};

use libc::{
    self, c_int, size_t,
    EOPNOTSUPP, MCL_CURRENT, MCL_FUTURE, PRIO_PROCESS, RLIMIT_STACK, SIGTERM,
};
use puffyrs::{flags::FlagParser, io};

const CTL_KERN: c_int = 1;
const KERN_WATCHDOG: c_int = 64;
const KERN_WATCHDOG_PERIOD: c_int = 1;
const KERN_WATCHDOG_AUTO: c_int = 2;

static QUIT: AtomicBool = AtomicBool::new(false);

extern "C" fn sighdlr(_signum: c_int) {
    QUIT.store(true, Ordering::Relaxed);
}

fn usage() -> ! {
    let args: Vec<String> = std::env::args().collect();
    let progname = args.first()
        .and_then(|a| Path::new(a).file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("watchdogd");
    io::die(1, &format!(
        "usage: {} [-dnq] [-i interval] [-p period]",
        progname
    ));
}

fn strtonum(s: &str, min: i64, max: i64) -> Result<u32, &'static str> {
    match s.parse::<i64>() {
        Ok(n) if n < min => Err("too small"),
        Ok(n) if n > max => Err("too large"),
        Ok(n) => Ok(n as u32),
        Err(_) => Err("invalid"),
    }
}

unsafe fn restore(mib: &mut [c_int; 3], speriod: c_int, sauto: c_int) {
    mib[2] = KERN_WATCHDOG_PERIOD;
    libc::sysctl(
        mib.as_mut_ptr(),
        3,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        &speriod as *const c_int as *mut _,
        std::mem::size_of::<c_int>(),
    );
    mib[2] = KERN_WATCHDOG_AUTO;
    libc::sysctl(
        mib.as_mut_ptr(),
        3,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        &sauto as *const c_int as *mut _,
        std::mem::size_of::<c_int>(),
    );
}

fn main() {
    let parsed = FlagParser::new()
        .bool('d', false)
        .bool('n', false)
        .bool('q', false)
        .string('i')
        .string('p')
        .parse(std::env::args());

    let daemonize = !parsed.bool('d');
    let do_restore = !parsed.bool('n');
    let quiet = parsed.bool('q');

    if !parsed.args().is_empty() {
        usage();
    }

    let mut interval: u32 = 0;
    let mut period: u32 = 30;

    if let Some(v) = parsed.string('i') {
        interval = strtonum(v, 1, 86400).unwrap_or_else(|errstr| {
            io::die(1, &format!("interval is {}: {}", errstr, v));
        });
    }

    if let Some(v) = parsed.string('p') {
        period = strtonum(v, 2, 86400).unwrap_or_else(|errstr| {
            io::die(1, &format!("period is {}: {}", errstr, v));
        });
    }

    if interval == 0 {
        interval = period / 3;
        if interval == 0 {
            interval = 1;
        }
    }

    if period <= interval {
        io::die(1, "retrigger interval too long");
    }

    let mut mib: [c_int; 3] = [CTL_KERN, KERN_WATCHDOG, KERN_WATCHDOG_PERIOD];
    let mut speriod: c_int = 0;
    let mut len: size_t = std::mem::size_of::<c_int>();

    unsafe {
        if libc::sysctl(
            mib.as_mut_ptr(),
            3,
            &mut speriod as *mut c_int as *mut _,
            &mut len,
            &period as *const u32 as *mut _,
            std::mem::size_of::<u32>(),
        ) == -1
        {
            let err = std::io::Error::last_os_error();
            if err.raw_os_error() == Some(EOPNOTSUPP) {
                io::die(1, "no watchdog timer available");
            } else {
                io::die(1, "can't access kern.watchdog.period");
            }
        }
    }

    mib[2] = KERN_WATCHDOG_AUTO;
    let mut sauto: c_int = 0;
    len = std::mem::size_of::<c_int>();
    let trigauto: c_int = 0;

    unsafe {
        if libc::sysctl(
            mib.as_mut_ptr(),
            3,
            &mut sauto as *mut c_int as *mut _,
            &mut len,
            &trigauto as *const c_int as *mut _,
            std::mem::size_of::<c_int>(),
        ) == -1
        {
            io::die(1, "can't access kern.watchdog.auto");
        }
    }

    mib[2] = KERN_WATCHDOG_PERIOD;
    let mut nperiod: c_int = 0;
    len = std::mem::size_of::<c_int>();

    unsafe {
        if libc::sysctl(
            mib.as_mut_ptr(),
            3,
            &mut nperiod as *mut c_int as *mut _,
            &mut len,
            std::ptr::null_mut(),
            0,
        ) == -1
        {
            io::warn("can't read back kern.watchdog.period, restoring original values");
            restore(&mut mib, speriod, sauto);
            process::exit(1);
        }
    }

    if nperiod as u32 != period && !quiet {
        io::warn(&format!("period adjusted to {} by device", nperiod));
    }

    if nperiod as u32 <= interval {
        io::warn(&format!(
            "retrigger interval {} too long, restoring original values",
            interval,
        ));
        unsafe { restore(&mut mib, speriod, sauto); }
        process::exit(1);
    }

    if daemonize {
        unsafe {
            if libc::daemon(0, 0) != 0 {
                io::warn("can't daemonize, restoring original values");
                restore(&mut mib, speriod, sauto);
                process::exit(1);
            }
        }
    }

    unsafe {
        let rlim = libc::rlimit {
            rlim_cur: 256 * 1024,
            rlim_max: 256 * 1024,
        };
        libc::setrlimit(RLIMIT_STACK, &rlim);
        libc::mlockall(MCL_CURRENT | MCL_FUTURE);
        libc::setpriority(PRIO_PROCESS, libc::getpid() as libc::id_t, -5);
        libc::signal(SIGTERM, sighdlr as *const () as libc::sighandler_t);
    }

    let mut retval: i32 = 0;
    mib[2] = KERN_WATCHDOG_PERIOD;

    while !QUIT.load(Ordering::Relaxed) {
        unsafe {
            if libc::sysctl(
                mib.as_mut_ptr(),
                3,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &period as *const u32 as *mut _,
                std::mem::size_of::<u32>(),
            ) == -1
            {
                QUIT.store(true, Ordering::Relaxed);
                retval = 1;
            }
        }
        unsafe {
            libc::sleep(interval);
        }
    }

    if do_restore {
        unsafe { restore(&mut mib, speriod, sauto); }
    }

    process::exit(retval);
}
