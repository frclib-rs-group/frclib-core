//! Digital GPIO functionality and traits.

use super::Channel;

/// A variety of [``Channel``](super::Channel) that can read digital data.
pub trait DigitalInputChannel: Channel {
    /// Reads the current pull of the channel.
    fn read(&self) -> bool;
}
/// A type represting a type erased [``DigitalInputChannel``](DigitalInputChannel).
pub type DigitalInput = Box<dyn DigitalInputChannel>;

/// A variety of [``Channel``](super::Channel) that can write digital data.
pub trait DigitalOutputChannel: Channel {
    /// Sets the current pull of the channel.
    fn write(&mut self, value: bool);
}
/// A type represting a type erased [``DigitalOutputChannel``](DigitalOutputChannel).
pub type DigitalOutput = Box<dyn DigitalOutputChannel>;


/// A variety of [``Channel``](super::Channel) that can read and write digital data.
pub trait DigitalBiChannel: DigitalInputChannel + DigitalOutputChannel {}

/// A type represting a type erased [``DigitalBiChannel``](DigitalBiChannel).
pub type DigitalBi = Box<dyn DigitalBiChannel>;

/// Implement [``DigitalBiChannel``](DigitalBiChannel) for all types that implement
/// [``DigitalInputChannel``](DigitalInputChannel) and [``DigitalOutputChannel``](DigitalOutputChannel).
impl<T: DigitalInputChannel + DigitalOutputChannel> DigitalBiChannel for T {}
