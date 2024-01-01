use crate::value::{
    error::{CastErrorReason, FrcValueCastError},
    FrcTimestampedValue, FrcType, FrcValue,
};

impl From<f64> for FrcValue {
    fn from(v: f64) -> Self {
        Self::Double(v)
    }
}
impl From<f32> for FrcValue {
    fn from(v: f32) -> Self {
        Self::Float(v)
    }
}
impl From<i64> for FrcValue {
    fn from(v: i64) -> Self {
        Self::Int(v)
    }
}
impl From<i32> for FrcValue {
    fn from(v: i32) -> Self {
        Self::Int(i64::from(v))
    }
}
impl From<i16> for FrcValue {
    fn from(v: i16) -> Self {
        Self::Int(i64::from(v))
    }
}
impl From<i8> for FrcValue {
    fn from(v: i8) -> Self {
        Self::Int(i64::from(v))
    }
}
impl TryFrom<u64> for FrcValue {
    type Error = FrcValueCastError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(Self::Int(i64::try_from(value).map_err(|_| FrcValueCastError::InvalidCastFrom(
            stringify!(u64),
            FrcType::Int,
            CastErrorReason::Overflow,
        ))?))
    }
}
impl From<u32> for FrcValue {
    fn from(v: u32) -> Self {
        Self::Int(i64::from(v))
    }
}
impl From<u16> for FrcValue {
    fn from(v: u16) -> Self {
        Self::Int(i64::from(v))
    }
}
impl From<u8> for FrcValue {
    fn from(v: u8) -> Self {
        Self::Int(i64::from(v))
    }
}
impl From<bool> for FrcValue {
    fn from(v: bool) -> Self {
        Self::Boolean(v)
    }
}
impl From<String> for FrcValue {
    fn from(v: String) -> Self {
        Self::String(Box::from(v))
    }
}
impl From<Box<str>> for FrcValue {
    fn from(v: Box<str>) -> Self {
        Self::String(v)
    }
}
impl From<&str> for FrcValue {
    fn from(v: &str) -> Self {
        Self::String(Box::from(v))
    }
}
impl From<Vec<bool>> for FrcValue {
    fn from(v: Vec<bool>) -> Self {
        Self::BooleanArray(v.into_boxed_slice())
    }
}
impl From<Box<[bool]>> for FrcValue {
    fn from(v: Box<[bool]>) -> Self {
        Self::BooleanArray(v)
    }
}
impl From<Vec<i64>> for FrcValue {
    fn from(v: Vec<i64>) -> Self {
        Self::IntArray(v.into_boxed_slice())
    }
}
impl From<Box<[i64]>> for FrcValue {
    fn from(v: Box<[i64]>) -> Self {
        Self::IntArray(v)
    }
}
impl From<Vec<i32>> for FrcValue {
    fn from(v: Vec<i32>) -> Self {
        Self::IntArray(v.iter().map(|v| i64::from(*v)).collect())
    }
}
impl From<Box<[i32]>> for FrcValue {
    fn from(v: Box<[i32]>) -> Self {
        Self::IntArray(v.iter().map(|v| i64::from(*v)).collect())
    }
}
impl From<Vec<i16>> for FrcValue {
    fn from(v: Vec<i16>) -> Self {
        Self::IntArray(v.iter().map(|v| i64::from(*v)).collect())
    }
}
impl From<Box<[i16]>> for FrcValue {
    fn from(v: Box<[i16]>) -> Self {
        Self::IntArray(v.iter().map(|v| i64::from(*v)).collect())
    }
}
impl From<Vec<i8>> for FrcValue {
    fn from(v: Vec<i8>) -> Self {
        Self::IntArray(v.iter().map(|v| i64::from(*v)).collect())
    }
}
impl From<Box<[i8]>> for FrcValue {
    fn from(v: Box<[i8]>) -> Self {
        Self::IntArray(v.iter().map(|v| i64::from(*v)).collect())
    }
}
impl TryFrom<Vec<u64>> for FrcValue {
    type Error = FrcValueCastError;
    fn try_from(v: Vec<u64>) -> Result<Self, Self::Error> {
        Ok(Self::IntArray(
            v.into_iter()
                .map(|v| i64::try_from(v).map_err(|_| FrcValueCastError::InvalidCastFrom(
                    stringify!(u64),
                    FrcType::Int,
                    CastErrorReason::Overflow,
                )))
                .collect::<Result<Box<[i64]>, FrcValueCastError>>()?,
        ))
    }
}
impl TryFrom<Box<[u64]>> for FrcValue {
    type Error = FrcValueCastError;
    fn try_from(v: Box<[u64]>) -> Result<Self, Self::Error> {
        Ok(Self::IntArray(
            v.iter()
                .map(|v| i64::try_from(*v).map_err(|_| FrcValueCastError::InvalidCastFrom(
                    stringify!(u64),
                    FrcType::Int,
                    CastErrorReason::Overflow,
                )))
                .collect::<Result<Box<[i64]>, FrcValueCastError>>()?,
        ))
    }
}
impl From<Vec<u32>> for FrcValue {
    fn from(v: Vec<u32>) -> Self {
        Self::IntArray(v.iter().map(|v| i64::from(*v)).collect())
    }
}
impl From<Box<[u32]>> for FrcValue {
    fn from(v: Box<[u32]>) -> Self {
        Self::IntArray(v.iter().map(|v| i64::from(*v)).collect())
    }
}
impl From<Vec<u16>> for FrcValue {
    fn from(v: Vec<u16>) -> Self {
        Self::IntArray(v.iter().map(|v| i64::from(*v)).collect())
    }
}
impl From<Box<[u16]>> for FrcValue {
    fn from(v: Box<[u16]>) -> Self {
        Self::IntArray(v.iter().map(|v| i64::from(*v)).collect())
    }
}
impl From<Vec<f32>> for FrcValue {
    fn from(v: Vec<f32>) -> Self {
        Self::FloatArray(v.into_boxed_slice())
    }
}
impl From<Box<[f32]>> for FrcValue {
    fn from(v: Box<[f32]>) -> Self {
        Self::FloatArray(v)
    }
}
impl From<Vec<f64>> for FrcValue {
    fn from(v: Vec<f64>) -> Self {
        Self::DoubleArray(v.into_boxed_slice())
    }
}
impl From<Box<[f64]>> for FrcValue {
    fn from(v: Box<[f64]>) -> Self {
        Self::DoubleArray(v)
    }
}
impl From<Vec<String>> for FrcValue {
    fn from(v: Vec<String>) -> Self {
        Self::StringArray(v.into_iter().map(Box::from).collect())
    }
}
impl From<Vec<&str>> for FrcValue {
    fn from(v: Vec<&str>) -> Self {
        Self::StringArray(v.iter().map(|s| Box::from(*s)).collect())
    }
}
impl From<Box<[&str]>> for FrcValue {
    fn from(v: Box<[&str]>) -> Self {
        Self::StringArray(v.iter().map(|s| Box::from(*s)).collect())
    }
}
impl From<Vec<Box<str>>> for FrcValue {
    fn from(v: Vec<Box<str>>) -> Self {
        Self::StringArray(Box::from(v))
    }
}
impl From<Box<[Box<str>]>> for FrcValue {
    fn from(v: Box<[Box<str>]>) -> Self {
        Self::StringArray(v)
    }
}
impl From<Box<[u8]>> for FrcValue {
    fn from(v: Box<[u8]>) -> Self {
        Self::Raw(Box::new(Bytes::from(v)))
    }
}
impl From<FrcTimestampedValue> for FrcValue {
    fn from(v: FrcTimestampedValue) -> Self {
        v.value
    }
}

impl TryFrom<FrcValue> for f64 {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::Double(v) => Ok(v),
            FrcValue::Float(v) => Ok(Self::from(v)),
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(f64),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for f32 {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::Double(v) => {
                #[allow(clippy::cast_possible_truncation)]
                let v_f32 = v as Self;
                if v_f32.is_infinite() {
                    if v < 0.0 {
                        Err(FrcValueCastError::InvalidCastTo(
                            value.get_type(),
                            stringify!(f32),
                            CastErrorReason::Overflow,
                        ))
                    } else {
                        Err(FrcValueCastError::InvalidCastTo(
                            value.get_type(),
                            stringify!(f32),
                            CastErrorReason::Underflow,
                        ))
                    }
                } else {
                    Ok(v_f32)
                }
            }
            FrcValue::Float(v) => Ok(v),
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(f32),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for i64 {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::Int(v) => Ok(v),
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(i64),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for i32 {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::Int(v) => {
                Self::try_from(v).map_err(|_| {
                    if v < 0 {
                        FrcValueCastError::InvalidCastTo(
                            value.get_type(),
                            stringify!(i32),
                            CastErrorReason::Underflow,
                        )
                    } else {
                        FrcValueCastError::InvalidCastTo(
                            value.get_type(),
                            stringify!(i32),
                            CastErrorReason::Overflow,
                        )
                    }
                })
            }
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(i32),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for i16 {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::Int(v) => {
                Self::try_from(v).map_err(|_| {
                    if v < 0 {
                        FrcValueCastError::InvalidCastTo(
                            value.get_type(),
                            stringify!(i16),
                            CastErrorReason::Underflow,
                        )
                    } else {
                        FrcValueCastError::InvalidCastTo(
                            value.get_type(),
                            stringify!(i16),
                            CastErrorReason::Overflow,
                        )
                    }
                })
            }
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(i16),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for i8 {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::Int(v) => {
                Self::try_from(v).map_err(|_| {
                    if v < 0 {
                        FrcValueCastError::InvalidCastTo(
                            value.get_type(),
                            stringify!(i8),
                            CastErrorReason::Underflow,
                        )
                    } else {
                        FrcValueCastError::InvalidCastTo(
                            value.get_type(),
                            stringify!(i8),
                            CastErrorReason::Overflow,
                        )
                    }
                })
            }
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(i8),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for u64 {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::Int(v) => {
                Self::try_from(v).map_err(|_| FrcValueCastError::InvalidCastTo(
                    value.get_type(),
                    stringify!(u64),
                    CastErrorReason::Underflow,
                ))
            }
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(u64),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for u32 {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::Int(v) => {
                Self::try_from(v).map_err(|_| {
                    if v < 0 {
                        FrcValueCastError::InvalidCastTo(
                            value.get_type(),
                            stringify!(u32),
                            CastErrorReason::Underflow,
                        )
                    } else {
                        FrcValueCastError::InvalidCastTo(
                            value.get_type(),
                            stringify!(u32),
                            CastErrorReason::Overflow,
                        )
                    }
                })
            }
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(u32),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for u16 {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::Int(v) => {
                Self::try_from(v).map_err(|_| {
                    if v < 0 {
                        FrcValueCastError::InvalidCastTo(
                            value.get_type(),
                            stringify!(u16),
                            CastErrorReason::Underflow,
                        )
                    } else {
                        FrcValueCastError::InvalidCastTo(
                            value.get_type(),
                            stringify!(u16),
                            CastErrorReason::Overflow,
                        )
                    }
                })
            }
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(u16),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for u8 {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::Int(v) => {
                Self::try_from(v).map_err(|_| {
                    if v < 0 {
                        FrcValueCastError::InvalidCastTo(
                            value.get_type(),
                            stringify!(u8),
                            CastErrorReason::Underflow,
                        )
                    } else {
                        FrcValueCastError::InvalidCastTo(
                            value.get_type(),
                            stringify!(u8),
                            CastErrorReason::Overflow,
                        )
                    }
                })
            }
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(u8),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for bool {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::Boolean(v) => Ok(v),
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(bool),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for String {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::String(v) => Ok(v.to_string()),
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(String),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for Box<str> {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::String(v) => Ok(v),
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(Box<str>),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<f64> {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::DoubleArray(va) => Ok(Self::from(va)),
            FrcValue::FloatArray(va) => Ok(va.iter().map(
                |v| f64::from(*v)).collect()),
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(Vec<f64>),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<f32> {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        let value_type = value.get_type();
        match value {
            FrcValue::DoubleArray(va) => {
                let mut ret_vec = Self::with_capacity(va.len());
                for v in va.iter() {
                    if *v > f64::from(f32::MAX) {
                        return Err(FrcValueCastError::InvalidCastTo(
                            value_type,
                            stringify!(Vec<f32>),
                            CastErrorReason::Overflow,
                        ));
                    } else if *v < f64::from(f32::MIN) {
                        return Err(FrcValueCastError::InvalidCastTo(
                            value_type,
                            stringify!(Vec<f32>),
                            CastErrorReason::Underflow,
                        ));
                    }
                    #[allow(clippy::cast_possible_truncation)]
                    ret_vec.push(*v as f32);
                }
                Ok(ret_vec)
            }
            FrcValue::FloatArray(v) => Ok(v.into_vec()),
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(Vec<f32>),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<i64> {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::IntArray(va) => Ok(va.into_vec()),
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(Vec<i64>),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<i32> {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::IntArray(ref va) => {
                let mut ret_vec = Self::with_capacity(va.len());
                for v in va.iter() {
                    ret_vec.push(i32::try_from(*v).map_err(|_| {
                        if *v < 0 {
                            FrcValueCastError::InvalidCastTo(
                                value.get_type(),
                                stringify!(Vec<i32>),
                                CastErrorReason::Underflow,
                            )
                        } else {
                            FrcValueCastError::InvalidCastTo(
                                value.get_type(),
                                stringify!(Vec<i32>),
                                CastErrorReason::Overflow,
                            )
                        }
                    })?);
                }
                Ok(ret_vec)
            }
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(Vec<i32>),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<i16> {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::IntArray(ref va) => {
                let mut ret_vec = Self::with_capacity(va.len());
                for v in va.iter() {
                    ret_vec.push(i16::try_from(*v).map_err(|_| {
                        if *v < 0 {
                            FrcValueCastError::InvalidCastTo(
                                value.get_type(),
                                stringify!(Vec<i16>),
                                CastErrorReason::Underflow,
                            )
                        } else {
                            FrcValueCastError::InvalidCastTo(
                                value.get_type(),
                                stringify!(Vec<i16>),
                                CastErrorReason::Overflow,
                            )
                        }
                    })?);
                }
                Ok(ret_vec)
            }
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(Vec<i16>),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<i8> {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::IntArray(ref va) => {
                let mut ret_vec = Self::with_capacity(va.len());
                for v in va.iter() {
                    ret_vec.push(i8::try_from(*v).map_err(|_| {
                        if *v < 0 {
                            FrcValueCastError::InvalidCastTo(
                                value.get_type(),
                                stringify!(Vec<i8>),
                                CastErrorReason::Underflow,
                            )
                        } else {
                            FrcValueCastError::InvalidCastTo(
                                value.get_type(),
                                stringify!(Vec<i8>),
                                CastErrorReason::Overflow,
                            )
                        }
                    })?);
                }
                Ok(ret_vec)
            }
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(Vec<i8>),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<u64> {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::IntArray(ref va) => {
                let mut ret_vec = Self::with_capacity(va.len());
                for v in va.iter() {
                    ret_vec.push(u64::try_from(*v).map_err(|_| {
                        if *v < 0 {
                            FrcValueCastError::InvalidCastTo(
                                value.get_type(),
                                stringify!(Vec<u64>),
                                CastErrorReason::Underflow,
                            )
                        } else {
                            FrcValueCastError::InvalidCastTo(
                                value.get_type(),
                                stringify!(Vec<u64>),
                                CastErrorReason::Overflow,
                            )
                        }
                    })?);
                }
                Ok(ret_vec)
            }
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(Vec<u64>),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<u32> {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::IntArray(ref va) => {
                let mut ret_vec = Self::with_capacity(va.len());
                for v in va.iter() {
                    ret_vec.push(u32::try_from(*v).map_err(|_| {
                        if *v < 0 {
                            FrcValueCastError::InvalidCastTo(
                                value.get_type(),
                                stringify!(Vec<u32>),
                                CastErrorReason::Underflow,
                            )
                        } else {
                            FrcValueCastError::InvalidCastTo(
                                value.get_type(),
                                stringify!(Vec<u32>),
                                CastErrorReason::Overflow,
                            )
                        }
                    })?);
                }
                Ok(ret_vec)
            }
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(Vec<u32>),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<u16> {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::IntArray(ref va) => {
                let mut ret_vec = Self::with_capacity(va.len());
                for v in va.iter() {
                    ret_vec.push(u16::try_from(*v).map_err(|_| {
                        if *v < 0 {
                            FrcValueCastError::InvalidCastTo(
                                value.get_type(),
                                stringify!(Vec<u16>),
                                CastErrorReason::Underflow,
                            )
                        } else {
                            FrcValueCastError::InvalidCastTo(
                                value.get_type(),
                                stringify!(Vec<u16>),
                                CastErrorReason::Overflow,
                            )
                        }
                    })?);
                }
                Ok(ret_vec)
            }
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(Vec<u16>),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<u8> {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::IntArray(ref va) => {
                let mut ret_vec = Self::with_capacity(va.len());
                for v in va.iter() {
                    ret_vec.push(u8::try_from(*v).map_err(|_| {
                        if *v < 0 {
                            FrcValueCastError::InvalidCastTo(
                                value.get_type(),
                                stringify!(Vec<u8>),
                                CastErrorReason::Underflow,
                            )
                        } else {
                            FrcValueCastError::InvalidCastTo(
                                value.get_type(),
                                stringify!(Vec<u8>),
                                CastErrorReason::Overflow,
                            )
                        }
                    })?);
                }
                Ok(ret_vec)
            }
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(Vec<u8>),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<bool> {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::BooleanArray(va) => Ok(va.into_vec()),
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(Vec<bool>),
                CastErrorReason::Type,
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<String> {
    type Error = FrcValueCastError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::StringArray(va) => Ok(va.iter().map(std::string::ToString::to_string).collect()),
            _ => Err(FrcValueCastError::InvalidCastTo(
                value.get_type(),
                stringify!(Vec<String>),
                CastErrorReason::Type,
            )),
        }
    }
}


use bytes::Bytes;
use rmpv::Value as MPValue;

impl TryFrom<MPValue> for FrcValue {
    type Error = FrcValueCastError;
    fn try_from(value: MPValue) -> Result<Self, Self::Error> {
        match value {
            MPValue::Nil => Ok(Self::Void),
            MPValue::Boolean(b) => Ok(Self::Boolean(b)),
            MPValue::Integer(i) => Ok(Self::Int(i.as_i64().unwrap_or_default())),
            MPValue::F32(f) => Ok(Self::Float(f)),
            MPValue::F64(f) => Ok(Self::Double(f)),
            MPValue::String(s) => Ok(Self::String(s.to_string().into_boxed_str())),
            MPValue::Binary(b) => Ok(Self::Raw(Box::new(bytes::Bytes::from(b)))),
            MPValue::Array(a) => {
                let mut arr = Vec::with_capacity(a.len());
                for v in a {
                    arr.push(Self::try_from(v)?);
                }
                if arr.is_empty() {
                    return Ok(Self::empty());
                }
                let first_type = arr[0].get_type();
                if arr.iter().all(|v| v.get_type() == first_type) {
                    match first_type {
                        FrcType::Boolean => Ok(Self::BooleanArray(
                            arr.into_iter().map(
                                bool::try_from
                            ).collect::<Result<Box<[_]>, FrcValueCastError>>()?,
                        )),
                        FrcType::Int => Ok(Self::IntArray(
                            arr.into_iter().map(
                                i64::try_from
                            ).collect::<Result<Box<[_]>, FrcValueCastError>>()?
                        )),
                        FrcType::Float => Ok(Self::FloatArray(
                            arr.into_iter().map(
                                f32::try_from
                            ).collect::<Result<Box<[_]>, FrcValueCastError>>()?
                        )),
                        FrcType::Double => Ok(Self::DoubleArray(
                            arr.into_iter().map(
                                f64::try_from
                            ).collect::<Result<Box<[_]>, FrcValueCastError>>()?
                        )),
                        FrcType::String => Ok(Self::StringArray(
                            arr.into_iter().map(
                                |v| String::try_from(v).map(Box::from)
                            ).collect::<Result<Box<[_]>, FrcValueCastError>>()?
                        )),
                        any => Err(FrcValueCastError::InvalidCastTo(
                            any,
                            stringify!(MPValue),
                            CastErrorReason::Type,
                        )),
                    }
                } else {
                    Err(FrcValueCastError::InvalidCastTo(
                        first_type,
                        stringify!(MPValue),
                        CastErrorReason::Type,
                    ))
                }
            }
            _ => Err(FrcValueCastError::UnrepresentableCast),
        }
    }
}

impl From<FrcValue> for MPValue {
    fn from(value: FrcValue) -> Self {
        match value {
            FrcValue::Void => Self::Nil,
            FrcValue::Boolean(b) => Self::Boolean(b),
            FrcValue::Int(i) => Self::Integer(i.into()),
            FrcValue::Float(f) => Self::F32(f),
            FrcValue::Double(f) => Self::F64(f),
            FrcValue::String(s) => Self::String(s.to_string().into()),
            FrcValue::BooleanArray(a) => Self::Array(
                a.iter()
                    .map(|v| Self::Boolean(*v))
                    .collect::<Vec<Self>>(),
            ),
            FrcValue::IntArray(a) => Self::Array(
                a.iter()
                    .map(|v| Self::Integer((*v).into()))
                    .collect::<Vec<Self>>(),
            ),
            FrcValue::FloatArray(a) => Self::Array(
                a.iter()
                    .map(|v| Self::F32(*v))
                    .collect::<Vec<Self>>(),
            ),
            FrcValue::DoubleArray(a) => Self::Array(
                a.iter()
                    .map(|v| Self::F64(*v))
                    .collect::<Vec<Self>>(),
            ),
            FrcValue::StringArray(a) => Self::Array(
                a.iter()
                    .map(|v| Self::String(v.to_string().into()))
                    .collect::<Vec<Self>>(),
            ),
            FrcValue::Raw(b) | FrcValue::Struct(_, b) => Self::Binary(b.to_vec()),
        }
    }
}

use serde_json::Value as JSONValue;

#[allow(clippy::match_wildcard_for_single_variants)]
impl TryFrom<JSONValue> for FrcValue {
    type Error = FrcValueCastError;
    fn try_from(value: JSONValue) -> Result<Self, Self::Error> {
        match value {
            JSONValue::Null => Ok(Self::Void),
            JSONValue::Bool(b) => Ok(Self::Boolean(b)),
            JSONValue::Number(n) => {
                if n.is_i64() {
                    Ok(Self::Int(n.as_i64().unwrap_or_default()))
                } else if n.is_f64() {
                    Ok(Self::Double(n.as_f64().unwrap_or_default()))
                } else {
                    Err(FrcValueCastError::InvalidCastTo(
                        FrcType::Double,
                        stringify!(JSONValue),
                        CastErrorReason::Type,
                    ))
                }
            }
            JSONValue::String(s) => Ok(Self::String(s.into_boxed_str())),
            JSONValue::Array(a) => {
                let mut arr = Vec::with_capacity(a.len());
                for v in a {
                    arr.push(Self::try_from(v)?);
                }
                if arr.is_empty() {
                    return Ok(Self::empty());
                }
                let first_type = arr[0].get_type();
                if arr.iter().all(|v| v.get_type() == first_type) {
                    match first_type {
                        FrcType::Boolean => Ok(Self::BooleanArray(
                            arr.into_iter().map(
                                bool::try_from
                            ).collect::<Result<Box<[_]>, FrcValueCastError>>()?,
                        )),
                        FrcType::Int => Ok(Self::IntArray(
                            arr.into_iter().map(
                                i64::try_from
                            ).collect::<Result<Box<[_]>, FrcValueCastError>>()?
                        )),
                        FrcType::Float => Ok(Self::FloatArray(
                            arr.into_iter().map(
                                f32::try_from
                            ).collect::<Result<Box<[_]>, FrcValueCastError>>()?
                        )),
                        FrcType::Double => Ok(Self::DoubleArray(
                            arr.into_iter().map(
                                f64::try_from
                            ).collect::<Result<Box<[_]>, FrcValueCastError>>()?
                        )),
                        FrcType::String => Ok(Self::StringArray(
                            arr.into_iter().map(
                                |v| String::try_from(v).map(Box::from)
                            ).collect::<Result<Box<[_]>, FrcValueCastError>>()?
                        )),
                        any => Err(FrcValueCastError::InvalidCastTo(
                            any,
                            stringify!(JSONValue),
                            CastErrorReason::Type,
                        )),
                    }
                } else {
                    Err(FrcValueCastError::InvalidCastTo(
                        first_type,
                        stringify!(JSONValue),
                        CastErrorReason::Type,
                    ))
                }
            }
            _ => Err(FrcValueCastError::UnrepresentableCast),
        }
    }
}

#[allow(clippy::fallible_impl_from, clippy::cast_sign_loss)]
impl From<FrcValue> for JSONValue {
    fn from(value: FrcValue) -> Self {
        match value {
            FrcValue::Void => Self::Null,
            FrcValue::Boolean(b) => Self::Bool(b),
            FrcValue::Int(i) => Self::Number({
                if i < 0 {
                    serde_json::Number::from(i)
                } else {
                    serde_json::Number::from(i as u64)
                }
            }),
            FrcValue::Float(f) => Self::Number(
                serde_json::Number::from_f64(f64::from(f))
                    .unwrap_or_else(|| serde_json::Number::from(0))
            ),
            FrcValue::Double(f) => Self::Number(
                serde_json::Number::from_f64(f)
                    .unwrap_or_else(|| serde_json::Number::from(0))
            ),
            FrcValue::String(s) => Self::String(s.into()),
            FrcValue::Raw(b) => Self::Array(
                b.iter()
                    .map(|v| Self::Number((i64::from(*v)).into()))
                    .collect::<Vec<Self>>(),
            ),
            FrcValue::BooleanArray(a) => {
                Self::Array(a.iter().map(|v| Self::Bool(*v)).collect::<Vec<Self>>())
            }
            FrcValue::IntArray(a) => Self::Array(
                a.iter()
                    .map(|v| {
                        Self::Number({
                            if *v < 0 {
                                serde_json::Number::from(*v)
                            } else {
                                serde_json::Number::from(*v as u64)
                            }
                        })
                    })
                    .collect::<Vec<Self>>(),
            ),
            FrcValue::FloatArray(a) => Self::Array(
                a.iter()
                    .map(|v| Self::Number(
                        serde_json::Number::from_f64(f64::from(*v))
                            .unwrap_or_else(|| serde_json::Number::from(0))
                    ))
                    .collect::<Vec<Self>>(),
            ),
            FrcValue::DoubleArray(a) => Self::Array(
                a.iter()
                    .map(|v| Self::Number(
                        serde_json::Number::from_f64(*v)
                            .unwrap_or_else(|| serde_json::Number::from(0))
                    ))
                    .collect::<Vec<Self>>(),
            ),
            FrcValue::StringArray(a) => Self::Array(
                a.iter()
                    .map(|v| Self::String(v.to_string()))
                    .collect::<Vec<Self>>(),
            ),
            FrcValue::Struct(_, b) => Self::Array(
                b.iter()
                    .map(|v| Self::Number((u64::from(*v)).into()))
                    .collect::<Vec<Self>>(),
            ),
        }
    }
}
