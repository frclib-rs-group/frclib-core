use std::{
    fmt::Display,
    hash::{Hash, Hasher},
};

use bytes::{Bytes, BytesMut};
// use protobuf::descriptor::FileDescriptorProto;
use serde::{Deserialize, Serialize};

mod error;
#[cfg(test)]
mod test;
mod trait_impls;
mod traits;

use crate::structure::{FrcStructDesc, FrcStructure};
pub use error::FrcValueCastError;
pub use traits::IntoFrcValue;

pub use bytes;
pub use inventory;

use self::error::CastErrorReason;

/// Measured in microseconds <p>
/// depending on source can be from unix epoch or some arbitrary start time
pub type FrcTimestamp = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrcType {
    Void,
    Boolean,
    Int,
    Double,
    Float,
    String,
    BoolArray,
    IntArray,
    FloatArray,
    DoubleArray,
    StringArray,
    Raw,
    Struct(&'static str),
    // Protobuf,
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
            Self::BoolArray => write!(f, "BoolArray"),
            Self::IntArray => write!(f, "IntArray"),
            Self::FloatArray => write!(f, "FloatArray"),
            Self::DoubleArray => write!(f, "DoubleArray"),
            Self::StringArray => write!(f, "StringArray"),
            Self::Raw => write!(f, "Raw"),
            Self::Struct(name) => write!(f, "Struct({})", *name),
        }
    }
}

impl Serialize for FrcType {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        if let Self::Struct(name) = self {
            serializer.serialize_str(format!("struct({})", *name).as_str())
        } else {
            serializer.serialize_str(&self.to_string().to_lowercase().replace("array", "[]"))
        }
    }
}

impl<'a> Deserialize<'a> for FrcType {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as serde::Deserializer<'a>>::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let s = String::deserialize(deserializer)?;
        #[allow(clippy::match_same_arms)]
        match s.as_str() {
            "boolean" => Ok(Self::Boolean),
            "int" => Ok(Self::Int),
            "double" => Ok(Self::Double),
            "float" => Ok(Self::Float),
            "string" => Ok(Self::String),
            "json" => Ok(Self::String),
            "bool[]" => Ok(Self::BoolArray),
            "int[]" => Ok(Self::IntArray),
            "float[]" => Ok(Self::FloatArray),
            "double[]" => Ok(Self::DoubleArray),
            "string[]" => Ok(Self::StringArray),
            "raw" => Ok(Self::Raw),
            "rpc" => Ok(Self::Raw),
            "msgpack" => Ok(Self::Raw),
            // _ if s.starts_with("struct(") && s.ends_with(')') => {
            //     Ok(Self::Struct(&s[7..s.len() - 1]))
            // }
            _ => Err(serde::de::Error::custom(format!("Invalid FrcType: {s}"))),
        }
    }
}

/// A stardized value type for FRC data piping
///
/// This enum is used to represent all possible values that can be sent over the FRC data piping system
/// including
/// - ``Void``
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
/// - ``Raw``
/// - ``Struct``
///
/// Struct is a special type that carries metadata to allow decoding into their inner type or a dynamic object
///
/// Bytes are Boxed to keep the size of the enum small
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FrcValue {
    Void,
    Boolean(bool),
    Int(i64),
    Double(f64),
    Float(f32),
    String(Box<str>),
    BooleanArray(Box<[bool]>),
    IntArray(Box<[i64]>),
    FloatArray(Box<[f32]>),
    DoubleArray(Box<[f64]>),
    StringArray(Box<[Box<str>]>),
    Raw(Box<Bytes>),
    #[serde(skip_deserializing)]
    Struct(#[serde(skip)] &'static FrcStructDesc, Box<Bytes>),
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
            Self::Struct(desc, data) => write!(f, "Struct({}):{:?}", desc.type_str, data),
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
            Self::Struct(desc, data) => {
                desc.schema_supplier.hash(state);
                desc.type_str.hash(state);
                data.hash(state);
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
            Self::BooleanArray(_) => FrcType::BoolArray,
            Self::IntArray(_) => FrcType::IntArray,
            Self::FloatArray(_) => FrcType::FloatArray,
            Self::DoubleArray(_) => FrcType::DoubleArray,
            Self::StringArray(_) => FrcType::StringArray,
            Self::Raw(_) => FrcType::Raw,
            Self::Struct(desc, _) => FrcType::Struct(desc.type_str),
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
            Self::Raw(v) | Self::Struct(_, v) => v.is_empty(),
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
        )
    }
    /// Consumes itself to a timestamped value with the given timestamp
    #[must_use]
    pub const fn to_timestamped(self, timestamp: FrcTimestamp) -> FrcTimestampedValue {
        FrcTimestampedValue::new(timestamp, self)
    }
    /// Clones itself to a timestamped value with the given timestamp
    #[must_use]
    pub fn as_timestamped(&self, timestamp: FrcTimestamp) -> FrcTimestampedValue {
        FrcTimestampedValue::new(timestamp, self.clone())
    }
    #[must_use]
    pub fn to_tagged(self) -> FrcTaggedValue {
        FrcTaggedValue {
            r#type: self.get_type(),
            value: self,
        }
    }
    /// Creates a default value based on the type
    ///
    /// Types that will return none:
    ///     - Void
    ///     - Struct
    #[must_use]
    pub fn default_value(r#type: FrcType) -> Option<Self> {
        match r#type {
            FrcType::Boolean => Some(Self::Boolean(false)),
            FrcType::Int => Some(Self::Int(0)),
            FrcType::Double => Some(Self::Double(0.0)),
            FrcType::Float => Some(Self::Float(0.0)),
            FrcType::String => Some(Self::String(Box::default())),
            FrcType::BoolArray => Some(Self::BooleanArray(Box::default())),
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
    pub fn from_struct<T: FrcStructure>(value: &T) -> Self {
        let mut buffer = BytesMut::with_capacity(T::SIZE);
        value.pack(&mut buffer);
        Self::Struct(&T::DESCRIPTION, Box::new(buffer.freeze()))
    }

    /// # Errors
    pub fn try_into_struct<T: FrcStructure>(self) -> Result<T, FrcValueCastError> {
        let frc_type = self.get_type();
        match self {
            Self::Struct(_, mut buffer) => {
                if buffer.len() == T::SIZE {
                    Ok(T::unpack(&mut *buffer))
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct FrcTaggedValue {
    #[serde(rename = "type")]
    pub r#type: FrcType,
    pub value: FrcValue,
}
impl Display for FrcTaggedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.r#type, self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct FrcTimestampedValue {
    pub timestamp: FrcTimestamp,
    pub value: FrcValue,
}
impl Display for FrcTimestampedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}", self.value, self.timestamp)
    }
}
impl FrcTimestampedValue {
    #[must_use]
    pub const fn new(timestamp: FrcTimestamp, value: FrcValue) -> Self {
        Self { timestamp, value }
    }
    #[must_use]
    pub const fn get_type(&self) -> FrcType {
        self.value.get_type()
    }
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
    #[must_use]
    pub const fn is_array(&self) -> bool {
        self.value.is_array()
    }
    #[must_use]
    pub const fn is_after_timestamp(&self, timestamp: FrcTimestamp) -> bool {
        self.timestamp > timestamp
    }
    #[must_use]
    pub const fn is_after_other(&self, other: &Self) -> bool {
        self.timestamp > other.timestamp
    }
    #[must_use]
    pub const fn is_before_timestamp(&self, timestamp: FrcTimestamp) -> bool {
        self.timestamp < timestamp
    }
    #[must_use]
    pub const fn is_before_other(&self, other: &Self) -> bool {
        self.timestamp < other.timestamp
    }
}

#[derive(Debug, Clone)]
pub struct FrcEntry {
    pub timestamp: FrcTimestamp,
    pub value: FrcValue,
    pub key: &'static str,
}
impl Display for FrcEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {} for {}", self.value, self.timestamp, self.key)
    }
}
