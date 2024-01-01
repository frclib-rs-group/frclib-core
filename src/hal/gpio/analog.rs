//! Analog GPIO functionality and traits.

use crate::units::energy::Volt;

use super::{Channel, GPIOError, GPIOPortType};


/// A variety of [``Channel``](super::Channel) that can read analog data.
pub trait AnalogInputChannel: Channel {
    /// Reads the raw digital data coming from the channel.
    /// no gurantee it can use all 32 bits, for example the `RoboRio`
    /// only has a 12-bit resolution.
    fn read_raw(&self) -> u32;

    /// Returns the current voltage on the channel
    /// with as little computation as possible(aka. little to no averaging/filtering).
    fn read_volts_instant(&self) -> Volt;

    /// Returns the current voltage on the channel
    fn read_volts(&self) -> Volt;

    /// Returns the voltage range of the input
    fn voltage_range(&self) -> (Volt, Volt);
}
/// A type represting a type erased [``AnalogInputChannel``](AnalogInputChannel).
pub type AnalogInput = Box<dyn AnalogInputChannel>;


/// A variety of [``Channel``](super::Channel) that can write analog data.
pub trait AnalogOutputChannel: Channel {
    /// Writes the raw digital data to the channel.
    /// no gurantee it can use all 32 bits, for example the `RoboRio`
    /// only has a 12-bit resolution.
    fn write_raw(&mut self, value: u32);

    /// Writes the voltage to the channel.
    fn write_volts(&mut self, value: Volt);

    /// Returns the voltage range of the output
    fn voltage_range(&self) -> (Volt, Volt);
}
/// A type represting a type erased [``AnalogOutputChannel``](AnalogOutputChannel).
pub type AnalogOutput = Box<dyn AnalogOutputChannel>;


/// A variety of [``Channel``](super::Channel) that can read and write analog data.
pub trait AnalogBiChannel: AnalogInputChannel + AnalogOutputChannel {}

/// A type represting a type erased [``AnalogBiChannel``](AnalogBiChannel).
pub type AnalogBi = Box<dyn AnalogBiChannel>;

/// Implement [``AnalogBiChannel``](AnalogBiChannel) for all types that implement
/// [``AnalogInputChannel``](AnalogInputChannel) and [``AnalogOutputChannel``](AnalogOutputChannel).
impl<T: AnalogInputChannel + AnalogOutputChannel> AnalogBiChannel for T {}

/// Checks if the specified port is available for analog use.
/// Has a sensible default implementation but can be overriden if needed.
#[allow(clippy::missing_errors_doc)]
pub fn inside_analog_range(port: u8, volts: Volt) -> Result<(), GPIOError> {
    if (0.0..=5.0).contains(&volts.value()) {
        Ok(())
    } else {
        Err(GPIOError::ValueOutOfRange(
            volts,
            GPIOPortType::Analog,
            port,
        ))
    }
}