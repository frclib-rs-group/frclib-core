use crate::{unit, unit_conversion, unit_family};

unit!(NewtonMeter: float);
unit!(NewtonCentimeter: float);
unit!(KilogramMeter: float);
unit!(FootPound: float);
unit!(InchPound: float);

unit_conversion!(NewtonMeter(float) <-> NewtonCentimeter(float) ~ newton_meter_to_newton_centimeter);
unit_conversion!(NewtonMeter(float) <-> KilogramMeter(float) ~ newton_meter_to_kilogram_meter);
unit_conversion!(NewtonMeter(float) <-> FootPound(float) ~ newton_meter_to_foot_pound);
unit_conversion!(NewtonMeter(float) <-> InchPound(float) ~ newton_meter_to_inch_pound);
unit_conversion!(NewtonCentimeter(float) <-> KilogramMeter(float) ~ newton_centimeter_to_kilogram_meter);
unit_conversion!(NewtonCentimeter(float) <-> FootPound(float) ~ newton_centimeter_to_foot_pound);
unit_conversion!(NewtonCentimeter(float) <-> InchPound(float) ~ newton_centimeter_to_inch_pound);
unit_conversion!(KilogramMeter(float) <-> FootPound(float) ~ kilogram_meter_to_foot_pound);
unit_conversion!(KilogramMeter(float) <-> InchPound(float) ~ kilogram_meter_to_inch_pound);
unit_conversion!(FootPound(float) <-> InchPound(float) ~ foot_pound_to_inch_pound);

unit_family!(Torque(NewtonMeter): NewtonCentimeter, KilogramMeter, FootPound, InchPound);

fn newton_meter_to_newton_centimeter(newton_meter: f64) -> f64 {
    newton_meter * 100.0
}

fn newton_meter_to_kilogram_meter(newton_meter: f64) -> f64 {
    newton_meter * 0.101_972
}

fn newton_meter_to_foot_pound(newton_meter: f64) -> f64 {
    newton_meter * 0.737_562
}

fn newton_meter_to_inch_pound(newton_meter: f64) -> f64 {
    newton_meter * 8.85075
}

fn newton_centimeter_to_kilogram_meter(newton_centimeter: f64) -> f64 {
    newton_meter_to_kilogram_meter(newton_centimeter / 100.0)
}

fn newton_centimeter_to_foot_pound(newton_centimeter: f64) -> f64 {
    newton_meter_to_foot_pound(newton_centimeter / 100.0)
}

fn newton_centimeter_to_inch_pound(newton_centimeter: f64) -> f64 {
    newton_meter_to_inch_pound(newton_centimeter / 100.0)
}

fn kilogram_meter_to_foot_pound(kilogram_meter: f64) -> f64 {
    kilogram_meter * 7.23301
}

fn foot_pound_to_inch_pound(foot_pound: f64) -> f64 {
    foot_pound / 12.0
}

fn kilogram_meter_to_inch_pound(kilogram_meter: f64) -> f64 {
    foot_pound_to_inch_pound(kilogram_meter_to_foot_pound(kilogram_meter))
}
