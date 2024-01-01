use crate::units::{
    distance::{Foot, Meter},
    linear_velocity::MetersPerSecond,
    time::Second,
};

#[test]
fn conversion() {
    let meter = Meter::new(1.0);
    let feet = Foot::new(3.28084);
    assert_eq!(feet, meter);
    assert_eq!(meter, feet);
    let combined = feet + meter;
    assert_eq!(combined, Foot::new(6.56168));
    assert_eq!(combined, Meter::new(2.0));
    assert!(combined > feet);
}

#[test]
fn dim_analysis() {
    let meter = Meter::new(1.0);
    let second = Second::new(1.0);
    let meter_per_second: MetersPerSecond = meter / second;
    assert_eq!(meter_per_second, MetersPerSecond::new(1.0));
}
