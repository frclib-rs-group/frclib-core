use crate::{unit, unit_conversion, unit_family};

unit!(DegreePerSecSqr: float);
unit!(RadianPerSecSqr: float);
unit!(RotationPerSecSqr: float);
unit!(RotationPerMinSqr: float);

unit_conversion!(DegreePerSecSqr(float) <-> RadianPerSecSqr(float) ~ degree_per_second_squared_to_radian_per_second_squared);
unit_conversion!(DegreePerSecSqr(float) <-> RotationPerSecSqr(float) ~ degree_per_second_squared_to_rotation_per_second_squared);
unit_conversion!(DegreePerSecSqr(float) <-> RotationPerMinSqr(float) ~ degree_per_second_squared_to_rotation_per_minute_squared);
unit_conversion!(RadianPerSecSqr(float) <-> RotationPerSecSqr(float) ~ radian_per_second_squared_to_rotation_per_second_squared);
unit_conversion!(RadianPerSecSqr(float) <-> RotationPerMinSqr(float) ~ radian_per_second_squared_to_rotation_per_minute_squared);
unit_conversion!(RotationPerSecSqr(float) <-> RotationPerMinSqr(float) ~ rotation_per_second_squared_to_rotation_per_minute_squared);

unit_family!(AngleAccel(RadianPerSecSqr): DegreePerSecSqr, RotationPerSecSqr, RotationPerMinSqr);

fn degree_per_second_squared_to_radian_per_second_squared(degree_per_second_squared: f64) -> f64 {
    degree_per_second_squared.to_radians()
}

fn degree_per_second_squared_to_rotation_per_second_squared(degree_per_second_squared: f64) -> f64 {
    degree_per_second_squared / 360.0
}

fn degree_per_second_squared_to_rotation_per_minute_squared(degree_per_second_squared: f64) -> f64 {
    degree_per_second_squared / 360.0 * 60.0
}

fn radian_per_second_squared_to_rotation_per_second_squared(radian_per_second_squared: f64) -> f64 {
    degree_per_second_squared_to_rotation_per_second_squared(radian_per_second_squared.to_degrees())
}

fn radian_per_second_squared_to_rotation_per_minute_squared(radian_per_second_squared: f64) -> f64 {
    degree_per_second_squared_to_rotation_per_minute_squared(radian_per_second_squared.to_degrees())
}

fn rotation_per_second_squared_to_rotation_per_minute_squared(
    rotation_per_second_squared: f64,
) -> f64 {
    rotation_per_second_squared * 60.0
}
