use crate::units::energy::Volt;

use super::{Channel, GPIOError, GPIOPortType};



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
pub type AnalogInput = Box<dyn AnalogInputChannel>;


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
pub type AnalogOutput = Box<dyn AnalogOutputChannel>;


pub trait AnalogBiChannel: AnalogInputChannel + AnalogOutputChannel {}
pub type AnalogBi = Box<dyn AnalogBiChannel>;

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