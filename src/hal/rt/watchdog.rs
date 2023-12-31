use crate::{units::energy::Volt, hal::NotSimError};

pub trait WatchdogDriver: 'static {
    /// Gets whether the watchdog is enabled
    fn enabled() -> bool;
    /// Gets the systems voltage
    fn system_power() -> Volt;
}

pub trait SimWatchdogDriver: WatchdogDriver {
    /// Sets whether the watchdog is enabled
    fn set_enabled(enabled: bool);
    /// Sets the systems voltage
    fn set_system_power(voltage: Volt);
}


#[derive(Debug, Clone, Copy)]
pub struct WatchdogVTable {
    pub(crate) enabled: fn() -> bool,
    pub(crate) system_power: fn() -> Volt,
    //sim
    pub(crate) set_enabled: Option<fn(enabled: bool)>,
    pub(crate) set_system_power: Option<fn(voltage: Volt)>,
}
impl WatchdogVTable {
    pub(crate) fn from_driver<T: WatchdogDriver>() -> Self {
        assert!(std::mem::size_of::<T>() == 0, "Watchdog Driver must be zero sized");
        Self {
            enabled: T::enabled,
            system_power: T::system_power,
            set_enabled: None,
            set_system_power: None,
        }
    }

    pub(crate) fn from_sim_driver<T: SimWatchdogDriver>() -> Self {
        assert!(std::mem::size_of::<T>() == 0, "Watchdog Driver must be zero sized");
        Self {
            enabled: T::enabled,
            system_power: T::system_power,
            set_enabled: Some(T::set_enabled),
            set_system_power: Some(T::set_system_power),
        }
    }

    /// Gets whether the watchdog is enabled
    #[must_use]
    pub fn enabled(&self) -> bool {
        (self.enabled)()
    }

    /// Gets the systems voltage
    #[must_use]
    pub fn system_power(&self) -> Volt {
        (self.system_power)()
    }

    /// If initialized with sim support, sets whether the watchdog is enabled.
    /// Returns `Err` if not initialized with sim support
    #[allow(clippy::missing_errors_doc)]
    pub fn set_enabled(&self, enabled: bool) -> Result<(), NotSimError> {
        self.set_enabled.map_or(Err(NotSimError), |set_enabled| {
            set_enabled(enabled);
            Ok(())
        })
    }

    /// If initialized with sim support, sets the systems voltage.
    /// Returns `Err` if not initialized with sim support
    #[allow(clippy::missing_errors_doc)]
    pub fn set_system_power(&self, voltage: Volt) -> Result<(), NotSimError> {
        self.set_system_power.map_or(Err(NotSimError), |set_system_power| {
            set_system_power(voltage);
            Ok(())
        })
    }
}