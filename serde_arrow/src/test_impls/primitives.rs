use super::macros::test_example;

test_example!(
    test_name = null,
    test_bytecode_deserialization = true,
    tracing_options = TracingOptions::default().allow_null_fields(true),
    field = GenericField::new("item", GenericDataType::Null, true),
    ty = (),
    values = [(), (), ()],
    // NOTE: arrow2 has an incorrect is_null impl for NullArray
    // nulls = [true, true, true],
);

test_example!(
    test_name = bool,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::Bool, false),
    ty = bool,
    values = [true, false],
    nulls = [false, false],
);

test_example!(
    test_name = nullable_bool,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::Bool, true),
    ty = Option<bool>,
    values = [Some(true), None, Some(false)],
    nulls = [false, true, false],
);

test_example!(
    test_name = u8,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::U8, false),
    ty = u8,
    values = [1, 2, 3, 4],
    nulls = [false, false, false, false],
);

test_example!(
    test_name = nullable_u8,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::U8, true),
    ty = Option<u8>,
    values = [Some(1), None, Some(3), Some(4)],
    nulls = [false, true, false, false],
);

test_example!(
    test_name = u16,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::U16, false),
    ty = u16,
    values = [1, 2, 3, 4],
    nulls = [false, false, false, false],
);

test_example!(
    test_name = nullable_u16,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::U16, true),
    ty = Option<u16>,
    values = [Some(1), None, Some(3), Some(4)],
    nulls = [false, true, false, false],
);

test_example!(
    test_name = u32,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::U32, false),
    ty = u32,
    values = [1, 2, 3, 4],
    nulls = [false, false, false, false],
);

test_example!(
    test_name = nullable_u32,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::U32, true),
    ty = Option<u32>,
    values = [Some(1), None, Some(3), Some(4)],
    nulls = [false, true, false, false],
);

test_example!(
    test_name = u64,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::U64, false),
    ty = u64,
    values = [1, 2, 3, 4],
    nulls = [false, false, false, false],
);

test_example!(
    test_name = nullable_u64,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::U64, true),
    ty = Option<u64>,
    values = [Some(1), None, Some(3), Some(4)],
    nulls = [false, true, false, false],
);

test_example!(
    test_name = i8,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::I8, false),
    ty = i8,
    values = [-1, 2, -3, 4],
    nulls = [false, false, false, false],
);

test_example!(
    test_name = nullable_i8,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::I8, true),
    ty = Option<i8>,
    values = [Some(-1), None, Some(3), Some(-4)],
    nulls = [false, true, false, false],
);

test_example!(
    test_name = i16,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::I16, false),
    ty = i16,
    values = [1, 2, 3, 4],
    nulls = [false, false, false, false],
);

test_example!(
    test_name = nullable_i16,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::I16, true),
    ty = Option<i16>,
    values = [Some(-1), None, Some(3), Some(-4)],
    nulls = [false, true, false, false],
);

test_example!(
    test_name = i32,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::I32, false),
    ty = i32,
    values = [-1, 2, -3, 4],
    nulls = [false, false, false, false],
);

test_example!(
    test_name = nullable_i32,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::I32, true),
    ty = Option<i32>,
    values = [Some(-1), None, Some(3), Some(-4)],
    nulls = [false, true, false, false],
);

test_example!(
    test_name = i64,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::I64, false),
    ty = i64,
    values = [-1, 2, -3, 4],
    nulls = [false, false, false, false],
);

test_example!(
    test_name = nullable_i64,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::I64, true),
    ty = Option<i64>,
    values = [Some(-1), None, Some(3), Some(-4)],
    nulls = [false, true, false, false],
);

test_example!(
    test_name = f32,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::F32, false),
    ty = f32,
    values = [-1.0, 2.0, -3.0, 4.0],
    nulls = [false, false, false, false],
);

test_example!(
    test_name = f32_from_f64,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::F64, false),
    overwrite_field = GenericField::new("item", GenericDataType::F32, false),
    ty = f64,
    values = [-1.0, 2.0, -3.0, 4.0],
    nulls = [false, false, false, false],
);

test_example!(
    test_name = nullable_f32,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::F32, true),
    ty = Option<f32>,
    values = [Some(-1.0), None, Some(3.0), Some(-4.0)],
    nulls = [false, true, false, false],
);

test_example!(
    test_name = f64,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::F64, false),
    ty = f64,
    values = [-1.0, 2.0, -3.0, 4.0],
    nulls = [false, false, false, false],
);

test_example!(
    test_name = nullable_f64,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::F64, true),
    ty = Option<f64>,
    values = [Some(-1.0), None, Some(3.0), Some(-4.0)],
    nulls = [false, true, false, false],
);

test_example!(
    test_name = f64_from_f32,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::F32, false),
    overwrite_field = GenericField::new("item", GenericDataType::F64, false),
    ty = f32,
    values = [-1.0, 2.0, -3.0, 4.0],
    nulls = [false, false, false, false],
);

test_example!(
    test_name = f16_from_f32,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::F32, false),
    overwrite_field = GenericField::new("item", GenericDataType::F16, false),
    ty = f32,
    values = [-1.0, 2.0, -3.0, 4.0],
    nulls = [false, false, false, false],
);

test_example!(
    test_name = f16_from_f64,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::F64, false),
    overwrite_field = GenericField::new("item", GenericDataType::F16, false),
    ty = f64,
    values = [-1.0, 2.0, -3.0, 4.0],
    nulls = [false, false, false, false],
);

test_example!(
    test_name = str,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::LargeUtf8, false),
    ty = String,
    values = [
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("d")
    ],
    nulls = [false, false, false, false],
);

test_example!(
    test_name = nullable_str,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::LargeUtf8, true),
    ty = Option<String>,
    values = [Some(String::from("a")), None, None, Some(String::from("d"))],
    nulls = [false, true, true, false],
);

test_example!(
    test_name = str_u32,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::LargeUtf8, false),
    overwrite_field = GenericField::new("item", GenericDataType::Utf8, false),
    ty = String,
    values = [
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("d")
    ],
    nulls = [false, false, false, false],
);

test_example!(
    test_name = nullable_str_u32,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::LargeUtf8, true),
    overwrite_field = GenericField::new("item", GenericDataType::Utf8, true),
    ty = Option<String>,
    values = [Some(String::from("a")), None, None, Some(String::from("d"))],
    nulls = [false, true, true, false],
);

test_example!(
    test_name = newtype_i64,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::I64, false),
    ty = I64,
    values = [I64(-1), I64(2), I64(3), I64(-4)],
    nulls = [false, false, false, false],
    define = {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct I64(i64);
    },
);

test_example!(
    test_name = u8_to_u16,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::U8, false),
    overwrite_field = GenericField::new("item", GenericDataType::U16, false),
    ty = u8,
    values = [1, 2, 3, 4],
    nulls = [false, false, false, false],
);

test_example!(
    test_name = u32_to_i64,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::U32, false),
    overwrite_field = GenericField::new("item", GenericDataType::I64, false),
    ty = u32,
    values = [1, 2, 3, 4],
    nulls = [false, false, false, false],
);

test_example!(
    test_name = chars,
    test_bytecode_deserialization = true,
    field = GenericField::new("item", GenericDataType::U32, false),
    ty = char,
    values = ['a', 'b', 'c'],
    nulls = [false, false, false],
);
