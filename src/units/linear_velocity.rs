use crate::units::distance::{Feet, Meter};
use crate::units::time::Second;
use crate::{unit, unit_conversion, unit_family};

unit!(MeterPerSecond: float);
unit!(KilometerPerHour: float);
unit!(MilePerHour: float);
unit!(FeetPerSecond: float);

unit_conversion!(MeterPerSecond(float) <-> KilometerPerHour(float) ~ meter_per_second_to_kilometer_per_hour);
unit_conversion!(MeterPerSecond(float) <-> MilePerHour(float) ~ meter_per_second_to_mile_per_hour);
unit_conversion!(MeterPerSecond(float) <-> FeetPerSecond(float) ~ meter_per_second_to_feet_per_second);
unit_conversion!(FeetPerSecond(float) <-> MilePerHour(float) ~ feet_per_second_to_mile_per_hour);
unit_conversion!(FeetPerSecond(float) <-> KilometerPerHour(float) ~ feet_per_second_to_kilometer_per_hour);
unit_conversion!(MilePerHour(float) <-> KilometerPerHour(float) ~ mile_per_hour_to_kilometer_per_hour);

unit_family!(LinearVelocity(MeterPerSecond): KilometerPerHour, MilePerHour, FeetPerSecond);

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

impl MilePerHour {
    #[must_use]
    pub fn to_feet(&self, seconds: Second) -> Feet {
        Feet::new(FeetPerSecond::from(*self).value() * seconds.value())
    }
}

impl KilometerPerHour {
    #[must_use]
    pub fn to_meters(&self, seconds: Second) -> Meter {
        Meter::new(MeterPerSecond::from(*self).value() * seconds.value())
    }
}
