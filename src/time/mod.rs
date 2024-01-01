//! A pluggable time interface for FRC.
//! 
//! This allows for utilities to be written and run on any platform,
//! not caring what it should use for a time source.

mod default;
mod instant;

use std::sync::atomic::{self, AtomicBool};

pub use instant::Instant;
pub use std::time::{Duration, SystemTime};

/// We don't use the unit as not to depend on the units module when we don't need to
type Microsecond = u64;

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
    Duration::from_micros(unsafe { UPTIME_SOURCE() })
}

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error, Clone, Copy)]
pub enum PauseError {
    #[error("Pause not implemented for {} time source", unsafe { IMPLEMENTATION_NAME })]
    PauseNotImplemented,
    #[error("Cannot pause if already paused")]
    AlreadyPaused,
    #[error("Cannot un-pause if not paused")]
    NotPaused,
}

/// Pauses the time with platforms that support it
/// 
/// # Errors
///  - [`PauseError::PauseNotImplemented`] if [`pause_implemented`] returns false
///  - [`PauseError::AlreadyPaused`] if the [`is_paused`] is true and `should_pause` is true
///  - [`PauseError::NotPaused`] if the [`is_paused`] is false and `should_pause` is false
#[inline]
pub fn try_pause(should_pause: bool) -> Result<(), PauseError> {
    if IS_PAUSED.swap(should_pause, atomic::Ordering::Relaxed) == should_pause {
        if should_pause {
            Err(PauseError::AlreadyPaused)
        } else {
            Err(PauseError::NotPaused)
        }
    } else if !PAUSE_IMPLEMENTED.load(atomic::Ordering::Relaxed) {
        Err(PauseError::PauseNotImplemented)
    } else {
        unsafe { UPTIME_PAUSE(should_pause) };
        Ok(())
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

    #[derive(Debug, Clone, Copy)]
    pub struct TimeImplementation {
        pub implementation_name: &'static str,
        /// A custom monotomic timestamp imlementation
        pub uptime: fn() -> super::Microsecond,
        /// A custom pause implementation,
        /// this is allowed to panic.
        pub pause: Option<fn(bool) -> ()>,
        /// A function returning whether the system time is valid.
        pub system_time_valid: fn() -> bool,
    }

    ///This is called by the HAL to set the time implementation,
    ///could be called in other places but is not recommended.
    /// 
    /// # Safety
    /// - This function is not thread safe and should only be called once
    /// 
    /// # Panics
    /// - If called more than once
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
        if let Some(pause) = time_imp.pause {
            super::PAUSE_IMPLEMENTED.store(true, Ordering::Relaxed);
            super::UPTIME_PAUSE = pause;
        } else {
            super::PAUSE_IMPLEMENTED.store(false, Ordering::Relaxed);
            super::UPTIME_PAUSE = |_| {};
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
        assert!(end.saturating_sub(start) >= 100_000);
    }

    fn test_pause() {
        use super::*;
        try_pause(true).expect("Pause Error");
        let start = uptime().as_micros();
        thread::sleep(Duration::from_millis(1000));
        let end = uptime().as_micros();
        // assert!(end + 5 - start < 100);
        assert!(end.saturating_sub(start) < 100);
        try_pause(false).expect("Pause Error");
        thread::sleep(Duration::from_millis(1000));
        let end = uptime().as_micros();
        assert!(end.saturating_sub(start) >= 1_000_000);
    }

    /// Tests all of the time functions in one thread to make it sequential
    /// and not parallel messing up global state
    #[test]
    fn test_all() {
        test_time();
        test_pause();
    }
}
