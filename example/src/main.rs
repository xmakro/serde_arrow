use std::{collections::HashMap, convert::TryFrom};

use arrow::{
    csv,
    datatypes::{DataType, Schema},
};
use chrono::NaiveDateTime;
use serde::Serialize;

macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };
    ($($key:expr => $value:expr),*) => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(m.insert($key.into(), $value.into());)*
            m
        }
    };
}

#[derive(Serialize)]
struct Example {
    int8: i8,
    int32: i32,
    float32: f32,
    date64: NaiveDateTime,
    boolean: bool,
    #[serde(flatten)]
    extra: HashMap<String, i32>,
}

fn main() -> serde_arrow::Result<()> {
    let examples = vec![
        Example {
            float32: 1.0,
            int8: 1,
            int32: 4,
            date64: NaiveDateTime::from_timestamp(0, 0),
            boolean: true,
            extra: hashmap! { "a" => 2 },
        },
        Example {
            float32: 2.0,
            int8: 2,
            int32: 5,
            date64: NaiveDateTime::from_timestamp(5 * 24 * 60 * 60, 0),
            boolean: false,
            extra: hashmap! { "a" => 3 },
        },
    ];

    let mut schema = serde_arrow::trace_schema(&examples)?;
    schema.set_data_type("date", DataType::Date64);

    let schema = Schema::try_from(schema)?;

    let batch = serde_arrow::to_record_batch(&examples, schema)?;
    csv::Writer::new(std::io::stdout()).write(&batch)?;

    Ok(())
}