use crate::value::FrcValue;

use super::FrcType;

/// A trait for converting a value into an [``FrcValue``](crate::value::FrcValue).
pub trait IntoFrcValue: Send + Sync {
    #[allow(missing_docs)]
    fn into_frc_value(self) -> FrcValue;
}

impl<T: Into<FrcValue> + Send + Sync> IntoFrcValue for T {
    fn into_frc_value(self) -> FrcValue {
        self.into()
    }
}

/// A trait allowing static type checking on some types that implement [``IntoFrcValue``](crate::value::IntoFrcValue).
#[allow(unused)]
pub trait StaticallyFrcTyped: IntoFrcValue {
    /// The type of the value, used for static type checking.
    const TYPE: FrcType;
    /// Whether the value could be void.
    /// 
    /// This may be superseeded by type unions but for now this will be used.
    /// If this is true think of it as adding and Option<> around the type.
    const COULD_BE_VOID: bool = false;
}