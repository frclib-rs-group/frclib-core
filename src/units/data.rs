use crate::units::data_rate::{
    BytesPerSecond, GigabytesPerHour, KilobytesPerSecond, MegabytesPerSecond,
};
use crate::units::time::Second;
use crate::{unit, unit_conversion, unit_family, unit_dim_analysis};

use super::time::Hour;

unit!(Byte: float);
unit!(Kilobyte: float);
unit!(Megabyte: float);
unit!(Gigabyte: float);

unit_conversion!(Byte(float) <-> Kilobyte(float) ~ byte_to_kilobyte);
unit_conversion!(Byte(float) <-> Megabyte(float) ~ byte_to_megabyte);
unit_conversion!(Byte(float) <-> Gigabyte(float) ~ byte_to_gigabyte);
unit_conversion!(Kilobyte(float) <-> Megabyte(float) ~ kilobyte_to_megabyte);
unit_conversion!(Kilobyte(float) <-> Gigabyte(float) ~ kilobyte_to_gigabyte);
unit_conversion!(Megabyte(float) <-> Gigabyte(float) ~ megabyte_to_gigabyte);

unit_family!(Data(Byte): Kilobyte, Megabyte, Gigabyte);

unit_dim_analysis!(BytesPerSecond * Second = Byte);
unit_dim_analysis!(KilobytesPerSecond * Second = Kilobyte);
unit_dim_analysis!(MegabytesPerSecond * Second = Megabyte);
unit_dim_analysis!(GigabytesPerHour * Hour = Gigabyte);

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
