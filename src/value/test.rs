
use crate::value::FrcValue;

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

// warns if enum size changes
static_assertions::assert_eq_size!(FrcValue, [u8; 24]);

