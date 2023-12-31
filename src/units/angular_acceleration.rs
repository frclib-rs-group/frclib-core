use crate::{unit, unit_conversion};

unit!(DegreePerSecondSquared: float);
unit!(RadianPerSecondSquared: float);
unit!(RotationPerSecondSquared: float);
unit!(RotationPerMinuteSquared: float);

unit_conversion!(DegreePerSecondSquared(float) <-> RadianPerSecondSquared(float) ~ degree_per_second_squared_to_radian_per_second_squared);
unit_conversion!(DegreePerSecondSquared(float) <-> RotationPerSecondSquared(float) ~ degree_per_second_squared_to_rotation_per_second_squared);
unit_conversion!(DegreePerSecondSquared(float) <-> RotationPerMinuteSquared(float) ~ degree_per_second_squared_to_rotation_per_minute_squared);
unit_conversion!(RadianPerSecondSquared(float) <-> RotationPerSecondSquared(float) ~ radian_per_second_squared_to_rotation_per_second_squared);
unit_conversion!(RadianPerSecondSquared(float) <-> RotationPerMinuteSquared(float) ~ radian_per_second_squared_to_rotation_per_minute_squared);
unit_conversion!(RotationPerSecondSquared(float) <-> RotationPerMinuteSquared(float) ~ rotation_per_second_squared_to_rotation_per_minute_squared);

// unit_family!(AngularAccel: DegreePerSecondSquared RadianPerSecondSquared RotationPerSecondSquared RotationPerMinuteSquared);

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
