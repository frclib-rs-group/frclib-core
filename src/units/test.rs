use crate::units::{
    distance::{Feet, Meter},
    linear_velocity::MeterPerSecond,
    time::Second,
};

#[test]
fn conversion() {
    let meter = Meter::new(1.0);
    let feet = Feet::new(3.28084);
    assert_eq!(feet, meter);
    assert_eq!(meter, feet);
    let combined = feet + meter;
    assert_eq!(combined, Feet::new(6.56168));
    assert_eq!(combined, Meter::new(2.0));
    assert!(combined > feet);
}

#[test]
fn dim_analysis() {
    let meter = Meter::new(1.0);
    let second = Second::new(1.0);
    let meter_per_second: MeterPerSecond = meter / second;
    assert_eq!(meter_per_second, MeterPerSecond::new(1.0));
}
