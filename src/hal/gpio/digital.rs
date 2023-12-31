use super::Channel;


pub trait DigitalInputChannel: Channel {
    /// Reads the current pull of the channel.
    fn read(&self) -> bool;
}
pub type DigitalInput = Box<dyn DigitalInputChannel>;

pub trait DigitalOutputChannel: Channel {
    /// Sets the current pull of the channel.
    fn write(&mut self, value: bool);
}
pub type DigitalOutput = Box<dyn DigitalOutputChannel>;

pub trait DigitalBiChannel: DigitalInputChannel + DigitalOutputChannel {}
pub type DigitalBi = Box<dyn DigitalBiChannel>;