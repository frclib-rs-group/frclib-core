//! Real Time Notifier HAL Driver

use crate::units::time::Microsecond;


/// The type of update to perform on the alarm
#[derive(Debug, Clone, Copy)]
pub enum NotifierUpdateType {
    /// Will set alarm to trigger every X microseconds.
    /// If canceled the alarm will stop re-priming and the current alarm will be removed
    Periodic{
        /// The period to trigger at
        period: Microsecond,
        /// If true, if an entire alarm is missed it will be ignored.
        /// If false, missed alarms will build up and trigger as soon as possible until caught up.
        skip_missed: bool,
    },
    /// Will set alarm to trigger at the specified time.
    /// If canceled the current alarm will be removed
    OneShot{
        /// The time to trigger at
        trigger_time: Microsecond,
    },
    /// Will set alarm to trigger at the specified time relative to the current time.
    /// If canceled the current alarm will be removed
    RelativeOneShot{
        /// The time from now to trigger at
        trigger_offset_time: Microsecond,
    },
}

/// A trait that represents a platform specific alarm notifier handle.
pub trait Notifier: Send + Sync {
    /// Returns the name of the notifier for debugging purposes
    fn get_name(&self) -> &str;

    /// Updates the alarm to trigger according to the specified update type
    fn update_alarm(&mut self, update: NotifierUpdateType);
    /// Cancels the alarm
    fn cancel_alarm(&mut self);
    /// Returns the time the alarm was triggered
    fn wait_for_alarm(&mut self) -> Microsecond;
}
type NotifierHandle = Box<dyn Notifier>;


/// A platform specific alarm notifier driver.
/// 
/// # Implementation
/// Most of the work for this driver involves setting up the interrupt logic.
/// It's reccomended for the notifier thread to be created on first use of the driver
/// and have the thread be high priority.
/// 
/// # Safety
/// ## Thread Safety
/// It's up to the driver developer to ensure that the driver is thread safe.
/// 
/// # Development Resources
/// - [WPILib Rio Notifier](https://github.com/wpilibsuite/allwpilib/blob/main/hal/src/main/native/athena/Notifier.cpp)
/// - [WPILib Sim Notifier](https://github.com/wpilibsuite/allwpilib/blob/main/hal/src/main/native/sim/Notifier.cpp)
/// - [Spin Sleep](https://github.com/alexheretic/spin-sleep)
/// - [Condition Variable](`std::sync::Condvar`)
pub trait NotifierDriver: 'static {
    /// Creates a new notifier
    fn new_notifier() -> impl Notifier;
}

/// A platform specific alarm notifier driver vtable.
#[derive(Debug, Clone, Copy)]
pub struct NotifierVTable {
    pub(crate) new_notifier: fn() -> NotifierHandle,
}
impl NotifierVTable {
    pub(crate) fn from_driver<T: NotifierDriver>() -> Self {
        assert!(std::mem::size_of::<T>() == 0, "Notifier Driver must be zero sized");
        Self {
            new_notifier: || Box::new(T::new_notifier()),
        }
    }
    /// Creates a new notifier
    #[must_use]
    pub fn new_notifier(&self) -> NotifierHandle {
        (self.new_notifier)()
    }
}