use crate::value::FrcValue;


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