//! Example how to convert arrow2 arrays to arrow arrays using the FFI interface
//!
//! Relevant docs:
//!
//! - https://docs.rs/arrow/latest/arrow/ffi/index.html
//! - https://docs.rs/arrow2/latest/arrow2/ffi/fn.export_array_to_c.html
//! - https://docs.rs/arrow2/latest/arrow2/ffi/fn.export_field_to_c.html
//!
use arrow::array::ArrayData;
use arrow2::{
    array::{Array, Int32Array},
    datatypes::Field,
};

#[derive(Debug, Clone, Copy)]
struct PanicOnError;

impl<E: std::fmt::Display> From<E> for PanicOnError {
    fn from(value: E) -> Self {
        panic!("{value}")
    }
}

fn main() -> Result<(), PanicOnError> {
    let arrow2_array = Int32Array::from(&[Some(1), None, Some(3)]);
    let arrow2_array = Box::new(arrow2_array);
    let arrow2_field = Field::new("a", arrow2_array.data_type().clone(), true);

    let arrow_array = convert_arrow2_to_arrow(arrow2_array, &arrow2_field);
    let array_data = ArrayData::try_from(arrow_array)?;
    array_data.validate_full()?;

    // to create a generic dyn Array use arrow::array::make_array()
    let arrow_array = arrow::array::Int32Array::from(array_data);

    {
        use arrow::array::Array;

        println!("len:         {}", arrow_array.len());
        println!("nulls count: {}", arrow_array.null_count());
        println!("array[0]:    {}", arrow_array.value(0));
        println!("array[1]:    {}", arrow_array.value(1));
        println!("array[2]:    {}", arrow_array.value(2));
    }
    Ok(())
}

fn convert_arrow2_to_arrow(array: Box<dyn Array>, field: &Field) -> arrow::ffi::ArrowArray {
    let array = arrow2::ffi::export_array_to_c(array);
    let schema = arrow2::ffi::export_field_to_c(field);

    let array = unsafe { std::mem::transmute::<_, arrow::ffi::FFI_ArrowArray>(array) };
    let schema = unsafe { std::mem::transmute::<_, arrow::ffi::FFI_ArrowSchema>(schema) };

    arrow::ffi::ArrowArray::new(array, schema)
}
