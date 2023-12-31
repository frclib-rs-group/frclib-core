use crate::{unit, unit_conversion, unit_family};

unit!(Kilogram: float);
unit!(Gram: float);
unit!(Pound: float);
unit!(Ounce: float);

unit_conversion!(Kilogram(float) <-> Gram(float) ~ kilogram_to_gram);
unit_conversion!(Kilogram(float) <-> Pound(float) ~ kilogram_to_pound);
unit_conversion!(Kilogram(float) <-> Ounce(float) ~ kilogram_to_ounce);
unit_conversion!(Gram(float) <-> Pound(float) ~ gram_to_pound);
unit_conversion!(Gram(float) <-> Ounce(float) ~ gram_to_ounce);
unit_conversion!(Pound(float) <-> Ounce(float) ~ pound_to_ounce);

unit_family!(Mass(Kilogram): Gram, Pound, Ounce);

fn kilogram_to_gram(kilogram: f64) -> f64 {
    kilogram * 1000.0
}

fn kilogram_to_pound(kilogram: f64) -> f64 {
    kilogram * 2.20462
}

fn kilogram_to_ounce(kilogram: f64) -> f64 {
    kilogram * 35.274
}

fn gram_to_pound(gram: f64) -> f64 {
    kilogram_to_pound(gram / 1000.0)
}

fn gram_to_ounce(gram: f64) -> f64 {
    kilogram_to_ounce(gram / 1000.0)
}

fn pound_to_ounce(pound: f64) -> f64 {
    kilogram_to_ounce(pound / 2.20462)
}
