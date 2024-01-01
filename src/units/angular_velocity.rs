use crate::{unit, unit_conversion, unit_family};

unit!(DegreePerSecond: float);
unit!(RadianPerSecond: float);
unit!(RotationPerSecond: float);
unit!(RotationPerMinute: float);

unit_conversion!(DegreePerSecond(float) <-> RadianPerSecond(float) ~ degree_per_second_to_radian_per_second);
unit_conversion!(DegreePerSecond(float) <-> RotationPerSecond(float) ~ degree_per_second_to_rotation_per_second);
unit_conversion!(DegreePerSecond(float) <-> RotationPerMinute(float) ~ degree_per_second_to_rotation_per_minute);
unit_conversion!(RadianPerSecond(float) <-> RotationPerSecond(float) ~ radian_per_second_to_rotation_per_second);
unit_conversion!(RadianPerSecond(float) <-> RotationPerMinute(float) ~ radian_per_second_to_rotation_per_minute);
unit_conversion!(RotationPerSecond(float) <-> RotationPerMinute(float) ~ rotation_per_second_to_rotation_per_minute);

unit_family!(AngleVel(RadianPerSecond): DegreePerSecond, RotationPerSecond, RotationPerMinute);

fn degree_per_second_to_radian_per_second(degree_per_second: f64) -> f64 {
    degree_per_second.to_radians()
}

fn degree_per_second_to_rotation_per_second(degree_per_second: f64) -> f64 {
    degree_per_second / 360.0
}

fn degree_per_second_to_rotation_per_minute(degree_per_second: f64) -> f64 {
    degree_per_second / 360.0 * 60.0
}

fn radian_per_second_to_rotation_per_second(radian_per_second: f64) -> f64 {
    degree_per_second_to_rotation_per_second(radian_per_second.to_degrees())
}

fn radian_per_second_to_rotation_per_minute(radian_per_second: f64) -> f64 {
    degree_per_second_to_rotation_per_minute(radian_per_second.to_degrees())
}

fn rotation_per_second_to_rotation_per_minute(rotation_per_second: f64) -> f64 {
    rotation_per_second * 60.0
}
