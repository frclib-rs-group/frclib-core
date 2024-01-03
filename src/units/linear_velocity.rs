use crate::{unit, unit_conversion, unit_family};

unit!(MetersPerSecond: float);
unit!(KilometersPerHour: float);
unit!(MilesPerHour: float);
unit!(FeetPerSecond: float);

unit_conversion!(MetersPerSecond(float) <-> KilometersPerHour(float) ~ meter_to_kilometer);
unit_conversion!(MetersPerSecond(float) <-> MilesPerHour(float) ~ meter_to_mile);
unit_conversion!(MetersPerSecond(float) <-> FeetPerSecond(float) ~ meter_to_feet);
unit_conversion!(FeetPerSecond(float) <-> MilesPerHour(float) ~ feet_to_mile_per_hour);
unit_conversion!(FeetPerSecond(float) <-> KilometersPerHour(float) ~ feet_to_kilometer_per_hour);
unit_conversion!(MilesPerHour(float) <-> KilometersPerHour(float) ~ mile_per_hour_to_kilometer_per_hour);

unit_family!(LinearVelocity(MetersPerSecond): KilometersPerHour, MilesPerHour, FeetPerSecond);

fn meter_to_kilometer(meter: f64) -> f64 {
    meter * 3.6
}

fn meter_to_mile(meter: f64) -> f64 {
    meter * 2.23694
}

fn meter_to_feet(meter: f64) -> f64 {
    meter * 3.28084
}

fn feet_to_mile_per_hour(feet: f64) -> f64 {
    meter_to_mile(feet / 3.28084)
}

fn feet_to_kilometer_per_hour(feet: f64) -> f64 {
    meter_to_kilometer(feet / 3.28084)
}

fn mile_per_hour_to_kilometer_per_hour(mile_per_hour: f64) -> f64 {
    meter_to_kilometer(mile_per_hour / 2.23694)
}
