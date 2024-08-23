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
    const TYPE: FrcType;
    const COULD_BE_VOID: bool = false;
}