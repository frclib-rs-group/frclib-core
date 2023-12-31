use crate::{unit, unit_conversion, unit_dim_analysis, unit_family};
use crate::units::linear_velocity::{FeetPerSecond, MeterPerSecond};
use crate::units::time::Second;

unit!(Meter: float);
unit!(Feet: float);
unit!(Inch: float);
unit!(Centimeter: float);

unit_conversion!(Meter(float) <-> Feet(float) ~ meter_to_feet);
unit_conversion!(Meter(float) <-> Inch(float) ~ meter_to_inch);
unit_conversion!(Feet(float) <-> Inch(float) ~ foot_to_inch);
unit_conversion!(Meter(float) <-> Centimeter(float) ~ meter_to_centimeter);
unit_conversion!(Centimeter(float) <-> Feet(float) ~ centimeter_to_foot);
unit_conversion!(Centimeter(float) <-> Inch(float) ~ centimeter_to_inch);

unit_dim_analysis!(MeterPerSecond * Second = Meter);
unit_dim_analysis!(FeetPerSecond * Second = Feet);

unit_family!(Distance(Meter): Inch, Feet, Centimeter);




fn meter_to_feet(meter: f64) -> f64 {
    meter * 3.28084
}

fn meter_to_inch(meter: f64) -> f64 {
    meter * 3.28084 * 12.0
}

fn foot_to_inch(foot: f64) -> f64 {
    foot * 12.0
}

fn meter_to_centimeter(meter: f64) -> f64 {
    meter * 100.0
}

fn centimeter_to_foot(centimeter: f64) -> f64 {
    meter_to_feet(centimeter / 100.0)
}

fn centimeter_to_inch(centimeter: f64) -> f64 {
    meter_to_inch(centimeter / 100.0)
}

impl Meter {
    #[must_use]
    pub fn per_second(self, seconds: Second) -> MeterPerSecond {
        MeterPerSecond::new(self.value() * seconds.value())
    }
}

impl Feet {
    #[must_use]
    pub fn per_second(self, seconds: Second) -> FeetPerSecond {
        FeetPerSecond::new(self.value() * seconds.value())
    }
}

impl Inch {
    #[must_use]
    pub fn to_feet_per_second(self, seconds: Second) -> FeetPerSecond {
        FeetPerSecond::new(self.value() * seconds.value() / 12.0)
    }
}

impl Centimeter {
    #[must_use]
    pub fn to_meter_per_second(self, seconds: Second) -> MeterPerSecond {
        MeterPerSecond::new(self.value() * seconds.value() / 100.0)
    }
}
