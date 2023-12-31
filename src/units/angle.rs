use crate::{unit_family, unit, unit_conversion, unit_dim_analysis};
use crate::units::angular_acceleration::{
    DegreePerSecondSquared, RadianPerSecondSquared, RotationPerMinuteSquared,
    RotationPerSecondSquared,
};
use crate::units::angular_velocity::{
    DegreePerSecond, RadianPerSecond, RotationPerMinute, RotationPerSecond,
};
use crate::units::time::{Minute, Second};

unit!(Degree: float);
unit!(Radian: float);
unit!(Rotation: float);

unit_conversion!(Degree(float) <-> Radian(float) ~ degree_to_radian);
unit_conversion!(Degree(float) <-> Rotation(float) ~ degree_to_rotation);
unit_conversion!(Radian(float) <-> Rotation(float) ~ radian_to_rotation);

unit_family!(Angle(Radian): Degree, Rotation);

unit_dim_analysis!(DegreePerSecond * Second = Degree);
unit_dim_analysis!(RadianPerSecond * Second = Radian);
unit_dim_analysis!(RotationPerSecond * Second = Rotation);
unit_dim_analysis!(RotationPerMinute * Minute = Rotation);

unit_dim_analysis!(DegreePerSecondSquared * Second = DegreePerSecond);
unit_dim_analysis!(RadianPerSecondSquared * Second = RadianPerSecond);
unit_dim_analysis!(RotationPerSecondSquared * Second = RotationPerSecond);
unit_dim_analysis!(RotationPerMinuteSquared * Minute = RotationPerMinute);

fn degree_to_radian(degree: f64) -> f64 {
    degree.to_radians()
}

fn degree_to_rotation(degree: f64) -> f64 {
    degree / 360.0
}

fn radian_to_rotation(radian: f64) -> f64 {
    degree_to_rotation(radian.to_degrees())
}
