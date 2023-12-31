use crate::value::FrcValue;



pub trait IntoFrcValue: Send + Sync {
    fn into_frc_value(self) -> FrcValue;
}

impl<T: Into<FrcValue> + Send + Sync> IntoFrcValue for T {
    fn into_frc_value(self) -> FrcValue {
        self.into()
    }
}