use crate::{unit, unit_conversion, unit_family};

unit!(DegreePerSec: float);
unit!(RadianPerSec: float);
unit!(RotationPerSec: float);
unit!(RotationPerMin: float);

unit_conversion!(DegreePerSec(float) <-> RadianPerSec(float) ~ degree_per_second_to_radian_per_second);
unit_conversion!(DegreePerSec(float) <-> RotationPerSec(float) ~ degree_per_second_to_rotation_per_second);
unit_conversion!(DegreePerSec(float) <-> RotationPerMin(float) ~ degree_per_second_to_rotation_per_minute);
unit_conversion!(RadianPerSec(float) <-> RotationPerSec(float) ~ radian_per_second_to_rotation_per_second);
unit_conversion!(RadianPerSec(float) <-> RotationPerMin(float) ~ radian_per_second_to_rotation_per_minute);
unit_conversion!(RotationPerSec(float) <-> RotationPerMin(float) ~ rotation_per_second_to_rotation_per_minute);

unit_family!(AngleVel(RadianPerSec): DegreePerSec, RotationPerSec, RotationPerMin);

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
