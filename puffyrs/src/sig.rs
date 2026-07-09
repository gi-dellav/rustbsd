use std::sync::atomic::{AtomicBool, Ordering};

/// Returns `true` if a signal–watching flag has been raised.
pub fn raised(flag: &AtomicBool) -> bool {
    flag.load(Ordering::Relaxed)
}

/// Declares a static `AtomicBool` flag and an `extern "C"` signal handler,
/// registers the handler with `signal(2)`, and returns a `&'static AtomicBool`
/// reference for use in a daemon loop condition.
///
/// ```ignore
/// let quit = signal_watch!(libc::SIGTERM => QUIT, on_term);
/// while !sig::raised(quit) { /* work */ }
/// ```
#[macro_export]
macro_rules! signal_watch {
    ($signal:path => $flag:ident, $handler:ident) => {
        {
            static $flag: ::std::sync::atomic::AtomicBool =
                ::std::sync::atomic::AtomicBool::new(false);
            extern "C" fn $handler(_: ::libc::c_int) {
                $flag.store(true, ::std::sync::atomic::Ordering::Relaxed);
            }
            unsafe {
                ::libc::signal(
                    $signal,
                    $handler as *const () as ::libc::sighandler_t,
                );
            }
            &$flag
        }
    };
}
