use crate::units::angular_acceleration::{
    DegreePerSecSqr, RadianPerSecSqr, RotationPerMinSqr, RotationPerSecSqr,
};
use crate::units::angular_velocity::{DegreePerSec, RadianPerSec, RotationPerMin, RotationPerSec};
use crate::units::time::{Minute, Second};
use crate::{unit, unit_conversion, unit_dim_analysis, unit_family};

unit!(Degree | Degrees | Deg | Degs: float);
unit!(Radian | Radians | Rad | Rads: float);
unit!(Rotation | Rotations | Rot | Rots: float);

unit_conversion!(Degree(float) <-> Radian(float)   ~ |x| x.to_radians());
unit_conversion!(Degree(float) <-> Rotation(float) ~ ratio 1.0/360.0);
unit_conversion!(Radian(float) <-> Rotation(float) ~ |x| x / (std::f64::consts::PI * 2.0));

unit_family!(Angle(Radian): Degree, Rotation);

unit_dim_analysis!(DegreePerSec * Second = Degree);
unit_dim_analysis!(RadianPerSec * Second = Radian);
unit_dim_analysis!(RotationPerSec * Second = Rotation);
unit_dim_analysis!(RotationPerMin * Minute = Rotation);

unit_dim_analysis!(DegreePerSecSqr * Second = DegreePerSec);
unit_dim_analysis!(RadianPerSecSqr * Second = RadianPerSec);
unit_dim_analysis!(RotationPerSecSqr * Second = RotationPerSec);
unit_dim_analysis!(RotationPerMinSqr * Minute = RotationPerMin);