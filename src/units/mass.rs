use frclib_units_macros::{unit, unit_conversion};

unit!(Kilogram, f64);
unit!(Gram, f64);
unit!(Pound, f64);
unit!(Ounce, f64);

unit_conversion!(Kilogram f64, Gram f64, kilogram_to_gram);
unit_conversion!(Kilogram f64, Pound f64, kilogram_to_pound);
unit_conversion!(Kilogram f64, Ounce f64, kilogram_to_ounce);
unit_conversion!(Gram f64, Pound f64, gram_to_pound);
unit_conversion!(Gram f64, Ounce f64, gram_to_ounce);
unit_conversion!(Pound f64, Ounce f64, pound_to_ounce);

// unit_family!(Mass: Kilogram Gram Pound Ounce);

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
