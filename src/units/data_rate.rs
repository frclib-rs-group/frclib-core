use crate::units::data::{Byte, Gigabyte, Kilobyte, Megabyte};
use crate::units::time::Second;
use crate::{unit, unit_conversion, unit_family};

unit!(BytesPerSecond: float);
unit!(KilobytesPerSecond: float);
unit!(MegabytesPerSecond: float);
unit!(GigabytesPerHour: float);

unit_conversion!(BytesPerSecond(float) <-> KilobytesPerSecond(float) ~ byte_per_second_to_kilobyte_per_second);
unit_conversion!(BytesPerSecond(float) <-> MegabytesPerSecond(float) ~ byte_per_second_to_megabyte_per_second);
unit_conversion!(BytesPerSecond(float) <-> GigabytesPerHour(float) ~ byte_per_second_to_gigabyte_per_hour);
unit_conversion!(KilobytesPerSecond(float) <-> MegabytesPerSecond(float) ~ kilobyte_per_second_to_megabyte_per_second);
unit_conversion!(KilobytesPerSecond(float) <-> GigabytesPerHour(float) ~ kilobyte_per_second_to_gigabyte_per_hour);
unit_conversion!(MegabytesPerSecond(float) <-> GigabytesPerHour(float) ~ megabyte_per_second_to_gigabyte_per_hour);

unit_family!(DataRate(BytesPerSecond): KilobytesPerSecond, MegabytesPerSecond, GigabytesPerHour);

fn byte_per_second_to_kilobyte_per_second(byte_per_second: f64) -> f64 {
    byte_per_second / 1000.0
}

fn byte_per_second_to_megabyte_per_second(byte_per_second: f64) -> f64 {
    byte_per_second / 1_000_000.0
}

fn byte_per_second_to_gigabyte_per_hour(byte_per_second: f64) -> f64 {
    byte_per_second * 0.000_036
}

fn kilobyte_per_second_to_megabyte_per_second(kilobyte_per_second: f64) -> f64 {
    kilobyte_per_second / 1000.0
}

fn kilobyte_per_second_to_gigabyte_per_hour(kilobyte_per_second: f64) -> f64 {
    kilobyte_per_second * 3600.0 / 1_000_000.0
}

fn megabyte_per_second_to_gigabyte_per_hour(megabyte_per_second: f64) -> f64 {
    megabyte_per_second * 3600.0 / 1000.0
}

impl BytesPerSecond {
    #[must_use]
    pub fn to_byte(&self, seconds: Second) -> Byte {
        Byte::new(self.value() * seconds.value())
    }
}

impl KilobytesPerSecond {
    #[must_use]
    pub fn to_kilobyte(&self, seconds: Second) -> Kilobyte {
        Kilobyte::new(self.value() * seconds.value())
    }
}

impl MegabytesPerSecond {
    #[must_use]
    pub fn to_megabyte(&self, seconds: Second) -> Megabyte {
        Megabyte::new(self.value() * seconds.value())
    }
}

impl GigabytesPerHour {
    #[must_use]
    pub fn to_gigabyte(&self, seconds: Second) -> Gigabyte {
        Gigabyte::new(self.value() / (3600.0 / seconds.value()))
    }
}
