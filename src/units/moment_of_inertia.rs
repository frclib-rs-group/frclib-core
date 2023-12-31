use crate::{unit, unit_conversion, unit_family};

unit!(KilogramSquareMeter: float);
unit!(PoundSquareFoot: float);

unit_conversion!(KilogramSquareMeter(float) <-> PoundSquareFoot(float) ~ kilogram_square_meter_to_pound_square_foot);

unit_family!(MomentOfInertia(KilogramSquareMeter): PoundSquareFoot);

fn kilogram_square_meter_to_pound_square_foot(kilogram_square_meter: f64) -> f64 {
    kilogram_square_meter * 0.204_816_143_622_5
}
