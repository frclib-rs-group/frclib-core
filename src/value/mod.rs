//! This module contains the [``FrcValue``](crate::value::FrcValue) type which is used to represent values in various frc protocols.

use std::{
    fmt::Display,
    hash::{Hash, Hasher},
    io::Cursor,
};

mod error;
#[cfg(test)]
mod test;
mod trait_impls;
mod traits;

use crate::structure::{FrcStructure, FrcStructureBytes};
pub use error::FrcValueCastError;
pub use traits::IntoFrcValue;
pub use traits::StaticallyFrcTyped;

pub use inventory;

use self::error::CastErrorReason;

/// Measured in microseconds
///
/// depending on source can be from unix epoch or some arbitrary start time
pub type FrcTimestamp = u64;

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrcType {
    Void,
    Raw,
    Boolean,
    Int,
    Double,
    Float,
    String,
    BooleanArray,
    IntArray,
    FloatArray,
    DoubleArray,
    StringArray,
    Struct,
    StructArray,
}
impl Display for FrcType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Void => write!(f, "Void"),
            Self::Boolean => write!(f, "Boolean"),
            Self::Int => write!(f, "Int"),
            Self::Double => write!(f, "Double"),
            Self::Float => write!(f, "Float"),
            Self::String => write!(f, "String"),
            Self::BooleanArray => write!(f, "BooleanArray"),
            Self::IntArray => write!(f, "IntArray"),
            Self::FloatArray => write!(f, "FloatArray"),
            Self::DoubleArray => write!(f, "DoubleArray"),
            Self::StringArray => write!(f, "StringArray"),
            Self::Raw => write!(f, "Raw"),
            Self::Struct => write!(f, "Struct"),
            Self::StructArray => write!(f, "StructArray"),
        }
    }
}

type BoxVec<T> = Box<[T]>;
type BoxStr = Box<str>;

/// A stardized value type for FRC data piping
///
/// This enum is used to represent all possible values that can be sent over the FRC data piping system
/// including
/// - ``Void``
/// - ``Raw``
/// - ``Boolean``
/// - ``Int``
/// - ``Double``
/// - ``Float``
/// - ``String``
/// - ``BoolArray``
/// - ``IntArray``
/// - ``FloatArray``
/// - ``DoubleArray``
/// - ``StringArray``
/// - ``Struct``
/// - ``StructArray``
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq)]
pub enum FrcValue {
    /// A value representing nothing, most api's that accept [`FrcValue`]
    /// simply ignore this variant but it is important for full compatability
    /// with things like json and messagepack
    Void,
    /// An immutable contigous chunk of bytes
    Raw(BoxVec<u8>),
    /// A single boolean
    Boolean(bool),
    /// A signed 64bit integer, this is the only integer available due to the limitations
    /// of some parts of the wpilib ecosystem
    Int(i64),
    Double(f64),
    Float(f32),
    String(Box<str>),
    BooleanArray(BoxVec<bool>),
    IntArray(BoxVec<i64>),
    FloatArray(BoxVec<f32>),
    DoubleArray(BoxVec<f64>),
    /// A contigous immutable list of heap allocated strings,
    /// this variant uses the most heap allocations and should be avoided if possible
    StringArray(BoxVec<BoxStr>),
    /// A single struct represented by a contigous chunk of bytes and a schema.
    /// The bytes are stored behind nested boxes to try and keep the enum size small
    Struct(Box<FrcStructureBytes>),
    /// A list of structs represented by a contigous chunk of bytes, a schema and count.
    /// The bytes are stored behind nested boxes to try and keep the enum size small
    StructArray(Box<FrcStructureBytes>),
}
impl Display for FrcValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Void => write!(f, "Void"),
            Self::Boolean(v) => write!(f, "{v}"),
            Self::Int(v) => write!(f, "{v}"),
            Self::Double(v) => write!(f, "{v}"),
            Self::Float(v) => write!(f, "{v}"),
            Self::String(v) => write!(f, "{v}"),
            Self::BooleanArray(v) => write!(f, "{v:?}"),
            Self::IntArray(v) => write!(f, "{v:?}"),
            Self::FloatArray(v) => write!(f, "{v:?}"),
            Self::DoubleArray(v) => write!(f, "{v:?}"),
            Self::StringArray(v) => write!(f, "{v:?}"),
            Self::Raw(v) => write!(f, "{v:?}"),
            Self::Struct(bytes) => write!(f, "Struct({}):{:?}", bytes.desc.type_str, bytes.data),
            Self::StructArray(bytes) => write!(
                f,
                "Struct({})[{}]:{:?}",
                bytes.desc.type_str, bytes.count, bytes.data
            ),
        }
    }
}
impl Hash for FrcValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Void => {}
            Self::Boolean(v) => v.hash(state),
            Self::Int(v) => v.hash(state),
            Self::Double(v) => v.to_bits().hash(state),
            Self::Float(v) => v.to_bits().hash(state),
            Self::String(v) => v.hash(state),
            Self::BooleanArray(v) => v.hash(state),
            Self::IntArray(v) => v.hash(state),
            Self::FloatArray(v) => v.iter().for_each(|v| v.to_bits().hash(state)),
            Self::DoubleArray(v) => v.iter().for_each(|v| v.to_bits().hash(state)),
            Self::StringArray(v) => v.hash(state),
            Self::Raw(v) => v.hash(state),
            Self::Struct(bytes) | Self::StructArray(bytes) => {
                bytes.desc.schema_supplier.hash(state);
                bytes.desc.type_str.hash(state);
                bytes.data.hash(state);
            }
        }
    }
}
impl FrcValue {
    ///Returns the type enum of the value, a more memory efficient way of checking the type
    #[must_use]
    pub const fn get_type(&self) -> FrcType {
        match self {
            Self::Void => FrcType::Void,
            Self::Boolean(_) => FrcType::Boolean,
            Self::Int(_) => FrcType::Int,
            Self::Double(_) => FrcType::Double,
            Self::Float(_) => FrcType::Float,
            Self::String(_) => FrcType::String,
            Self::BooleanArray(_) => FrcType::BooleanArray,
            Self::IntArray(_) => FrcType::IntArray,
            Self::FloatArray(_) => FrcType::FloatArray,
            Self::DoubleArray(_) => FrcType::DoubleArray,
            Self::StringArray(_) => FrcType::StringArray,
            Self::Raw(_) => FrcType::Raw,
            Self::Struct(_) => FrcType::Struct,
            Self::StructArray(_) => FrcType::StructArray,
        }
    }
    ///Creates an empty Binary
    #[must_use]
    pub const fn empty() -> Self {
        Self::Void
    }
    /// Always false if not a collection or binary
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        match self {
            Self::Void => true,
            Self::String(v) => v.is_empty(),
            Self::BooleanArray(v) => v.is_empty(),
            Self::IntArray(v) => v.is_empty(),
            Self::DoubleArray(v) => v.is_empty(),
            Self::FloatArray(v) => v.is_empty(),
            Self::StringArray(v) => v.is_empty(),
            Self::Raw(v) => v.is_empty(),
            Self::Struct(bytes) | Self::StructArray(bytes) => bytes.data.is_empty(),
            _ => false,
        }
    }
    ///Binary is false
    #[must_use]
    pub const fn is_array(&self) -> bool {
        matches!(
            self,
            Self::BooleanArray(_)
                | Self::IntArray(_)
                | Self::DoubleArray(_)
                | Self::FloatArray(_)
                | Self::StringArray(_)
                | Self::StructArray(_)
        )
    }
    /// Consumes itself to a timestamped value with the given timestamp
    #[must_use]
    pub const fn to_timestamped(self, timestamp: FrcTimestamp) -> FrcTimestampedValue {
        FrcTimestampedValue::new(timestamp, self)
    }
    /// Creates a default value based on the type
    ///
    /// Types that will return none:
    ///     - `Void`
    ///     - `Struct`
    ///     - `StructArray`
    #[must_use]
    pub fn default_value(r#type: FrcType) -> Option<Self> {
        match r#type {
            FrcType::Boolean => Some(Self::Boolean(false)),
            FrcType::Int => Some(Self::Int(0)),
            FrcType::Double => Some(Self::Double(0.0)),
            FrcType::Float => Some(Self::Float(0.0)),
            FrcType::String => Some(Self::String(Box::default())),
            FrcType::BooleanArray => Some(Self::BooleanArray(Box::default())),
            FrcType::IntArray => Some(Self::IntArray(Box::default())),
            FrcType::FloatArray => Some(Self::FloatArray(Box::default())),
            FrcType::DoubleArray => Some(Self::DoubleArray(Box::default())),
            FrcType::StringArray => Some(Self::StringArray(Box::default())),
            FrcType::Raw => Some(Self::Raw(Box::default())),
            _ => None,
        }
    }
}

impl FrcValue {
    /// Converts the given [``FrcStructure``](crate::structure::FrcStructure) into a [``FrcValue``](FrcValue)
    pub fn from_struct<T: FrcStructure>(value: &T) -> Self {
        let mut buffer = Vec::with_capacity(T::SIZE);
        value.pack(&mut buffer);
        Self::Struct(Box::new(FrcStructureBytes::from_parts(
            &T::DESCRIPTION,
            1,
            buffer.into_boxed_slice(),
        )))
    }

    /// # Errors
    /// Returns an error if the value is not a struct or the struct is not the correct type
    pub fn try_into_struct<T: FrcStructure>(self) -> Result<T, FrcValueCastError> {
        let frc_type = self.get_type();
        match self {
            Self::Struct(bytes) => {
                let buffer = bytes.data;
                if buffer.len() == T::SIZE {
                    let mut cursor = Cursor::new(buffer.as_ref());
                    Ok(T::unpack(&mut cursor))
                } else {
                    Err(FrcValueCastError::InvalidCastTo(
                        frc_type,
                        T::TYPE,
                        CastErrorReason::Deserialization,
                    ))
                }
            }
            _ => Err(FrcValueCastError::InvalidCastTo(
                frc_type,
                T::TYPE,
                CastErrorReason::Type,
            )),
        }
    }

    /// Converts the given [``FrcStructure``](crate::structure::FrcStructure) slice/array into a [``FrcValue``](FrcValue)
    pub fn from_struct_array<T: FrcStructure>(values: &[T]) -> Self {
        let mut buffer = Vec::with_capacity(T::SIZE * values.len());
        for value in values {
            value.pack(&mut buffer);
        }
        Self::StructArray(Box::new(FrcStructureBytes::from_parts(
            &T::DESCRIPTION,
            values.len(),
            buffer.into_boxed_slice(),
        )))
    }

    /// # Errors
    /// Returns an error if the value is not a struct or the struct is not the correct type
    pub fn try_into_struct_array<T: FrcStructure>(self) -> Result<Vec<T>, FrcValueCastError> {
        let frc_type = self.get_type();
        match self {
            Self::StructArray(bytes) => {
                let buffer = bytes.data;
                if buffer.len() % T::SIZE == 0 {
                    let mut cursor = Cursor::new(buffer.as_ref());
                    let mut values = Vec::with_capacity(buffer.len() / T::SIZE);
                    while cursor.position() < buffer.len() as u64 {
                        values.push(T::unpack(&mut cursor));
                    }
                    Ok(values)
                } else {
                    Err(FrcValueCastError::InvalidCastTo(
                        frc_type,
                        T::TYPE,
                        CastErrorReason::Deserialization,
                    ))
                }
            }
            _ => Err(FrcValueCastError::InvalidCastTo(
                frc_type,
                T::TYPE,
                CastErrorReason::Type,
            )),
        }
    }
}

/// An [``FrcValue``](FrcValue) with a [``FrcTimestamp``](FrcTimestamp) attached,
/// important for passing to logging systems
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct FrcTimestampedValue {
    /// The timestamp of the value,
    /// typically the uptime of the robot if running on the robot
    /// and the unix epoch if running on desktop (non sim)
    pub timestamp: FrcTimestamp,
    /// The value
    pub value: FrcValue,
}
impl Display for FrcTimestampedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}", self.value, self.timestamp)
    }
}
impl FrcTimestampedValue {
    /// Creates a new timestamped value
    #[must_use]
    pub const fn new(timestamp: FrcTimestamp, value: FrcValue) -> Self {
        Self { timestamp, value }
    }
    /// Checks if the timestamp is after the given timestamp
    #[must_use]
    pub const fn is_after_timestamp(&self, timestamp: FrcTimestamp) -> bool {
        self.timestamp > timestamp
    }
    /// Checks if the timestamp is after the given timestamped value
    #[must_use]
    pub const fn is_after_other(&self, other: &Self) -> bool {
        self.timestamp > other.timestamp
    }
    /// Checks if the timestamp is before the given timestamp
    #[must_use]
    pub const fn is_before_timestamp(&self, timestamp: FrcTimestamp) -> bool {
        self.timestamp < timestamp
    }
    /// Checks if the timestamp is before the given timestamped value
    #[must_use]
    pub const fn is_before_other(&self, other: &Self) -> bool {
        self.timestamp < other.timestamp
    }
}

/// A timestamped value with a key attached,
/// important for passing to pub/sub logging systems
#[derive(Debug, Clone)]
pub struct FrcEntry {
    /// The timestamp of the value,
    /// typically the uptime of the robot if running on the robot
    /// and the unix epoch if running on desktop
    pub timestamp: FrcTimestamp,
    /// The value
    pub value: FrcValue,
    /// The key
    pub key: &'static str,
}
impl Display for FrcEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {} for {}", self.value, self.timestamp, self.key)
    }
}
