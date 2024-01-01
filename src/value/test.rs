
use crate::value::{FrcValue, IntoFrcValue};

#[test]
fn test_value_serde() {
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
    struct Test {
        field: FrcValue,
    }

    let test_inst = Test {
        field: FrcValue::from(1.0),
    };
    let json_str = r#"{"field":1.0}"#;

    let test_de: Test = serde_json::from_str(json_str).expect("Failed to deserialize");
    let test_ser = serde_json::to_string(&test_inst).expect("Failed to serialize");

    assert_eq!(json_str, test_ser);
    assert_eq!(test_inst, test_de);
}

const _: fn() = || {
    let _ = core::mem::transmute::<FrcValue, [u8; 24]>;
};

#[allow(trivial_casts, clippy::missing_assert_message)]
#[test]
fn test_into_frc_value() {
    fn assert_frc_value(v: impl IntoFrcValue, cmp: FrcValue) {{
        match (v.into_frc_value(), cmp) {
            (FrcValue::Boolean(a), FrcValue::Boolean(b)) => assert_eq!(a, b),
            (FrcValue::Float(a), FrcValue::Float(b)) => assert!((a-b).abs() < f32::EPSILON),
            (FrcValue::Double(a), FrcValue::Double(b)) => assert!((a-b).abs() < f64::EPSILON),
            (FrcValue::Int(a), FrcValue::Int(b)) => assert_eq!(a, b),
            (FrcValue::Void, FrcValue::Void) => (),
            (FrcValue::String(a), FrcValue::String(b)) => assert_eq!(a, b),
            (FrcValue::BooleanArray(a), FrcValue::BooleanArray(b)) => assert_eq!(a, b),
            (FrcValue::FloatArray(a), FrcValue::FloatArray(b)) => assert_eq!(a, b),
            (FrcValue::DoubleArray(a), FrcValue::DoubleArray(b)) => assert_eq!(a, b),
            (FrcValue::StringArray(a), FrcValue::StringArray(b)) => assert_eq!(a, b),
            (any_v, any_cmp) => panic!("assert_frc_value failed: {any_v:?} != {any_cmp:?}")
        }
    }}
    assert_frc_value(1.0f64, FrcValue::Double(1.0));
    assert_frc_value(1.0f32, FrcValue::Float(1.0));
    assert_frc_value(1i64, FrcValue::Int(1));
    assert_frc_value(1i32, FrcValue::Int(1));
    assert_frc_value(1i16, FrcValue::Int(1));
    assert_frc_value(1i8, FrcValue::Int(1));
    assert_frc_value(1u32, FrcValue::Int(1));
    assert_frc_value(1u16, FrcValue::Int(1));
    assert_frc_value(1u8, FrcValue::Int(1));
    assert_frc_value("1", FrcValue::String(Box::from("1")));
    assert_frc_value("1".to_string(), FrcValue::String(Box::from("1")));
    assert_frc_value(Box::from("1") as Box<str>, FrcValue::String(Box::from("1")));
    assert_frc_value(true, FrcValue::Boolean(true));
    assert_frc_value([true, false], FrcValue::BooleanArray(Box::from([true, false])));
    assert_frc_value(
        Box::from(["1", "2"]) as Box<[&str]>,
        FrcValue::StringArray(
            Box::from([Box::from("1"), Box::from("2")])
    ));
    assert_frc_value(
        vec!["1", "2"],
        FrcValue::StringArray(
            Box::from([Box::from("1"), Box::from("2")])
    ));
    assert_frc_value(None::<i64>, FrcValue::Void);
    assert_frc_value(Some(1), FrcValue::Int(1));
}