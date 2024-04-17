/// NOT FOR DIRECT USE
#[doc(hidden)]
#[macro_export]
macro_rules! constant {
    ($( $method:ident () -> $ret:expr ; )*)
        => {$(
            #[inline]
            fn $method() -> Self {
                $ret
            }
        )*};
}

/// NOT FOR DIRECT USE
#[doc(hidden)]
#[macro_export]
macro_rules! forward {
    ($( $imp:path as $method:ident ( & self $( , $arg:ident : $ty:ty )* ) -> $ret:ty ; )*)
        => {$(
            #[inline]
            fn $method(&self $( , $arg : $ty )* ) -> $ret {
                $imp(&self.0 $( , $arg )* )
            }
        )*};
    ($( $imp:path as $method:ident ( self $( , $arg:ident : $ty:ty )* ) -> $ret:ty ; )*)
        => {$(
            #[inline]
            fn $method(self $( , $arg : $ty )* ) -> $ret {
                $imp(self.0 $( , $arg )* )
            }
        )*};
}

/// NOT FOR DIRECT USE
#[doc(hidden)]
#[macro_export]
macro_rules! forward_into {
    ($( $imp:path as $method:ident ( & self $( , $arg:ident : $ty:ty )* ) -> $ret:ty ; )*)
        => {$(
            #[inline]
            fn $method(&self $( , $arg : $ty )* ) -> $ret {
                $imp(&self.0 $( , $arg )* ).clone().into()
            }
        )*};
    ($( $imp:path as $method:ident ( self $( , $arg:ident : $ty:ty )* ) -> $ret:ty ; )*)
        => {$(
            #[inline]
            fn $method(self $( , $arg : $ty )* ) -> $ret {
                $imp(self.0 $( , $arg )* ).into()
            }
        )*};
}

/// NOT FOR DIRECT USE
#[doc(hidden)]
#[macro_export]
macro_rules! forward_into_args {
    ($( $imp:path as $method:ident ( self $( , $arg:ident : $ty:ty )* ) -> $ret:ty ; )*)
        => {$(
            #[inline]
            fn $method(self $( , $arg : $ty )* ) -> $ret {
                $imp(self.0 $( , $arg.0 )* ).into()
            }
        )*};
}

/// NOT FOR DIRECT USE
#[doc(hidden)]
#[macro_export]
macro_rules! unit_float {
    ($unit_name:ident) => {
        impl num::traits::NumCast for $unit_name {
            fn from<T: num::traits::ToPrimitive>(n: T) -> Option<Self> {
                n.to_f64().map(Self)
            }
        }
        impl num::traits::Float for $unit_name {
            #[allow(deprecated)]
            fn abs_sub(self, other: Self) -> Self {
                Self(self.0.abs_sub(other.0))
            }
            fn sin_cos(self) -> (Self, Self) {
                let (sin, cos) = self.0.sin_cos();
                (Self(sin), Self(cos))
            }
            fn powi(self, n: i32) -> Self {
                Self(self.0.powi(n))
            }
            $crate::forward_into! {
                f64::floor as floor(self) -> Self;
                f64::ceil as ceil(self) -> Self;
                f64::round as round(self) -> Self;
                f64::trunc as trunc(self) -> Self;
                f64::abs as abs(self) -> Self;
                f64::sqrt as sqrt(self) -> Self;
                f64::exp as exp(self) -> Self;
                f64::exp2 as exp2(self) -> Self;
                f64::ln as ln(self) -> Self;
                f64::log2 as log2(self) -> Self;
                f64::log10 as log10(self) -> Self;
                f64::cbrt as cbrt(self) -> Self;
                f64::sin as sin(self) -> Self;
                f64::cos as cos(self) -> Self;
                f64::tan as tan(self) -> Self;
                f64::asin as asin(self) -> Self;
                f64::acos as acos(self) -> Self;
                f64::atan as atan(self) -> Self;
                f64::exp_m1 as exp_m1(self) -> Self;
                f64::ln_1p as ln_1p(self) -> Self;
                f64::sinh as sinh(self) -> Self;
                f64::cosh as cosh(self) -> Self;
                f64::tanh as tanh(self) -> Self;
                f64::asinh as asinh(self) -> Self;
                f64::acosh as acosh(self) -> Self;
                f64::atanh as atanh(self) -> Self;
                f64::to_degrees as to_degrees(self) -> Self;
                f64::to_radians as to_radians(self) -> Self;
                f64::fract as fract(self) -> Self;
                f64::recip as recip(self) -> Self;
                f64::signum as signum(self) -> Self;
            }
            $crate::forward! {
                f64::is_nan as is_nan(self) -> bool;
                f64::is_infinite as is_infinite(self) -> bool;
                f64::is_finite as is_finite(self) -> bool;
                f64::is_normal as is_normal(self) -> bool;
                f64::classify as classify(self) -> std::num::FpCategory;
                f64::integer_decode as integer_decode(self) -> (u64, i16, i8);
                f64::is_sign_positive as is_sign_positive(self) -> bool;
                f64::is_sign_negative as is_sign_negative(self) -> bool;
            }
            $crate::forward_into_args! {
                f64::mul_add as mul_add(self, a: Self, b: Self) -> Self;
                f64::log as log(self, base: Self) -> Self;
                f64::hypot as hypot(self, other: Self) -> Self;
                f64::atan2 as atan2(self, other: Self) -> Self;
                f64::copysign as copysign(self, sign: Self) -> Self;
                f64::min as min(self, other: Self) -> Self;
                f64::max as max(self, other: Self) -> Self;
                f64::powf as powf(self, n: Self) -> Self;
            }
            $crate::constant! {
                infinity() -> $unit_name(f64::INFINITY);
                neg_infinity() -> $unit_name(f64::NEG_INFINITY);
                nan() -> $unit_name(f64::NAN);
                neg_zero() -> $unit_name(-0.0);
                min_value() -> $unit_name(f64::MIN);
                min_positive_value() -> $unit_name(f64::MIN_POSITIVE);
                epsilon() -> $unit_name(f64::EPSILON);
                max_value() -> $unit_name(f64::MAX);
            }
        }
    };
}

/// NOT FOR DIRECT USE
#[doc(hidden)]
#[macro_export]
macro_rules! unit_integer {
    ($unit_name:ident) => {
        impl num::Integer for $unit_name {
            fn div_floor(&self, other: &Self) -> Self {
                Self(num::Integer::div_floor(&self.0, &other.0))
            }
            fn mod_floor(&self, other: &Self) -> Self {
                Self(num::Integer::mod_floor(&self.0, &other.0))
            }
            fn gcd(&self, other: &Self) -> Self {
                Self(num::Integer::gcd(&self.0, &other.0))
            }
            fn lcm(&self, other: &Self) -> Self {
                Self(num::Integer::lcm(&self.0, &other.0))
            }
            fn next_multiple_of(&self, other: &Self) -> Self {
                Self(num::Integer::next_multiple_of(&self.0, &other.0))
            }
            fn divides(&self, other: &Self) -> bool {
                num::Integer::is_multiple_of(&self.0, &other.0)
            }
            fn is_multiple_of(&self, other: &Self) -> bool {
                num::Integer::is_multiple_of(&self.0, &other.0)
            }
            fn is_even(&self) -> bool {
                num::Integer::is_even(&self.0)
            }
            fn is_odd(&self) -> bool {
                num::Integer::is_odd(&self.0)
            }
            fn div_rem(&self, other: &Self) -> (Self, Self) {
                let (div, rem) = self.0.div_rem(&other.0);
                (Self(div), Self(rem))
            }
            fn div_mod_floor(&self, other: &Self) -> (Self, Self) {
                let (div, rem) = self.0.div_mod_floor(&other.0);
                (Self(div), Self(rem))
            }
        }

        impl num::Signed for $unit_name {
            fn abs(&self) -> Self {
                Self(self.0.abs())
            }
            fn abs_sub(&self, other: &Self) -> Self {
                Self(self.0.abs_sub(&other.0))
            }
            fn signum(&self) -> Self {
                Self(self.0.signum())
            }
            fn is_positive(&self) -> bool {
                self.0.is_positive()
            }
            fn is_negative(&self) -> bool {
                self.0.is_negative()
            }
        }

        impl num::traits::CheckedAdd for $unit_name {
            fn checked_add(&self, other: &Self) -> Option<Self> {
                num::traits::CheckedAdd::checked_add(&self.0, &other.0).map(Self)
            }
        }

        impl num::traits::CheckedSub for $unit_name {
            fn checked_sub(&self, other: &Self) -> Option<Self> {
                num::traits::CheckedSub::checked_sub(&self.0, &other.0).map(Self)
            }
        }

        impl num::traits::CheckedMul for $unit_name {
            fn checked_mul(&self, other: &Self) -> Option<Self> {
                num::traits::CheckedMul::checked_mul(&self.0, &other.0).map(Self)
            }
        }

        impl num::traits::CheckedDiv for $unit_name {
            fn checked_div(&self, other: &Self) -> Option<Self> {
                num::traits::CheckedDiv::checked_div(&self.0, &other.0).map(Self)
            }
        }

        impl num::traits::CheckedRem for $unit_name {
            fn checked_rem(&self, other: &Self) -> Option<Self> {
                num::traits::CheckedRem::checked_rem(&self.0, &other.0).map(Self)
            }
        }

        impl num::traits::CheckedNeg for $unit_name {
            fn checked_neg(&self) -> Option<Self> {
                num::traits::CheckedNeg::checked_neg(&self.0).map(Self)
            }
        }
    };
}

/// NOT FOR DIRECT USE
#[doc(hidden)]
#[macro_export]
macro_rules! unit_uinteger {
    ($unit_name:ident) => {
        impl num::Integer for $unit_name {
            fn div_floor(&self, other: &Self) -> Self {
                Self(num::Integer::div_floor(&self.0, &other.0))
            }
            fn mod_floor(&self, other: &Self) -> Self {
                Self(num::Integer::mod_floor(&self.0, &other.0))
            }
            fn gcd(&self, other: &Self) -> Self {
                Self(num::Integer::gcd(&self.0, &other.0))
            }
            fn lcm(&self, other: &Self) -> Self {
                Self(num::Integer::lcm(&self.0, &other.0))
            }
            fn next_multiple_of(&self, other: &Self) -> Self {
                Self(num::Integer::next_multiple_of(&self.0, &other.0))
            }
            fn divides(&self, other: &Self) -> bool {
                num::Integer::is_multiple_of(&self.0, &other.0)
            }
            fn is_multiple_of(&self, other: &Self) -> bool {
                num::Integer::is_multiple_of(&self.0, &other.0)
            }
            fn is_even(&self) -> bool {
                num::Integer::is_even(&self.0)
            }
            fn is_odd(&self) -> bool {
                num::Integer::is_odd(&self.0)
            }
            fn div_rem(&self, other: &Self) -> (Self, Self) {
                let (div, rem) = self.0.div_rem(&other.0);
                (Self(div), Self(rem))
            }
            fn div_mod_floor(&self, other: &Self) -> (Self, Self) {
                let (div, rem) = self.0.div_mod_floor(&other.0);
                (Self(div), Self(rem))
            }
        }

        impl num::Unsigned for $unit_name {}

        impl num::traits::CheckedAdd for $unit_name {
            fn checked_add(&self, other: &Self) -> Option<Self> {
                num::traits::CheckedAdd::checked_add(&self.0, &other.0).map(Self)
            }
        }

        impl num::traits::CheckedSub for $unit_name {
            fn checked_sub(&self, other: &Self) -> Option<Self> {
                num::traits::CheckedSub::checked_sub(&self.0, &other.0).map(Self)
            }
        }

        impl num::traits::CheckedMul for $unit_name {
            fn checked_mul(&self, other: &Self) -> Option<Self> {
                num::traits::CheckedMul::checked_mul(&self.0, &other.0).map(Self)
            }
        }

        impl num::traits::CheckedDiv for $unit_name {
            fn checked_div(&self, other: &Self) -> Option<Self> {
                num::traits::CheckedDiv::checked_div(&self.0, &other.0).map(Self)
            }
        }

        impl num::traits::CheckedRem for $unit_name {
            fn checked_rem(&self, other: &Self) -> Option<Self> {
                num::traits::CheckedRem::checked_rem(&self.0, &other.0).map(Self)
            }
        }
    };
}
