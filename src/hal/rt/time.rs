//! Real Time Clock HAL Driver.
//! Not for direct use by users, use [`crate::time`] instead.

use crate::units::time::Microsecond;

/// A platform specific Monotonic Clock driver.
///
/// # Safety
/// ## Thread Safety
/// It's up to the driver developer to ensure that the driver is thread safe.
///
/// # Development Resources
/// - [WPILib Rio FPGAClock](https://github.com/wpilibsuite/allwpilib/blob/main/hal/src/main/native/athena/HAL.cpp#L388)
/// - [WPILib Sim FPGAClock](https://github.com/wpilibsuite/allwpilib/blob/main/hal/src/main/native/sim/MockHooks.cpp#L60)
pub trait ClockDriver: 'static {
    /// If this is true, [`crate::time::uptime`] will continue to use it's default implementation
    /// even if this driver is initialized. This is useful for platforms that just use system time like `FRC Sim`.
    const USE_DEFAULT_DRIVER: bool;

    /// True if this driver implements a non-panicing pause implementation
    const CAN_PAUSE: bool = false;

    /// Returns the uptime in microseconds.
    ///
    /// When the HAL is initialized and [`Self::USE_DEFAULT_UPTIME`] is false,
    /// [`crate::time::uptime`] will use this function to get the uptime.
    /// [`crate::time::uptime`] returns a [`std::time::Duration`], so it is recommended to use that function instead of this one.
    fn uptime() -> Microsecond;

    /// Pauses the program uptime until [`Self::pause`] is called again with `false`.
    ///
    /// When the HAL is initialized, [`crate::time::pause`] will use this function to pause the program.
    /// [`crate::time::pause`] has more implicit safety checks, so it is recommended to use that function instead of this one.
    ///
    /// # Safety
    /// This function should not be called by the user, only by [`crate::time::pause`].
    /// Any direct calls are undefined behavior.
    ///
    /// # Development
    /// ## Panics
    /// This function should panic if not available on a platform
    ///
    /// ## Implementing
    /// You can assume that `should_pause` will be `true` and `false` in alternating calls
    /// due to the [`crate::time::pause`] implementation.
    unsafe fn pause(_should_pause: bool) {}

    /// Returns is system time is a valid source of real-world time.
    /// For platforms like the `RoboRio` this will only be true when connected DS
    /// but platforms like the `Raspberry Pi` or `FRC Sim` will always be true because they have an RTC.
    fn system_time_valid() -> bool;
}

#[allow(dead_code)]
pub(crate) fn initialize_time_callbacks<T: ClockDriver>(name: &'static str) {
    assert!(std::mem::size_of::<T>() == 0, "Clock Driver must be zero sized");
    if T::USE_DEFAULT_DRIVER {
        return;
    }
    let r#impl = crate::time::__private::TimeImplementation {
        implementation_name: name,
        uptime: T::uptime,
        pause: if T::CAN_PAUSE {
            Some(|should_pause: bool| unsafe { T::pause(should_pause) })
        } else {
            None
        },
        system_time_valid: T::system_time_valid,
    };
    unsafe { crate::time::__private::set_time_implementation(r#impl) }
}
