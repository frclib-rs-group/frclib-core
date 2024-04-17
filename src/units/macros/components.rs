/// NOT FOR DIRECT USE
#[doc(hidden)]
#[macro_export]
macro_rules! unit_general {
    ($unit_name:ident : f64) => {
        impl From<f64> for $unit_name {
            fn from(value: f64) -> Self {
                Self(value)
            }
        }

        impl From<f32> for $unit_name {
            fn from(value: f32) -> Self {
                Self(value as f64)
            }
        }

        impl From<i64> for $unit_name {
            fn from(value: i64) -> Self {
                Self(value as f64)
            }
        }

        impl From<i32> for $unit_name {
            fn from(value: i32) -> Self {
                Self(value as f64)
            }
        }

        impl From<i16> for $unit_name {
            fn from(value: i16) -> Self {
                Self(value as f64)
            }
        }

        impl From<i8> for $unit_name {
            fn from(value: i8) -> Self {
                Self(value as f64)
            }
        }

        impl From<u64> for $unit_name {
            fn from(value: u64) -> Self {
                Self(value as f64)
            }
        }

        impl From<u32> for $unit_name {
            fn from(value: u32) -> Self {
                Self(value as f64)
            }
        }

        impl From<u16> for $unit_name {
            fn from(value: u16) -> Self {
                Self(value as f64)
            }
        }

        impl From<u8> for $unit_name {
            fn from(value: u8) -> Self {
                Self(value as f64)
            }
        }

        impl From<&$unit_name> for $unit_name {
            fn from(value: &$unit_name) -> Self {
                Self(value.0)
            }
        }

        impl From<$unit_name> for f64 {
            fn from(value: $unit_name) -> Self {
                value.0
            }
        }

        impl std::fmt::Display for $unit_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({}f64)", stringify!($unit_name), self.0)
            }
        }
    };
    ($unit_name:ident : i64) => {
        impl From<i64> for $unit_name {
            fn from(value: i64) -> Self {
                Self(value)
            }
        }

        impl From<i32> for $unit_name {
            fn from(value: i32) -> Self {
                Self(value as i64)
            }
        }

        impl From<i16> for $unit_name {
            fn from(value: i16) -> Self {
                Self(value as i64)
            }
        }

        impl From<i8> for $unit_name {
            fn from(value: i8) -> Self {
                Self(value as i64)
            }
        }

        impl From<u64> for $unit_name {
            fn from(value: u64) -> Self {
                Self(value as i64)
            }
        }

        impl From<u32> for $unit_name {
            fn from(value: u32) -> Self {
                Self(value as i64)
            }
        }

        impl From<u16> for $unit_name {
            fn from(value: u16) -> Self {
                Self(value as i64)
            }
        }

        impl From<u8> for $unit_name {
            fn from(value: u8) -> Self {
                Self(value as i64)
            }
        }

        impl From<&$unit_name> for $unit_name {
            fn from(value: &$unit_name) -> Self {
                Self(value.0)
            }
        }

        impl From<$unit_name> for i64 {
            fn from(value: $unit_name) -> Self {
                value.0
            }
        }

        impl std::fmt::Display for $unit_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({}i64)", stringify!($unit_name), self.0)
            }
        }
    };
    ($unit_name:ident : u64) => {
        impl From<u64> for $unit_name {
            fn from(value: u64) -> Self {
                Self(value)
            }
        }

        impl From<u32> for $unit_name {
            fn from(value: u32) -> Self {
                Self(value as u64)
            }
        }

        impl From<u16> for $unit_name {
            fn from(value: u16) -> Self {
                Self(value as u64)
            }
        }

        impl From<u8> for $unit_name {
            fn from(value: u8) -> Self {
                Self(value as u64)
            }
        }

        impl From<&$unit_name> for $unit_name {
            fn from(value: &$unit_name) -> Self {
                Self(value.0)
            }
        }

        impl From<$unit_name> for u64 {
            fn from(value: $unit_name) -> Self {
                value.0
            }
        }

        impl std::fmt::Display for $unit_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({}u64)", stringify!($unit_name), self.0)
            }
        }
    };
}

/// NOT FOR DIRECT USE
#[doc(hidden)]
#[macro_export]
macro_rules! unit_binops {
    ($unit_name:ident : $type:ty) => {
        impl<T> std::ops::Add<T> for $unit_name
        where
            T: Into<Self>,
        {
            type Output = Self;
            #[must_use]
            #[inline]
            fn add(self, rhs: T) -> Self::Output {
                Self(self.0 + rhs.into().0)
            }
        }

        impl<T> std::ops::Add<T> for &$unit_name
        where
            T: Into<$unit_name>,
        {
            type Output = <$unit_name as std::ops::Add<T>>::Output;
            #[must_use]
            #[inline]
            fn add(self, rhs: T) -> Self::Output {
                <$unit_name>::add(*self, rhs)
            }
        }

        impl std::ops::Add<$unit_name> for $type {
            type Output = $unit_name;
            #[must_use]
            #[inline]
            fn add(self, rhs: $unit_name) -> Self::Output {
                $unit_name(self + rhs.0)
            }
        }

        impl std::ops::Add<&$unit_name> for $type {
            type Output = $unit_name;
            #[must_use]
            #[inline]
            fn add(self, rhs: &$unit_name) -> Self::Output {
                $unit_name(self + rhs.0)
            }
        }

        impl<T> std::ops::Sub<T> for $unit_name
        where
            T: Into<Self>,
        {
            type Output = Self;
            #[must_use]
            #[inline]
            fn sub(self, rhs: T) -> Self::Output {
                Self(self.0 - rhs.into().0)
            }
        }

        impl<T> std::ops::Sub<T> for &$unit_name
        where
            T: Into<$unit_name>,
        {
            type Output = <$unit_name as std::ops::Sub<T>>::Output;
            #[must_use]
            #[inline]
            fn sub(self, rhs: T) -> Self::Output {
                <$unit_name>::sub(*self, rhs)
            }
        }

        impl std::ops::Sub<$unit_name> for $type {
            type Output = $unit_name;
            #[must_use]
            #[inline]
            fn sub(self, rhs: $unit_name) -> Self::Output {
                $unit_name(self - rhs.0)
            }
        }

        impl std::ops::Sub<&$unit_name> for $type {
            type Output = $unit_name;
            #[must_use]
            #[inline]
            fn sub(self, rhs: &$unit_name) -> Self::Output {
                $unit_name(self - rhs.0)
            }
        }

        impl<T> std::ops::Mul<T> for $unit_name
        where
            T: Into<Self>,
        {
            type Output = Self;
            #[must_use]
            #[inline]
            fn mul(self, rhs: T) -> Self::Output {
                Self(self.0 * rhs.into().0)
            }
        }

        impl<T> std::ops::Mul<T> for &$unit_name
        where
            T: Into<$unit_name>,
        {
            type Output = <$unit_name as std::ops::Mul<T>>::Output;
            #[must_use]
            #[inline]
            fn mul(self, rhs: T) -> Self::Output {
                <$unit_name>::mul(*self, rhs)
            }
        }

        impl std::ops::Mul<$unit_name> for $type {
            type Output = $unit_name;
            #[must_use]
            #[inline]
            fn mul(self, rhs: $unit_name) -> Self::Output {
                $unit_name(self * rhs.0)
            }
        }

        impl std::ops::Mul<&$unit_name> for $type {
            type Output = $unit_name;
            #[must_use]
            #[inline]
            fn mul(self, rhs: &$unit_name) -> Self::Output {
                $unit_name(self * rhs.0)
            }
        }

        impl<T> std::ops::Div<T> for $unit_name
        where
            T: Into<Self>,
        {
            type Output = Self;
            #[must_use]
            #[inline]
            fn div(self, rhs: T) -> Self::Output {
                Self(self.0 / rhs.into().0)
            }
        }

        impl<T> std::ops::Div<T> for &$unit_name
        where
            T: Into<$unit_name>,
        {
            type Output = <$unit_name as std::ops::Div<T>>::Output;
            #[must_use]
            #[inline]
            fn div(self, rhs: T) -> Self::Output {
                <$unit_name>::div(*self, rhs)
            }
        }

        impl std::ops::Div<$unit_name> for $type {
            type Output = $unit_name;
            #[must_use]
            #[inline]
            fn div(self, rhs: $unit_name) -> Self::Output {
                $unit_name(self / rhs.0)
            }
        }

        impl std::ops::Div<&$unit_name> for $type {
            type Output = $unit_name;
            #[must_use]
            #[inline]
            fn div(self, rhs: &$unit_name) -> Self::Output {
                $unit_name(self / rhs.0)
            }
        }

        impl std::ops::Rem for $unit_name {
            type Output = Self;
            #[must_use]
            #[inline]
            fn rem(self, rhs: Self) -> Self::Output {
                Self(self.0 % rhs.0)
            }
        }

        impl std::ops::RemAssign for $unit_name {
            #[inline]
            fn rem_assign(&mut self, rhs: Self) {
                self.0 %= rhs.0;
            }
        }

        impl<T> std::ops::AddAssign<T> for $unit_name
        where
            T: Into<Self>,
        {
            #[inline]
            fn add_assign(&mut self, rhs: T) {
                self.0 += rhs.into().0;
            }
        }

        impl<T> std::ops::SubAssign<T> for $unit_name
        where
            T: Into<Self>,
        {
            #[inline]
            fn sub_assign(&mut self, rhs: T) {
                self.0 -= rhs.into().0;
            }
        }

        impl<T> std::ops::MulAssign<T> for $unit_name
        where
            T: Into<Self>,
        {
            #[inline]
            fn mul_assign(&mut self, rhs: T) {
                self.0 *= rhs.into().0;
            }
        }

        impl<T> std::ops::DivAssign<T> for $unit_name
        where
            T: Into<Self>,
        {
            #[inline]
            fn div_assign(&mut self, rhs: T) {
                self.0 /= rhs.into().0;
            }
        }
    };
}

/// NOT FOR DIRECT USE
#[doc(hidden)]
#[macro_export]
macro_rules! unit_neg {
    ($unit_name:ident : $type:ty) => {
        impl std::ops::Neg for $unit_name {
            type Output = Self;
            #[must_use]
            #[inline]
            fn neg(self) -> Self::Output {
                Self(-self.0)
            }
        }
    };
}

/// NOT FOR DIRECT USE
#[doc(hidden)]
#[macro_export]
macro_rules! unit_serde {
    ($unit_name:ident : $type:ty) => {
        impl serde::Serialize for $unit_name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                self.0.serialize(serializer)
            }
        }
        impl<'de> serde::Deserialize<'de> for $unit_name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                <$type as serde::Deserialize>::deserialize(deserializer).map(|value| Self(value))
            }
        }
    };
}

/// NOT FOR DIRECT USE
#[doc(hidden)]
#[macro_export]
macro_rules! unit_num {
    ($unit_name:ident : $type:ty) => {
        impl num::Zero for $unit_name {
            fn zero() -> Self {
                Self(<$type as num::Zero>::zero())
            }
            fn is_zero(&self) -> bool {
                self.0.is_zero()
            }
        }
        impl num::One for $unit_name {
            fn one() -> Self {
                Self(<$type as num::One>::one())
            }
            fn is_one(&self) -> bool {
                self.0.is_one()
            }
        }
        impl num::Num for $unit_name {
            type FromStrRadixErr = <$type as num::Num>::FromStrRadixErr;
            fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                Ok(Self(<$type as num::Num>::from_str_radix(str, radix)?))
            }
        }
        impl num::ToPrimitive for $unit_name {
            fn to_i64(&self) -> Option<i64> {
                self.0.to_i64()
            }
            fn to_u64(&self) -> Option<u64> {
                self.0.to_u64()
            }
        }
        impl num::FromPrimitive for $unit_name {
            fn from_i64(n: i64) -> Option<Self> {
                Some(Self(<$type as num::FromPrimitive>::from_i64(n)?))
            }
            fn from_u64(n: u64) -> Option<Self> {
                Some(Self(<$type as num::FromPrimitive>::from_u64(n)?))
            }
            fn from_f64(n: f64) -> Option<Self> {
                Some(Self(<$type as num::FromPrimitive>::from_f64(n)?))
            }
        }
    };
}

/// NOT FOR DIRECT USE
#[doc(hidden)]
#[macro_export]
macro_rules! unit_structure {
    ($unit_name:ident : f64) => {
        impl $crate::structure::FrcStructure for $unit_name {
            const TYPE: &'static str = "float64";
            const SIZE: usize = 8;
            const SCHEMA_SUPPLIER: fn() -> String = || String::with_capacity(0);

            fn pack(&self, buffer: &mut Vec<u8>) {
                buffer.extend_from_slice(&f64::to_le_bytes(self.0));
            }

            fn unpack(buffer: &mut std::io::Cursor<&[u8]>) -> Self {
                let mut value_buffer = [0u8; Self::SIZE];
                let _ = std::io::Read::read_exact(buffer, &mut value_buffer);
                Self(f64::from_le_bytes(value_buffer))
            }
        }
    };
    ($unit_name:ident : i64) => {
        impl $crate::structure::FrcStructure for $unit_name {
            const TYPE: &'static str = "int64";
            const SIZE: usize = 8;
            const SCHEMA_SUPPLIER: fn() -> String = || String::with_capacity(0);

            fn pack(&self, buffer: &mut Vec<u8>) {
                buffer.extend_from_slice(&i64::to_le_bytes(self.0));
            }

            fn unpack(buffer: &mut std::io::Cursor<&[u8]>) -> Self {
                let mut value_buffer = [0u8; Self::SIZE];
                let _ = std::io::Read::read_exact(buffer, &mut value_buffer);
                Self(i64::from_le_bytes(value_buffer))
            }
        }
    };
    ($unit_name:ident : u64) => {
        impl $crate::structure::FrcStructure for $unit_name {
            const TYPE: &'static str = "uint64";
            const SIZE: usize = 8;
            const SCHEMA_SUPPLIER: fn() -> String = || String::with_capacity(0);

            fn pack(&self, buffer: &mut Vec<u8>) {
                buffer.extend_from_slice(&u64::to_le_bytes(self.0));
            }

            fn unpack(buffer: &mut std::io::Cursor<&[u8]>) -> Self {
                let mut value_buffer = [0u8; Self::SIZE];
                let _ = std::io::Read::read_exact(buffer, &mut value_buffer);
                Self(u64::from_le_bytes(value_buffer))
            }
        }
    };
}
