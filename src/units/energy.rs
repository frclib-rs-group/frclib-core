use crate::units::time::Second;
use crate::{unit, unit_conversion, unit_dim_analysis};

unit!(Joule: float);
unit!(Volt: float);
unit!(Amp: float);
unit!(Watt: float);
unit!(WattHour: float);
unit!(Ohm: float);

unit_conversion!(Joule(float) <-> WattHour(float) ~ joule_to_watthour);

fn joule_to_watthour(joule: f64) -> f64 {
    joule / 3600.0
}

unit_dim_analysis!(Volt * Ohm = Watt);
unit_dim_analysis!(Watt * Second = Joule);

impl Watt {
    #[must_use]
    pub fn to_watt_hour(&self, seconds: &Second) -> WattHour {
        WattHour::new(self.value() / (3600.0 / seconds.value()))
    }
}

impl WattHour {
    #[must_use]
    pub fn to_watt(&self, seconds: &Second) -> Watt {
        Watt::new(self.value() * (3600.0 / seconds.value()))
    }
}
