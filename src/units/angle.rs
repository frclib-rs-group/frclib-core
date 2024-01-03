use crate::units::angular_acceleration::{
    DegreePerSecSqr, RadianPerSecSqr, RotationPerMinSqr, RotationPerSecSqr,
};
use crate::units::angular_velocity::{DegreePerSec, RadianPerSec, RotationPerMin, RotationPerSec};
use crate::units::time::{Minute, Second};
use crate::{unit, unit_conversion, unit_dim_analysis, unit_family};

unit!(Degree: float);
unit!(Radian: float);
unit!(Rotation: float);

unit_conversion!(Degree(float) <-> Radian(float) ~ degree_to_radian);
unit_conversion!(Degree(float) <-> Rotation(float) ~ degree_to_rotation);
unit_conversion!(Radian(float) <-> Rotation(float) ~ radian_to_rotation);

unit_family!(Angle(Radian): Degree, Rotation);

unit_dim_analysis!(DegreePerSec * Second = Degree);
unit_dim_analysis!(RadianPerSec * Second = Radian);
unit_dim_analysis!(RotationPerSec * Second = Rotation);
unit_dim_analysis!(RotationPerMin * Minute = Rotation);

unit_dim_analysis!(DegreePerSecSqr * Second = DegreePerSec);
unit_dim_analysis!(RadianPerSecSqr * Second = RadianPerSec);
unit_dim_analysis!(RotationPerSecSqr * Second = RotationPerSec);
unit_dim_analysis!(RotationPerMinSqr * Minute = RotationPerMin);

fn degree_to_radian(degree: f64) -> f64 {
    degree.to_radians()
}

fn degree_to_rotation(degree: f64) -> f64 {
    degree / 360.0
}

fn radian_to_rotation(radian: f64) -> f64 {
    degree_to_rotation(radian.to_degrees())
}
