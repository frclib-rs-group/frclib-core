use crate::{unit, unit_conversion, unit_family};

unit!(MetersPerSecond: float);
unit!(KilometersPerHour: float);
unit!(MilesPerHour: float);
unit!(FeetPerSecond: float);

unit_conversion!(MetersPerSecond(float) <-> KilometersPerHour(float) ~ meter_per_second_to_kilometer_per_hour);
unit_conversion!(MetersPerSecond(float) <-> MilesPerHour(float) ~ meter_per_second_to_mile_per_hour);
unit_conversion!(MetersPerSecond(float) <-> FeetPerSecond(float) ~ meter_per_second_to_feet_per_second);
unit_conversion!(FeetPerSecond(float) <-> MilesPerHour(float) ~ feet_per_second_to_mile_per_hour);
unit_conversion!(FeetPerSecond(float) <-> KilometersPerHour(float) ~ feet_per_second_to_kilometer_per_hour);
unit_conversion!(MilesPerHour(float) <-> KilometersPerHour(float) ~ mile_per_hour_to_kilometer_per_hour);

unit_family!(LinearVelocity(MetersPerSecond): KilometersPerHour, MilesPerHour, FeetPerSecond);

fn meter_per_second_to_kilometer_per_hour(meter_per_second: f64) -> f64 {
    meter_per_second * 3.6
}

fn meter_per_second_to_mile_per_hour(meter_per_second: f64) -> f64 {
    meter_per_second * 2.23694
}

fn meter_per_second_to_feet_per_second(meter_per_second: f64) -> f64 {
    meter_per_second * 3.28084
}

fn feet_per_second_to_mile_per_hour(feet_per_second: f64) -> f64 {
    meter_per_second_to_mile_per_hour(feet_per_second / 3.28084)
}

fn feet_per_second_to_kilometer_per_hour(feet_per_second: f64) -> f64 {
    meter_per_second_to_kilometer_per_hour(feet_per_second / 3.28084)
}

fn mile_per_hour_to_kilometer_per_hour(mile_per_hour: f64) -> f64 {
    meter_per_second_to_kilometer_per_hour(mile_per_hour / 2.23694)
}
