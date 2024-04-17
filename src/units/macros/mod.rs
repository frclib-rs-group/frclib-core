#[macro_use]
mod components;
#[macro_use]
mod number;
#[macro_use]
mod helper;
#[doc(hidden)]
pub use paste;

/// A macro for defining a unit of measurement.
/// This macro is used to define a new unit of measurement.
///
/// # Example
/// ```
/// use frclib_core::unit;
///
/// unit!(DegreeFloat: float);
/// unit!(RadianInt: int);
/// unit!(RotationUint: uint);
/// ```
#[macro_export]
macro_rules! unit {
    ($unit_name:ident $( | $unit_alias:ident)* : float) => {
        /// A unit of measurement.
        /// This is a newtype wrapper around a [`f64`].
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
        pub struct $unit_name(pub f64);

        $crate::units::macros::paste::paste! {
            $(
                #[doc = "A unit of measurement, this is an alias for [`" $unit_name "`]."]
                #[doc = "This is a newtype wrapper around a [`f64`]."]
                pub type $unit_alias = $unit_name;
            )*
        }

        impl std::hash::Hash for $unit_name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.to_bits().hash(state);
            }
        }

        impl $unit_name {
            /// Creates a new instance of the unit with the given value.
            #[must_use]
            #[inline]
            pub const fn new(value: f64) -> Self {
                Self(value)
            }

            /// Returns the inner [`f64`] value.
            #[must_use]
            #[inline]
            pub const fn value(self) -> f64 {
                self.0
            }
        }

        $crate::unit_general!($unit_name : f64);
        $crate::unit_binops!($unit_name : f64);
        $crate::unit_neg!($unit_name : f64);
        $crate::unit_serde!($unit_name : f64);
        $crate::unit_num!($unit_name : f64);
        $crate::unit_float!($unit_name);
        $crate::unit_structure!($unit_name : f64);
    };
    ($unit_name:ident : int) => {
        /// A unit of measurement.
        /// This is a newtype wrapper around a [`i64`].
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        pub struct $unit_name(pub i64);

        impl $unit_name {
            /// Creates a new instance of the unit with the given value.
            #[must_use]
            #[inline]
            pub const fn new(value: i64) -> Self {
                Self(value)
            }

            /// Returns the inner [`i64`] value.
            #[must_use]
            #[inline]
            pub const fn value(self) -> i64 {
                self.0
            }
        }

        $crate::unit_general!($unit_name : i64);
        $crate::unit_binops!($unit_name : i64);
        $crate::unit_neg!($unit_name : i64);
        $crate::unit_serde!($unit_name : i64);
        $crate::unit_num!($unit_name : i64);
        $crate::unit_integer!($unit_name);
        $crate::unit_structure!($unit_name : i64);
    };
    ($unit_name:ident : uint) => {
        /// A unit of measurement.
        /// This is a newtype wrapper around a [`u64`].
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        pub struct $unit_name(pub u64);

        impl $unit_name {
            /// Creates a new instance of the unit with the given value.
            #[must_use]
            #[inline]
            pub const fn new(value: u64) -> Self {
                Self(value)
            }

            /// Returns the inner [`u64`] value.
            #[must_use]
            #[inline]
            pub const fn value(self) -> u64 {
                self.0
            }
        }

        $crate::unit_general!($unit_name : u64);
        $crate::unit_binops!($unit_name : u64);
        $crate::unit_serde!($unit_name : u64);
        $crate::unit_num!($unit_name : u64);
        $crate::unit_uinteger!($unit_name);
        $crate::unit_structure!($unit_name : u64);
    };
}

/// A macro for defining a unit conversion.
/// This macro is used to define a conversion between two units of the same dimension.
///
/// # Example
/// ```
/// use frclib_core::{unit_conversion, unit};
///
/// unit!(Degree: float);
/// unit!(Radian: float);
///
/// unit_conversion!(Degree(float) <-> Radian(float) ~ degree_to_radian);
///
/// fn degree_to_radian(degree: f64) -> f64 {
///     degree.to_radians()
/// }
/// ````
#[macro_export]
macro_rules! unit_conversion {
    ($unit_a:ident ( $unit_a_type:ident ) <-> $unit_b:ident ( $unit_b_type:ident ) ~ $conv_fn:ident ) => {
        $crate::inner_unit_conversion!(
            $unit_a $crate::complex_type_name!($unit_a_type)
            | $unit_b $crate::complex_type_name!($unit_b_type)
            : $conv_fn
        );
    };
    ($unit_a:ident ( $unit_a_type:ident ) <-> $unit_b:ident ( $unit_b_type:ident ) ~ | $c:ident | $conv_ex:expr ) => {
        $crate::units::macros::paste::paste! {
            #[doc(hidden)]
            #[inline]
            fn [< $unit_a:lower _to_ $unit_b:lower >]
                (inner_value: $crate::complex_type_name!($unit_a_type))
                -> $crate::complex_type_name!($unit_b_type)
            {
                #[allow(clippy::redundant_closure_call)]
                (| $c : $crate::complex_type_name!($unit_a_type) | $conv_ex)(inner_value)
            }
            $crate::inner_unit_conversion!(
                $unit_a $crate::complex_type_name!($unit_a_type)
                | $unit_b $crate::complex_type_name!($unit_b_type)
                : [< $unit_a:lower _to_ $unit_b:lower >]
            );
        }
    };
    ($unit_a:ident ( $unit_a_type:ident ) <-> $unit_b:ident ( $unit_b_type:ident ) ~ ratio $conv_ex:expr ) => {
        $crate::units::macros::paste::paste! {
            #[doc(hidden)]
            #[inline]
            fn [< $unit_a:lower _to_ $unit_b:lower >]
                (inner_value: $crate::complex_type_name!($unit_a_type))
                -> $crate::complex_type_name!($unit_b_type)
            {
                inner_value * $conv_ex
            }
            $crate::inner_unit_conversion!(
                $unit_a $crate::complex_type_name!($unit_a_type)
                | $unit_b $crate::complex_type_name!($unit_b_type)
                : [< $unit_a:lower _to_ $unit_b:lower >]
            );
        }
    };
}

/// A macro for defining a unit family.
/// Unit families allow all units to fall under a single trait.
/// This allows for easy conversion between units of the same family
/// and allows for functions to be generic over all units of a family.
///
/// # Example
/// ```
/// use frclib_core::{unit_family, unit, unit_conversion};
///
/// unit!(Degree: float);
/// unit!(Radian: float);
/// unit!(Rotation: float);
///
/// unit_conversion!(Degree(float) <-> Radian(float) ~ degree_to_radian);
/// unit_conversion!(Degree(float) <-> Rotation(float) ~ degree_to_rotation);
/// unit_conversion!(Radian(float) <-> Rotation(float) ~ radian_to_rotation);
///
/// unit_family!(Angle(Radian): Degree, Rotation);
///
/// fn degree_to_radian(degree: f64) -> f64 {
///     degree.to_radians()
/// }
///
/// fn degree_to_rotation(degree: f64) -> f64 {
///     degree / 360.0
/// }
///
/// fn radian_to_rotation(radian: f64) -> f64 {
///     degree_to_rotation(radian.to_degrees())
/// }
/// ````
#[macro_export]
macro_rules! unit_family {
    ($family_name:ident ( $standard:ident ): $($unit_name:ident),*) => {
        $crate::units::macros::paste::paste! {
            #[doc = "A family of units representing an `" $standard "` measurement."]
            #[doc = ""]
            #[doc = "The standard unit is `" $standard "`."]
            #[doc = ""]
            #[doc = "The other units are `" $($unit_name)"`, `"* "` and other user-defined units."]
            pub trait $family_name: Into<$standard> + From<$standard> + Copy {
                #[doc = "Converts this unit to the standard unit of the family."]
                #[inline]
                fn standard(self) -> $standard {
                    self.into()
                }

                fn conv<U: $family_name>(self) -> U {
                    U::from(self.standard())
                }
            }
        }

        impl<T> $family_name for T
        where
            T: Into<$standard> + From<$standard> + Copy,
        {
        }
    };
}

/// A macro for defining a unit dimension analysis.
///
/// # Example
/// ```
/// use frclib_core::{unit_dim_analysis, unit};
///
/// unit!(Degree: float);
/// unit!(Second: float);
/// unit!(DegreePerSecond: float);
///
/// unit_dim_analysis!(DegreePerSecond * Second = Degree);
/// // also supports division but mult implicitly adds support for division the other way
/// ```
#[macro_export]
macro_rules! unit_dim_analysis {
    ($unit_a:ident * $unit_b:ident = $ret:ident) => {
        impl std::ops::Mul<$unit_b> for $unit_a {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: $unit_b) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }
        impl std::ops::Mul<$unit_a> for $unit_b {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: $unit_a) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }
        impl std::ops::Mul<&$unit_b> for $unit_a {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: &$unit_b) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }
        impl std::ops::Mul<$unit_a> for &$unit_b {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: $unit_a) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }
        impl std::ops::Mul<$unit_b> for &$unit_a {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: $unit_b) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }
        impl std::ops::Mul<&$unit_a> for $unit_b {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: &$unit_a) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }
        impl std::ops::Mul<&$unit_b> for &$unit_a {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: &$unit_b) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }
        impl std::ops::Mul<&$unit_a> for &$unit_b {
            type Output = $ret;
            #[inline]
            fn mul(self, rhs: &$unit_a) -> Self::Output {
                $ret::from(self.0 * rhs.0)
            }
        }

        //other order
        impl std::ops::Div<$unit_a> for $ret {
            type Output = $unit_b;
            #[inline]
            fn div(self, rhs: $unit_a) -> Self::Output {
                $unit_b::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<$unit_b> for $ret {
            type Output = $unit_a;
            #[inline]
            fn div(self, rhs: $unit_b) -> Self::Output {
                $unit_a::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<&$unit_a> for $ret {
            type Output = $unit_b;
            #[inline]
            fn div(self, rhs: &$unit_a) -> Self::Output {
                $unit_b::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<$unit_a> for &$ret {
            type Output = $unit_b;
            #[inline]
            fn div(self, rhs: $unit_a) -> Self::Output {
                $unit_b::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<&$unit_a> for &$ret {
            type Output = $unit_b;
            #[inline]
            fn div(self, rhs: &$unit_a) -> Self::Output {
                $unit_b::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<&$unit_b> for $ret {
            type Output = $unit_a;
            #[inline]
            fn div(self, rhs: &$unit_b) -> Self::Output {
                $unit_a::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<$unit_b> for &$ret {
            type Output = $unit_a;
            #[inline]
            fn div(self, rhs: $unit_b) -> Self::Output {
                $unit_a::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<&$unit_b> for &$ret {
            type Output = $unit_a;
            #[inline]
            fn div(self, rhs: &$unit_b) -> Self::Output {
                $unit_a::from(self.0 / rhs.0)
            }
        }
    };
    ($unit_a:ident / $unit_b:ident = $ret:ident) => {
        impl std::ops::Div<$unit_b> for $unit_a {
            type Output = $ret;
            fn div(self, rhs: $unit_b) -> Self::Output {
                $ret::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<&$unit_b> for $unit_a {
            type Output = $ret;
            fn div(self, rhs: &$unit_b) -> Self::Output {
                $ret::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<$unit_b> for &$unit_a {
            type Output = $ret;
            fn div(self, rhs: $unit_b) -> Self::Output {
                $ret::from(self.0 / rhs.0)
            }
        }
        impl std::ops::Div<&$unit_b> for &$unit_a {
            type Output = $ret;
            fn div(self, rhs: &$unit_b) -> Self::Output {
                $ret::from(self.0 / rhs.0)
            }
        }
    };
}

#[cfg(test)]
mod test {
    unit!(Degree: float);
    unit!(Millisecond: int);
    unit!(Microsecond: uint);

    #[test]
    fn ops() {
        let deg = Degree(1.0);
        let new_deg = 1.0f64 + deg;
        assert_eq!(new_deg, Degree(2.0));

        let milli = Millisecond(-1);
        let new_milli = 1i64 + milli;
        assert_eq!(new_milli, Millisecond(0));

        let micro = Microsecond(1);
        let new_micro = 1u64 + micro;
        assert_eq!(new_micro, Microsecond(2));
    }
}
