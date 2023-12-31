mod default;
mod instant;

use std::sync::atomic::{self, AtomicBool};
use crate::units::time::Microsecond;

pub use instant::Instant;
pub use std::time::{Duration, SystemTime};

static mut UPTIME_SOURCE: fn() -> Microsecond = default::default_uptime_source;
static mut UPTIME_PAUSE: fn(bool) = default::default_uptime_pause;
static mut SYSTEM_TIME_VALID: fn() -> bool = || true;
static mut IMPLEMENTATION_NAME: &str = default::DEFAULT_IMPL_NAME;

static IS_PAUSED: AtomicBool = AtomicBool::new(false);
static PAUSE_IMPLEMENTED: AtomicBool = AtomicBool::new(true);
static TIME_IMPL_FROZEN: atomic::AtomicBool = atomic::AtomicBool::new(false);

/// Gets the uptime of the program in microseconds
///
/// This can be manipulated with [`pause`] on platforms that support it
#[inline]
pub fn uptime() -> Duration {
    TIME_IMPL_FROZEN.store(true, atomic::Ordering::Relaxed);
    unsafe { UPTIME_SOURCE() }.into()
}

/// Pauses the time with platforms that support it
///
/// # Panics
/// Panics if the time source is already paused and `should_pause` is true
/// Panics if the time source is not paused and `should_pause` is false
/// Panics if called and [`pause_implemented`] returns false
#[inline]
pub fn pause(should_pause: bool) {
    if IS_PAUSED.swap(should_pause, atomic::Ordering::Relaxed) == should_pause {
        if should_pause {
            panic!("Cannot pause if already paused")
        } else {
            panic!("Cannot un-pause if not paused")
        }
    } else if !PAUSE_IMPLEMENTED.load(atomic::Ordering::Relaxed) {
        unimplemented!("Pause not implemented for {} time source", unsafe {
            IMPLEMENTATION_NAME
        })
    } else {
        unsafe { UPTIME_PAUSE(should_pause) };
    }
}

/// Returns true if the time source is paused
#[inline]
pub fn is_paused() -> bool {
    IS_PAUSED.load(atomic::Ordering::Relaxed)
}

/// Returns true if the time source supports pausing
#[inline]
pub fn pause_implemented() -> bool {
    PAUSE_IMPLEMENTED.load(atomic::Ordering::Relaxed)
}

/// Gets the current system time if its currently valid, otherwise returns None.
/// System time validity can changed anytime unlike time-source implementation.
#[inline]
pub fn system_time() -> Option<SystemTime> {
    TIME_IMPL_FROZEN.store(true, atomic::Ordering::Relaxed);
    if unsafe { SYSTEM_TIME_VALID() } {
        Some(SystemTime::now())
    } else {
        None
    }
}

#[doc(hidden)]
pub mod __private {
    use crate::units::time::Microsecond;

    #[derive(Debug, Clone, Copy)]
    pub struct TimeImplementation {
        pub implementation_name: &'static str,
        /// A custom monotomic timestamp imlementation
        pub uptime: fn() -> Microsecond,
        /// A custom pause implementation,
        /// this is allowed to panic.
        pub pause: Option<fn(bool) -> ()>,
        /// A function returning whether the system time is valid.
        pub system_time_valid: fn() -> bool,
    }

    ///This is called by the HAL to set the time implementation,
    ///could be called in other places but is not recommended.
    pub unsafe fn set_time_implementation(time_imp: TimeImplementation) {
        use std::sync::atomic::Ordering;
        assert!(
            !super::TIME_IMPL_FROZEN.swap(true, Ordering::Relaxed),
            "Cannot set time source after it has been used or previously set(old: {}, new: {})",
            super::IMPLEMENTATION_NAME,
            time_imp.implementation_name
        );
        super::UPTIME_SOURCE = time_imp.uptime;
        super::IMPLEMENTATION_NAME = time_imp.implementation_name;
        super::SYSTEM_TIME_VALID = time_imp.system_time_valid;
        #[allow(clippy::option_if_let_else)]
        if let Some(pause) = time_imp.pause {
            super::PAUSE_IMPLEMENTED.store(true, Ordering::Relaxed);
            super::UPTIME_PAUSE = pause;
        } else {
            super::PAUSE_IMPLEMENTED.store(false, Ordering::Relaxed);
            super::UPTIME_PAUSE = |_| unimplemented!("Pause not implemented for {} time source", unsafe {
                super::IMPLEMENTATION_NAME
            });
        }
    }
}

#[cfg(test)]
mod test {
    use std::thread;

    fn test_time() {
        use super::*;
        let start = uptime().as_micros();
        thread::sleep(Duration::from_millis(100));
        let end = uptime().as_micros();
        assert!(end - start >= 100_000);
    }

    fn test_pause() {
        use super::*;
        pause(true);
        let start = uptime().as_micros();
        thread::sleep(Duration::from_millis(1000));
        let end = uptime().as_micros();
        assert!(end + 5 - start < 100);
        pause(false);
        thread::sleep(Duration::from_millis(1000));
        let end = uptime().as_micros();
        assert!(end - start >= 1_000_000);
    }

    /// Tests all of the time functions in one thread to make it sequential
    /// and not parallel messing up global state
    #[test]
    fn test_all() {
        test_time();
        test_pause();
    }
}
