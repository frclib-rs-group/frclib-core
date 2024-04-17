//! This module contains what is needed to write and use a HAL(Hardware Abstraction Layer).

pub mod comm;
pub mod gpio;
pub mod rt;

#[cfg(not(test))]
use std::sync::OnceLock;
#[allow(unused_imports)]
use std::{cell::RefCell, mem::size_of};

use self::{
    gpio::SimGPIODriver,
    rt::{
        station_interface::{StationInterfaceDriver, StationInterfaceVTable},
        watchdog::SimWatchdogDriver,
    },
};
#[allow(unused_imports)]
use self::{
    gpio::{GPIODriver, GPIOVTable},
    rt::{
        notifier::{NotifierDriver, NotifierVTable},
        time::{initialize_time_callbacks, ClockDriver},
        watchdog::{WatchdogDriver, WatchdogVTable},
    },
};

#[cfg(not(test))]
static HAL_INSTANCE: OnceLock<HAL> = OnceLock::new();

#[cfg(test)]
thread_local! {
    static HAL_INSTANCE_LOCAL: RefCell<Option<HAL>> = RefCell::new(None);
}

/// A trait that defines a HAL(Hardware Abstraction Layer) Driver.
/// Should be implemented by a platform specific HAL Driver Zero Sized Type.
pub trait HALDriver:
    GPIODriver
    + ClockDriver
    + NotifierDriver
    + WatchdogDriver
    + StationInterfaceDriver
    /*
    + CanDriver
    + SpiDriver
    + I2cDriver
    + UartDriver
    */
    + 'static {
    /// The name of the driver, used for logging/debugging.
    const NAME: &'static str;

    /// Will only be called once per program execution.
    /// Can be used to setup global state for the driver.
    fn init();

    /// Will only be called once per program execution but is not guaranteed to be called.
    /// Will always be called after [`HALDriver::init`].
    /// Can be used to cleanup global state for the driver.
    fn cleanup();
}

/// A trait that defines a HAL(Hardware Abstraction Layer) Driver with sim support.
/// Should be implemented by a platform specific HAL Driver Zero Sized Type.
///
/// A HAL with sim support should implement this trait instead of [`HALDriver`],
/// because of this a HAL with sim support can be used in as a [`HALDriver`] too.
pub trait SimHALDriver: HALDriver + SimGPIODriver + SimWatchdogDriver {}

/// A struct that defines a platform specific HAL(Hardware Abstraction Layer) using a Driver.
///
/// Using a HAL allows for significant platform changes without having to rewrite much code.
/// HALs are also useful for testing/sim as they can be mocked.
///
/// HALs also allow for easier community support for exotic platforms.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
pub struct HAL {
    driver_name: &'static str,
    cleanup: fn(),
    gpio: GPIOVTable,
    notifier: NotifierVTable,
    watchdog: WatchdogVTable,
    station_interface: StationInterfaceVTable,
}

impl HAL {
    /// Initializes the HAL, can only be called once across every HAL.
    ///
    /// # Panics
    /// If HAL has already been initialized this will panic
    /// If the used driver is not zero sized this will panic
    pub fn init<Driver: HALDriver>() {
        assert!(size_of::<Driver>() == 0, "Driver must be zero sized");
        Driver::init();
        initialize_time_callbacks::<Driver>(Driver::NAME);
        Self {
            driver_name: Driver::NAME,
            cleanup: Driver::cleanup,
            gpio: GPIOVTable::from_driver::<Driver>(),
            notifier: NotifierVTable::from_driver::<Driver>(),
            watchdog: WatchdogVTable::from_driver::<Driver>(),
            station_interface: StationInterfaceVTable::from_driver::<Driver>(),
        }
        .set_hal();
    }

    /// Initializes the HAL with support for sim, can only be called once across every HAL.
    ///
    /// # Panics
    /// If HAL has already been initialized this will panic.
    /// If the used driver is not zero sized this will panic.
    pub fn init_sim<Driver: SimHALDriver>() {
        assert!(size_of::<Driver>() == 0, "Driver must be zero sized");
        Driver::init();
        #[cfg(not(test))]
        initialize_time_callbacks::<Driver>(Driver::NAME);
        Self {
            driver_name: Driver::NAME,
            cleanup: Driver::cleanup,
            gpio: GPIOVTable::from_sim_driver::<Driver>(),
            notifier: NotifierVTable::from_driver::<Driver>(),
            watchdog: WatchdogVTable::from_sim_driver::<Driver>(),
            station_interface: StationInterfaceVTable::from_driver::<Driver>(),
        }
        .set_hal();
    }

    fn set_hal(self) {
        #[cfg(not(test))]
        {
            assert!(
                HAL_INSTANCE.get().is_none(),
                "HAL has already been initialized"
            );
            let _ = HAL_INSTANCE.set(self);
        }
        #[cfg(test)]
        {
            assert!(
                HAL_INSTANCE_LOCAL.with(|inst| inst.borrow().is_none()),
                "HAL has already been initialized"
            );
            HAL_INSTANCE_LOCAL.with(|local_hal_instance| {
                *local_hal_instance.borrow_mut() = Some(self);
            });
        }
    }

    /// Cleans up the HAL
    pub fn cleanup(&self) {
        (self.cleanup)();
    }

    /// Returns the name of the current [`HALDriver`].
    #[must_use]
    pub const fn driver_name(&self) -> &'static str {
        self.driver_name
    }

    /// Returns the [`NotifierVTable`] for the current [`HALDriver`].
    #[must_use]
    pub const fn notifier_api(&self) -> NotifierVTable {
        self.notifier
    }

    /// Returns the [`WatchdogVTable`] for the current [`HALDriver`].
    #[must_use]
    pub const fn watchdog_api(&self) -> WatchdogVTable {
        self.watchdog
    }

    /// Returns the [`GPIOVTable`] for the current [`HALDriver`].
    #[must_use]
    pub const fn gpio_api(&self) -> GPIOVTable {
        self.gpio
    }

    /// Returns the [`StationInterfaceVTable`] for the current [`HALDriver`].
    #[must_use]
    pub const fn station_interface_api(&self) -> StationInterfaceVTable {
        self.station_interface
    }
}

/// Returns the current [`HAL`] instance.
///
/// # Errors
///  - [`HALNotInitializedError`] if the [`HAL`] has not been initialized
pub fn get_hal() -> Result<HAL, HALNotInitializedError> {
    #[cfg(not(test))]
    {
        HAL_INSTANCE.get().copied().ok_or(HALNotInitializedError)
    }
    #[cfg(test)]
    {
        HAL_INSTANCE_LOCAL
            .with(|local_hal_instance| *local_hal_instance.borrow())
            .ok_or(HALNotInitializedError)
    }
}

/// An error that occurs when a function is called that is only available when the [`HAL`] is initialized with sim support
/// but the [`HAL`] is not initialized with sim support.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotSimError;
impl std::fmt::Display for NotSimError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "This function is only available when the HAL is initialized with sim support"
        )
    }
}
impl std::error::Error for NotSimError {}

/// An error that occurs when the [`HAL`] has not been initialized but is attempted to be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HALNotInitializedError;
impl std::fmt::Display for HALNotInitializedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`HAL` has not been initialized")
    }
}
impl std::error::Error for HALNotInitializedError {}

// #[doc(hidden)]
// pub mod __private {
//     pub trait ZST {}
// }
//
// /// Validates that a driver is zero size
// #[macro_export]
// macro_rules! assert_driver_zst {
//     ($driver:ty) => {
//         const _: fn() = || {
//             let _ = core::mem::transmute::<$driver, ()>;
//         };
//         impl $crate::hal::__private::ZST for $driver {}
//     };
// }
