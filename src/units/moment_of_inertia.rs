use frclib_units_macros::{unit, unit_conversion};

unit!(KilogramSquareMeter, f64);
unit!(PoundSquareFoot, f64);

unit_conversion!(KilogramSquareMeter f64, PoundSquareFoot f64, kilogram_square_meter_to_pound_square_foot);

// unit_family!(MomentOfInertia: KilogramSquareMeter PoundSquareFoot);

fn kilogram_square_meter_to_pound_square_foot(kilogram_square_meter: f64) -> f64 {
    kilogram_square_meter * 0.204_816_143_622_5
}
