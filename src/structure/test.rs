use super::*;

#[derive(Debug, PartialEq, Clone, Copy)]
struct SubStruct {
    value: f64,
}
impl FrcStructure for SubStruct {
    const TYPE: &'static str = "Meter";
    const SIZE: usize = 8;
    const SCHEMA_SUPPLIER: fn() -> String = || "float64 value".to_owned();

    fn pack(&self, buffer: &mut Vec<u8>) {
        self.value.pack(buffer);
    }

    fn unpack(buffer: &mut std::io::Cursor<&[u8]>) -> Self {
        Self {
            value: <f64 as FrcStructure>::unpack(buffer),
        }
    }
}

inventory::submit! { SubStruct::DESCRIPTION }

#[test]
#[cfg(feature = "value-union")]
fn test_structures() {
    use crate as frclib_core;
    use crate::value::FrcValue;

    #[derive(Debug, PartialEq, Clone, Copy, FrcStructure, Default)]
    #[repr(u8)]
    enum MyEnum {
        #[default]
        A,
        B,
        C = 100,
    }

    #[derive(Debug, PartialEq, Clone, Copy, FrcStructure)]
    struct NestedTestStruct {
        enum_val: MyEnum,
        boolean: bool,
        test_struct: SubStruct,
        test_struct_arr: [SubStruct; 2],
        integer: i32,
        string: [char; 128],
    }

    let test_struct = SubStruct { value: 1.0 };
    let value = FrcValue::from_struct(&test_struct);
    let test_struct2: SubStruct = value.try_into_struct().expect("Failed to convert");
    assert_eq!(test_struct, test_struct2);

    let nested_struct = NestedTestStruct {
        boolean: true,
        test_struct,
        test_struct_arr: [test_struct; 2],
        integer: 1,
        string: ['a'; 128],
        enum_val: MyEnum::B,
    };
    let value = FrcValue::from_struct(&nested_struct);
    let nested_struct2: NestedTestStruct = value.try_into_struct().expect("Failed to convert");
    assert_eq!(nested_struct, nested_struct2);

    FrcStructDescDB::add(FrcStructDesc {
        schema_supplier: || "bool idk;".to_owned(),
        type_str: "proc",
        size: 1,
    });

    //iterate through all inventory values of FrcStructureDescription and print type_str
    for struct_desc in inventory::iter::<FrcStructDesc> {
        println!(
            "{} {{{}}}",
            struct_desc.type_str,
            (struct_desc.schema_supplier)()
        );
    }
}

// #[test]
// fn test_schema() {
//     const SCHEMA: &str = "enum {a=1, b=2} int8 val[3]";
//     let fields = parse_schema_toplevel(SCHEMA);
//     assert_eq!(fields.len(), 1);
//     assert_eq!(
//         fields[0],
//         ("val".to_owned(), 0, StructureFieldTypes::Int8(3))
//     );
// }

// #[test]
// fn test_schema_advanced() {
//     const SCHEMA: &str = "Rotation2d rot; Translation2d trans;";
//     FrcStructDescDB::add(FrcStructDesc {
//         schema_supplier: || "double value".to_owned(),
//         type_str: "Rotation2d",
//         size: 8,
//     });
//     FrcStructDescDB::add(FrcStructDesc {
//         schema_supplier: || "double x; double y".to_owned(),
//         type_str: "Translation2d",
//         size: 16,
//     });
//     let fields = parse_schema_toplevel(SCHEMA);
//     assert_eq!(fields.len(), 3);
//     assert_eq!(
//         fields,
//         vec![
//             (
//                 "rot.value".to_owned(),
//                 0usize,
//                 StructureFieldTypes::Float64(1)
//             ),
//             (
//                 "trans.x".to_owned(),
//                 8usize,
//                 StructureFieldTypes::Float64(1)
//             ),
//             (
//                 "trans.y".to_owned(),
//                 16usize,
//                 StructureFieldTypes::Float64(1)
//             )
//         ]
//     );
// }
