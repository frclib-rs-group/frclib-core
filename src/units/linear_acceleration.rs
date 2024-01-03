use crate::{unit, unit_conversion, unit_family};

unit!(MetersPerSecSqr: float);
unit!(KilometersPerHrSqr: float);
unit!(MilesPerHrSqr: float);
unit!(FeetPerSecSqr: float);

unit_conversion!(MetersPerSecSqr(float) <-> KilometersPerHrSqr(float) ~ meter_to_kilometer);
unit_conversion!(MetersPerSecSqr(float) <-> MilesPerHrSqr(float) ~ meter_to_mile);
unit_conversion!(MetersPerSecSqr(float) <-> FeetPerSecSqr(float) ~ meter_to_feet);
unit_conversion!(FeetPerSecSqr(float) <-> MilesPerHrSqr(float) ~ feet_to_mile);
unit_conversion!(FeetPerSecSqr(float) <-> KilometersPerHrSqr(float) ~ feet_to_kilometer);
unit_conversion!(MilesPerHrSqr(float) <-> KilometersPerHrSqr(float) ~ mile_to_kilometer);

unit_family!(LinearVelocity(MetersPerSecSqr): KilometersPerHrSqr, MilesPerHrSqr, FeetPerSecSqr);

fn meter_to_kilometer(meter: f64) -> f64 {
    meter * 3.6
}

fn meter_to_mile(meter: f64) -> f64 {
    meter * 2.23694
}

fn meter_to_feet(meter: f64) -> f64 {
    meter * 3.28084
}

fn feet_to_mile(feet: f64) -> f64 {
    meter_to_mile(feet / 3.28084)
}

fn feet_to_kilometer(feet: f64) -> f64 {
    meter_to_kilometer(feet / 3.28084)
}

fn mile_to_kilometer(mile: f64) -> f64 {
    meter_to_kilometer(mile / 2.23694)
}