use frclib_units_macros::{unit, unit_conversion};

unit!(DegreePerSecond, f64);
unit!(RadianPerSecond, f64);
unit!(RotationPerSecond, f64);
unit!(RotationPerMinute, f64);

unit_conversion!(DegreePerSecond f64, RadianPerSecond f64, degree_per_second_to_radian_per_second);
unit_conversion!(DegreePerSecond f64, RotationPerSecond f64, degree_per_second_to_rotation_per_second);
unit_conversion!(DegreePerSecond f64, RotationPerMinute f64, degree_per_second_to_rotation_per_minute);
unit_conversion!(RadianPerSecond f64, RotationPerSecond f64, radian_per_second_to_rotation_per_second);
unit_conversion!(RadianPerSecond f64, RotationPerMinute f64, radian_per_second_to_rotation_per_minute);
unit_conversion!(RotationPerSecond f64, RotationPerMinute f64, rotation_per_second_to_rotation_per_minute);

// unit_family!(AngularVelo: DegreePerSecond RadianPerSecond RotationPerSecond RotationPerMinute);

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
