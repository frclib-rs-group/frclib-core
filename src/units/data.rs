use crate::units::data_rate::{
    BytesPerSecond, GigabytesPerHour, KilobytesPerSecond, MegabytesPerSecond,
};
use crate::units::time::Second;
use frclib_units_macros::{unit, unit_conversion};

unit!(Byte, f64);
unit!(Kilobyte, f64);
unit!(Megabyte, f64);
unit!(Gigabyte, f64);

unit_conversion!(Byte f64, Kilobyte f64, byte_to_kilobyte);
unit_conversion!(Byte f64, Megabyte f64, byte_to_megabyte);
unit_conversion!(Byte f64, Gigabyte f64, byte_to_gigabyte);
unit_conversion!(Kilobyte f64, Megabyte f64, kilobyte_to_megabyte);
unit_conversion!(Kilobyte f64, Gigabyte f64, kilobyte_to_gigabyte);
unit_conversion!(Megabyte f64, Gigabyte f64, megabyte_to_gigabyte);

// unit_family!(Data: Byte Kilobyte Megabyte Gigabyte);

fn byte_to_kilobyte(byte: f64) -> f64 {
    byte / 1000.0
}

fn byte_to_megabyte(byte: f64) -> f64 {
    byte / 1_000_000.0
}

fn byte_to_gigabyte(byte: f64) -> f64 {
    byte / 1_000_000_000.0
}

fn kilobyte_to_megabyte(kilobyte: f64) -> f64 {
    kilobyte / 1000.0
}

fn kilobyte_to_gigabyte(kilobyte: f64) -> f64 {
    kilobyte / 1_000_000.0
}

fn megabyte_to_gigabyte(megabyte: f64) -> f64 {
    megabyte / 1000.0
}

impl Byte {
    #[must_use]
    pub fn per_second(self, seconds: Second) -> BytesPerSecond {
        BytesPerSecond(self.value() * seconds.value())
    }
}

impl Kilobyte {
    #[must_use]
    pub fn per_second(self, seconds: Second) -> KilobytesPerSecond {
        KilobytesPerSecond(self.value() * seconds.value())
    }
}

impl Megabyte {
    #[must_use]
    pub fn per_second(self, seconds: Second) -> MegabytesPerSecond {
        MegabytesPerSecond(self.value() * seconds.value())
    }
}

impl Gigabyte {
    #[must_use]
    pub fn per_hour(self, seconds: Second) -> GigabytesPerHour {
        GigabytesPerHour(self.value() * seconds.value() * 3600.0)
    }
}
