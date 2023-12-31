pub mod analog;
pub mod digital;

use std::fmt::Display;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GPIOPortType {
    Analog,
    Digital,
    PWM,
}
impl Display for GPIOPortType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Analog => write!(f, "Analog Port"),
            Self::Digital => write!(f, "Digital Port"),
            Self::PWM => write!(f, "PWM Port"),
        }
    }
}

#[allow(variant_size_differences)]
#[non_exhaustive]
#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq)]
pub enum GPIOError {
    #[error("GPIO port {0} is not available")]
    PortNotAvailable(u8),
    #[error("Value {0} is out of range for {1} port {2}")]
    ValueOutOfRange(Volt, GPIOPortType, u8),
    #[error("Wrong mode for {0} port {1}, expected output to be {2}")]
    WrongMode(GPIOPortType, u8, bool),
    #[error("GPIO port {0} is already in use")]
    PortInUse(u8),
}

pub trait Channel {
    fn channel_id(&self) -> u8;
}

use analog::{AnalogInput, AnalogOutput};
use digital::{DigitalInput, DigitalOutput};

use crate::units::energy::Volt;

use super::NotSimError;

/// A platform specific GPIO driver.
///
/// # Ports
/// ## Identifying Ports
/// Ports are identified by a `u8` value. The meaning of this value is up to driver developer but
/// it is recommended to use the following convention of the ``NI RoboRio`` of starting at 0 and
/// incrementing by 1 for each port. This is not enforced by the HAL.
///
/// ## Port Types
/// There are three types of ports: analog, digital, and PWM.
/// If the platform allows it one port can be multiple types.
/// This means that id 0 could mean a different physical pin/port depending on the type
/// or that a port could be both analog and digital and setting the analog value would overwrite the digital value.
///
/// ### Analog
/// Analog ports are ports that can read a voltage value from 0 to 5 volts.
/// If the platform supports a different range of voltages it is recommended to scale the value.
///
/// ### Digital
/// Digital ports are ports that can read a boolean value.
///
/// ### PWM
/// PWM ports are ports that can write a value from 0 to 1.
///
/// # Safety
/// ## Thread Safety
/// It's up to the driver developer to ensure that the driver is thread safe.
///
/// # Development Resources
/// - [WPILib Rio DIO](https://github.com/wpilibsuite/allwpilib/blob/main/hal/src/main/native/athena/DIO.cpp)
/// - [WPILib Sim DIO](https://github.com/wpilibsuite/allwpilib/blob/main/hal/src/main/native/sim/DIO.cpp)
/// - [WPILib Rio AnalogIn](https://github.com/wpilibsuite/allwpilib/blob/main/hal/src/main/native/athena/AnalogInput.cpp)
/// - [WPILib Sim AnalogIn](https://github.com/wpilibsuite/allwpilib/blob/main/hal/src/main/native/sim/AnalogInput.cpp)
/// - [WPILib Rio AnalogOut](https://github.com/wpilibsuite/allwpilib/blob/main/hal/src/main/native/athena/AnalogOutput.cpp)
/// - [WPILib Sim AnalogOut](https://github.com/wpilibsuite/allwpilib/blob/main/hal/src/main/native/sim/AnalogOutput.cpp)
/// - [Raspberry Pi GPIO](https://github.com/golemparts/rppal/tree/master)
/// - [Jetson GPIO](https://github.com/Kajatin/jetson-gpio-rust/tree/main)
pub trait GPIODriver {
    const ANALOG_PORTS: &'static [u8];
    const DIGITAL_PORTS: &'static [u8];
    const PWM_PORTS: &'static [u8];
    const RELAY_PORTS: &'static [u8];

    //
    // # Channels
    //

    /// Returns a new analog input for the specified port.
    /// 
    /// # Errors
    /// - [`GPIOError::PortNotAvailable`] if the port is not available for analog in use
    /// - [`GPIOError::PortInUse`] if the port is already in use
    fn new_analog_input(port: u8) -> Result<AnalogInput, GPIOError>;

    /// Returns a new analog output for the specified port.
    ///
    /// # Errors
    /// - [`GPIOError::PortNotAvailable`] if the port is not available for analog out use
    /// - [`GPIOError::PortInUse`] if the port is already in use
    fn new_analog_output(port: u8) -> Result<AnalogOutput, GPIOError>;

    /// Returns a new digital input for the specified port.
    ///
    /// # Errors
    /// - [`GPIOError::PortNotAvailable`] if the port is not available for digital in use
    /// - [`GPIOError::PortInUse`] if the port is already in use
    fn new_digital_input(port: u8) -> Result<DigitalInput, GPIOError>;

    /// Returns a new digital output for the specified port.
    /// 
    /// # Errors
    /// - [`GPIOError::PortNotAvailable`] if the port is not available for digital out use
    /// - [`GPIOError::PortInUse`] if the port is already in use
    fn new_digital_output(port: u8) -> Result<DigitalOutput, GPIOError>;

    //
    // # Checks
    //

    /// Checks if the specified port is available for analog use.
    /// Has a sensible default implementation but can be overriden if needed.
    #[allow(clippy::missing_errors_doc)]
    fn analog_available(port: u8) -> Result<(), GPIOError> {
        if Self::ANALOG_PORTS.contains(&port) {
            Ok(())
        } else {
            Err(GPIOError::PortNotAvailable(port))
        }
    }

    /// Checks if the specified port is available for digital use.
    /// Has a sensible default implementation but can be overriden if needed.
    #[allow(clippy::missing_errors_doc)]
    fn digital_available(port: u8) -> Result<(), GPIOError> {
        if Self::DIGITAL_PORTS.contains(&port) {
            Ok(())
        } else {
            Err(GPIOError::PortNotAvailable(port))
        }
    }

    /// Checks if the specified port is available for PWM use.
    /// Has a sensible default implementation but can be overriden if needed.
    #[allow(clippy::missing_errors_doc)]
    fn pwm_available(port: u8) -> Result<(), GPIOError> {
        if Self::PWM_PORTS.contains(&port) {
            Ok(())
        } else {
            Err(GPIOError::PortNotAvailable(port))
        }
    }
}


pub trait SimGPIODriver: GPIODriver {
    /// Returns the other end of the analog input port as an [`AnalogOutput`].
    fn sim_analog_input(port: u8) -> AnalogOutput;

    /// Returns the other end of the analog output port as an [`AnalogInput`].
    fn sim_analog_output(port: u8) -> AnalogInput;

    /// Returns the other end of the digital input port as an [`DigitalOutput`].
    fn sim_digital_input(port: u8) -> DigitalOutput;

    /// Returns the other end of the digital output port as an [`DigitalInput`].
    fn sim_digital_output(port: u8) -> DigitalInput;
}



#[derive(Debug, Clone, Copy)]
pub struct GPIOVTable {
    pub(crate) new_analog_input: fn(u8) -> Result<AnalogInput, GPIOError>,
    pub(crate) new_analog_output: fn(u8) -> Result<AnalogOutput, GPIOError>,
    pub(crate) new_digital_input: fn(u8) -> Result<DigitalInput, GPIOError>,
    pub(crate) new_digital_output: fn(u8) -> Result<DigitalOutput, GPIOError>,
    //sim
    pub(crate) sim_analog_input: Option<fn(u8) -> AnalogOutput>,
    pub(crate) sim_analog_output: Option<fn(u8) -> AnalogInput>,
    pub(crate) sim_digital_input: Option<fn(u8) -> DigitalOutput>,
    pub(crate) sim_digital_output: Option<fn(u8) -> DigitalInput>,
}

impl GPIOVTable {
    pub(crate) fn from_driver<T: GPIODriver>() -> Self {
        assert!(std::mem::size_of::<T>() == 0, "GPIO Driver must be zero sized");
        Self {
            new_analog_input: T::new_analog_input,
            new_analog_output: T::new_analog_output,
            new_digital_input: T::new_digital_input,
            new_digital_output: T::new_digital_output,
            sim_analog_input: None,
            sim_analog_output: None,
            sim_digital_input: None,
            sim_digital_output: None,
        }
    }

    pub(crate) fn from_sim_driver<T: SimGPIODriver>() -> Self {
        assert!(std::mem::size_of::<T>() == 0, "GPIO Driver must be zero sized");
        Self {
            new_analog_input: T::new_analog_input,
            new_analog_output: T::new_analog_output,
            new_digital_input: T::new_digital_input,
            new_digital_output: T::new_digital_output,
            sim_analog_input: Some(T::sim_analog_input),
            sim_analog_output: Some(T::sim_analog_output),
            sim_digital_input: Some(T::sim_digital_input),
            sim_digital_output: Some(T::sim_digital_output),
        }
    }

    /// Returns a new analog input for the specified port.
    /// 
    /// # Errors
    /// - [`GPIOError::PortNotAvailable`] if the port is not available for analog use
    /// - [`GPIOError::PortInUse`] if the port is already in use
    pub fn new_analog_input(&self, port: u8) -> Result<AnalogInput, GPIOError> {
        (self.new_analog_input)(port)
    }

    /// Returns a new analog output for the specified port.
    ///
    /// # Errors
    /// - [`GPIOError::PortNotAvailable`] if the port is not available for analog use
    /// - [`GPIOError::PortInUse`] if the port is already in use
    pub fn new_analog_output(&self, port: u8) -> Result<AnalogOutput, GPIOError> {
        (self.new_analog_output)(port)
    }

    /// Returns a new digital input for the specified port.
    ///
    /// # Errors
    /// - [`GPIOError::PortNotAvailable`] if the port is not available for digital in use
    /// - [`GPIOError::PortInUse`] if the port is already in use
    pub fn new_digital_input(&self, port: u8) -> Result<DigitalInput, GPIOError> {
        (self.new_digital_input)(port)
    }

    /// Returns a new digital output for the specified port.
    /// 
    /// # Errors
    /// - [`GPIOError::PortNotAvailable`] if the port is not available for digital out use
    /// - [`GPIOError::PortInUse`] if the port is already in use
    pub fn new_digital_output(&self, port: u8) -> Result<DigitalOutput, GPIOError> {
        (self.new_digital_output)(port)
    }

    /// Returns the other end of the analog input port as an [`AnalogOutput`] if initialized as sim.
    /// If not sim this function will return `Err(NotSimError)`
    #[allow(clippy::missing_errors_doc)]
    pub fn sim_analog_input(&self, port: u8) -> Result<AnalogOutput, NotSimError> {
        self.sim_analog_input.map_or(
            Err(NotSimError),
            |f| Ok((f)(port))
        )
    }

    /// Returns the other end of the analog output port as an [`AnalogInput`] if initialized as sim.
    /// If not sim this function will return `Err(NotSimError)`
    #[allow(clippy::missing_errors_doc)]
    pub fn sim_analog_output(&self, port: u8) -> Result<AnalogInput, NotSimError> {
        self.sim_analog_output.map_or(
            Err(NotSimError),
            |f| Ok((f)(port))
        )
    }

    /// Returns the other end of the digital input port as an [`DigitalOutput`] if initialized as sim.
    /// If not sim this function will return `Err(NotSimError)`
    #[allow(clippy::missing_errors_doc)]
    pub fn sim_digital_input(&self, port: u8) -> Result<DigitalOutput, NotSimError> {
        self.sim_digital_input.map_or(
            Err(NotSimError),
            |f| Ok((f)(port))
        )
    }

    /// Returns the other end of the digital output port as an [`DigitalInput`] if initialized as sim.
    /// If not sim this function will return `Err(NotSimError)`
    #[allow(clippy::missing_errors_doc)]
    pub fn sim_digital_output(&self, port: u8) -> Result<DigitalInput, NotSimError> {
        self.sim_digital_output.map_or(
            Err(NotSimError),
            |f| Ok((f)(port))
        )
    }
}
