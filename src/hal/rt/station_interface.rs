//! A platform and driver station independent interface for interacting with the user control station.

/// The commanded enable state of the robot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EnabledState {
    /// The robot is unable to actuate any mechanisms.
    #[default]
    Disabled,
    /// Same as [`Disabled`](EnabledState::Disabled) but the robot has to be restarted to re-enable.
    EStopped,
    /// The robot is able to actuate mechanisms.
    Enabled,
}

/// The commanded mode of the robot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Mode {
    /// The is being controlled by a user directly.
    #[default]
    Teleop,
    /// The robot is being controlled by a user code autonomously.
    Auto,
    /// The robot is being controlled by a user directly for testing purposes.
    Test,
}

/// The data for a single joystick.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct JoystickData {
    /// If the joystick is plugged in.
    pub plugged_in: bool,
    /// An array of the joystick axies,
    /// this can be lossy if the joystick has more than 8 axies.
    pub axies: [f32; 8],
    /// An array of the joystick buttons,
    /// this can be lossy if the joystick has more than 32 buttons.
    pub buttons: u32,
    /// An array of the joystick povs,
    /// this can be lossy if the joystick has more than 4 povs.
    pub povs: [i16; 4],
}

/// The data for the current station.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Default)]
pub struct StationData {
    pub enabled_state: EnabledState,
    pub mode: Mode,
    pub station_attached: bool,
    pub fms_attached: bool,
    pub team_number: u16,
    pub joysticks: [JoystickData; 8],
}

/// A trait that represents a platform specific user control station interface.
pub trait StationInterfaceDriver: 'static {
    /// Will send an error message to the driver station console,
    /// driver station has to support error messages.
    fn send_console_line_error(line: &str, location: &str, callstack: &str);

    /// Will send a warning message to the driver station console,
    /// if the driver station does not support warnings it will be sent as an error.
    fn send_console_line_warn(line: &str, location: &str);

    /// Will send an info message to the driver station console,
    /// driver station has to support info messages.
    fn send_console_line_info(line: &str, location: &str);

    /// Will send a debug message to the driver station console,
    /// if the driver station does not support debug messages it will be ignored.
    fn send_console_line_debug(line: &str, location: &str);

    /// Should update all process local station data from the driver station latest values.
    fn refresh();

    /// Should return the latest station data.
    fn get_station_data() -> StationData;
}

/// A struct that defines a platform specific user control station interface for user code.
#[derive(Debug, Clone, Copy)]
pub struct StationInterfaceVTable {
    pub(crate) send_console_line_error: fn(line: &str, location: &str, callstack: &str),
    pub(crate) send_console_line_warn: fn(line: &str, location: &str),
    pub(crate) send_console_line_info: fn(line: &str, location: &str),
    pub(crate) send_console_line_debug: fn(line: &str, location: &str),
    pub(crate) refresh: fn(),
    pub(crate) get_station_data: fn() -> StationData,
}

impl StationInterfaceVTable {
    pub(crate) fn from_driver<T: StationInterfaceDriver>() -> Self {
        assert!(
            std::mem::size_of::<T>() == 0,
            "Station Interface Driver must be zero sized"
        );
        Self {
            send_console_line_error: T::send_console_line_error,
            send_console_line_warn: T::send_console_line_warn,
            send_console_line_info: T::send_console_line_info,
            send_console_line_debug: T::send_console_line_debug,
            refresh: T::refresh,
            get_station_data: T::get_station_data,
        }
    }

    /// Will send an error message to the driver station console,
    /// driver station has to support error messages.
    pub fn send_console_line_error(&self, line: &str, location: &str, callstack: &str) {
        (self.send_console_line_error)(line, location, callstack);
    }

    /// Will send a warning message to the driver station console,
    /// if the driver station does not support warnings it will be sent as an error.
    pub fn send_console_line_warn(&self, line: &str, location: &str) {
        (self.send_console_line_warn)(line, location);
    }

    /// Will send an info message to the driver station console,
    /// driver station has to support info messages.
    pub fn send_console_line_info(&self, line: &str, location: &str) {
        (self.send_console_line_info)(line, location);
    }

    /// Will send a debug message to the driver station console,
    /// if the driver station does not support debug messages it will be ignored.
    pub fn send_console_line_debug(&self, line: &str, location: &str) {
        (self.send_console_line_debug)(line, location);
    }

    /// Should update all process local station data from the driver station latest values.
    pub fn refresh(&self) {
        (self.refresh)();
    }

    /// Should return the latest station data.
    #[must_use]
    pub fn get_station_data(&self) -> StationData {
        (self.get_station_data)()
    }

    /// Returns if the latest station data declares the robot enabled.
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.get_station_data().enabled_state == EnabledState::Enabled
    }

    /// Returns if the latest station data declares the robot disabled.
    #[must_use]
    pub fn is_disabled(&self) -> bool {
        self.get_station_data().enabled_state == EnabledState::Disabled
    }

    /// Returns if the latest station data declares the robot estopped.
    #[must_use]
    pub fn is_estopped(&self) -> bool {
        self.get_station_data().enabled_state == EnabledState::EStopped
    }

    /// Returns if the latest station data declares the robot in teleop mode.
    #[must_use]
    pub fn is_teleop(&self) -> bool {
        self.get_station_data().mode == Mode::Teleop
    }

    /// Returns if the latest station data declares the robot in auto mode.
    #[must_use]
    pub fn is_auto(&self) -> bool {
        self.get_station_data().mode == Mode::Auto
    }

    /// Returns if the latest station data declares the robot in test mode.
    #[must_use]
    pub fn is_test(&self) -> bool {
        self.get_station_data().mode == Mode::Test
    }

    /// Returns if the latest station data declares the robot is attached to the FMS.
    #[must_use]
    pub fn is_fms_attached(&self) -> bool {
        self.get_station_data().fms_attached
    }

    /// Returns if the latest station data declares the robot is attached to a station.
    #[must_use]
    pub fn is_station_attached(&self) -> bool {
        self.get_station_data().station_attached
    }

    /// Returns the team number of the latest station data.
    #[must_use]
    pub fn get_station_team_number(&self) -> u16 {
        self.get_station_data().team_number
    }
}
