//! arrow2 dependent functionality (requires one the `arrow2-*` features)
//!
//! Functions to convert `arrow2  arrays from and into Rust objects.
//!
//! The functions come in pairs: some work on single  arrays, i.e., the series
//! of a data frames, some work on multiples arrays, i.e., data frames
//! themselves.
//!
//! | operation | mutliple arrays |  single array  |
//! |---|-----------------|----------------|
//! | schema tracing | [serialize_into_fields] | [serialize_into_field] |
//! | Rust to arrow2 | [serialize_into_arrays] | [serialize_into_array] |
//! | arrow2 to Rust | [deserialize_from_arrays] | [deserialize_from_array] |
//!
//! Functions working on multiple arrays expect sequences of records in Rust,
//! e.g., a vector of structs. Functions working on single arrays expect vectors
//! of arrays elements.
//!
pub(crate) mod display;
pub(crate) mod schema;
pub(crate) mod sinks;
pub(crate) mod sources;
mod type_support;

#[cfg(test)]
mod test;

use serde::{Deserialize, Serialize};

use crate::{
    impls::arrow2::{array::Array, datatypes::Field},
    internal::{
        self,
        error::Result,
        schema::{GenericField, TracingOptions},
        source::{deserialize_from_source, AddOuterSequenceSource},
    },
};

use self::{
    sinks::Arrow2PrimitiveBuilders,
    sources::{build_dynamic_source, build_record_source},
};

/// Determine the schema (as a list of fields) for the given items
///
/// `items` should be given in the form a list of records (e.g., a vector of
/// structs).
///
/// ```rust
/// # use serde_arrow::impls::arrow2::datatypes::{DataType, Field};
/// # use serde::Serialize;
/// # use serde_arrow::arrow2::serialize_into_fields;
/// #
/// ##[derive(Serialize)]
/// struct Record {
///     a: Option<f32>,
///     b: u64,
/// }
///
/// let items = vec![
///     Record { a: Some(1.0), b: 2},
///     // ...
/// ];
///
/// let fields = serialize_into_fields(&items, Default::default()).unwrap();
/// let expected = vec![
///     Field::new("a", DataType::Float32, true),
///     Field::new("b", DataType::UInt64, false),
/// ];
///
/// assert_eq!(fields, expected);
/// ```
/// To correctly record the type information make sure to:
///
/// - include values for `Option<T>`
/// - include all variants of an enum
/// - include at least single element of a list or a map
///
pub fn serialize_into_fields<T>(items: &T, options: TracingOptions) -> Result<Vec<Field>>
where
    T: Serialize + ?Sized,
{
    internal::serialize_into_fields(items, options)?
        .iter()
        .map(|f| f.try_into())
        .collect()
}

/// Build arrays from the given items
///
/// `items` should be given in the form a list of records (e.g., a vector of
/// structs).
///
/// To build arrays record by record use [ArraysBuilder].
///
/// ```rust
/// # use serde::Serialize;
/// # use serde_arrow::arrow2::{serialize_into_fields, serialize_into_arrays};
/// #
/// ##[derive(Serialize)]
/// struct Record {
///     a: Option<f32>,
///     b: u64,
/// }
///
/// let items = vec![
///     Record { a: Some(1.0), b: 2},
///     // ...
/// ];
///
/// let fields = serialize_into_fields(&items, Default::default()).unwrap();
/// let arrays = serialize_into_arrays(&fields, &items).unwrap();
///
/// assert_eq!(arrays.len(), 2);
/// ```
///
pub fn serialize_into_arrays<T>(fields: &[Field], items: &T) -> Result<Vec<Box<dyn Array>>>
where
    T: Serialize + ?Sized,
{
    let fields = fields
        .iter()
        .map(GenericField::try_from)
        .collect::<Result<Vec<_>>>()?;
    internal::serialize_into_arrays::<T, Arrow2PrimitiveBuilders>(&fields, items)
}

/// Deserialize a type from the given arrays
///
/// The type should be a list of records (e.g., a vector of structs).
///
/// ```rust
/// # use serde_arrow::impls::arrow2::datatypes::{Field, DataType};
/// # use serde::{Serialize, Deserialize};
/// # use serde_arrow::arrow2::{
/// #   serialize_into_fields,
/// #   serialize_into_arrays,
/// #   deserialize_from_arrays,
/// # };
/// #
/// ##[derive(Deserialize, Serialize)]
/// struct Record {
///     a: Option<f32>,
///     b: u64,
/// }
///
/// // provide an example record to get the field information
/// let fields = serialize_into_fields(
///     &[Record { a: Some(1.0), b: 2}],
///     Default::default(),
/// ).unwrap();
/// # let items = &[Record { a: Some(1.0), b: 2}];
/// # let arrays = serialize_into_arrays(&fields, &items).unwrap();
/// #
///
/// // deserialize the records from arrays
/// let items: Vec<Record> = deserialize_from_arrays(&fields, &arrays).unwrap();
/// ```
///
pub fn deserialize_from_arrays<'de, T, A>(fields: &'de [Field], arrays: &'de [A]) -> Result<T>
where
    T: Deserialize<'de>,
    A: AsRef<dyn Array>,
{
    let source = build_record_source(fields, arrays)?;
    deserialize_from_source(source)
}

/// Determine the schema of an object that represents a single array
///
/// Example:
///
/// ```rust
/// # use serde_arrow::impls::arrow2::{array::Array, datatypes::{DataType, Field}};
/// # use serde::Serialize;
/// # use serde_arrow::arrow2::{serialize_into_field, serialize_into_array};
/// #
/// let items: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];
///
/// let field = serialize_into_field(&items, "floats", Default::default()).unwrap();
/// assert_eq!(field, Field::new("floats", DataType::Float32, false));
/// ```
///
pub fn serialize_into_field<T>(items: &T, name: &str, options: TracingOptions) -> Result<Field>
where
    T: Serialize + ?Sized,
{
    let field = internal::serialize_into_field(items, name, options)?;
    (&field).try_into()
}

/// Serialize an object that represents a single array into an array
///
/// Example:
///
/// ```rust
/// # use serde_arrow::impls::arrow2::{array::Array, datatypes::{DataType, Field}};
/// # use serde::Serialize;
/// # use serde_arrow::arrow2::{serialize_into_field, serialize_into_array};
/// #
/// let items: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];
///
/// let field = Field::new("floats", DataType::Float32, false);
/// let array = serialize_into_array(&field, &items).unwrap();
///
/// assert_eq!(array.len(), 4);
/// ```
///
pub fn serialize_into_array<T>(field: &Field, items: &T) -> Result<Box<dyn Array>>
where
    T: Serialize + ?Sized,
{
    let field: GenericField = field.try_into()?;
    internal::serialize_into_array::<T, Arrow2PrimitiveBuilders>(&field, items)
}

/// Deserialize an object that represents a single array from an array
///
/// /// Determine the schema of an object that represents a single array
///
/// Example:
///
/// ```rust
/// # use serde_arrow::impls::arrow2::{array::Array, datatypes::{DataType, Field}};
/// # use serde::Serialize;
/// # use serde_arrow::arrow2::{
/// #   serialize_into_field,
/// #   serialize_into_array,
/// #   deserialize_from_array,
/// # };
/// let field = Field::new("floats", DataType::Float32, false);
/// # let base_items: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];
/// # let array = serialize_into_array(&field, &base_items).unwrap();
/// let items: Vec<f32> = deserialize_from_array(&field, &array).unwrap();
/// ```
///
pub fn deserialize_from_array<'de, T, A>(field: &Field, array: A) -> Result<T>
where
    T: Deserialize<'de>,
    A: AsRef<dyn Array> + 'de,
{
    let source = build_dynamic_source(field, array.as_ref())?;
    let source = AddOuterSequenceSource::new(source);
    deserialize_from_source(source)
}

/// Build arrays record by record
///
/// Usage:
///
/// ```rust
/// # use serde_arrow::impls::arrow2::datatypes::{DataType, Field};
/// # use serde::Serialize;
/// # use serde_arrow::arrow2::{ArraysBuilder};
/// #
/// ##[derive(Serialize)]
/// struct Record {
///     a: Option<f32>,
///     b: u64,
/// }

/// let fields = vec![
///     Field::new("a", DataType::Float32, true),
///     Field::new("b", DataType::UInt64, false),
/// ];
/// let mut builder = ArraysBuilder::new(&fields).unwrap();
///
/// for item in &[
///     Record { a: Some(1.0), b: 2},
///     Record { a: Some(3.0), b: 4},
///     Record { a: Some(5.0), b: 5},
///     // ...
/// ] {
///     builder.push(item).unwrap()
/// }
///  
/// let arrays = builder.build_arrays().unwrap();
/// assert_eq!(arrays.len(), 2);
/// ```
pub struct ArraysBuilder {
    inner: internal::ArraysBuilder<Arrow2PrimitiveBuilders>,
}

impl std::fmt::Debug for ArraysBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArraysBuilder<...>")
    }
}

impl ArraysBuilder {
    /// Build a new ArraysBuilder for the given fields
    ///
    /// This method may fail when unsupported data types are encountered in the
    /// given fields.
    ///
    pub fn new(fields: &[Field]) -> Result<Self> {
        let fields = fields
            .iter()
            .map(GenericField::try_from)
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            inner: internal::ArraysBuilder::new(fields)?,
        })
    }

    /// Add a single record to the arrays
    ///
    pub fn push<T: Serialize + ?Sized>(&mut self, item: &T) -> Result<()> {
        self.inner.push(item)
    }

    /// Add multiple records to the arrays
    ///
    pub fn extend<T: Serialize + ?Sized>(&mut self, items: &T) -> Result<()> {
        self.inner.extend(items)
    }

    /// Build the arrays built from the rows pushed to far.
    ///
    /// This operation will reset the underlying buffers and start a new batch.
    ///
    pub fn build_arrays(&mut self) -> Result<Vec<Box<dyn Array>>> {
        self.inner.build_arrays()
    }
}

/// Experimental functionality that is not subject to semver compatibility
pub mod experimental {
    use crate::impls::arrow2::datatypes::Field;

    use super::display;

    /// Format the fields as a string
    ///
    /// The fields are displayed as Rust code that allows to build the fields in
    /// code. The following symbols of the `arrow2::datatypes `module are
    /// assumed to be in scope `DataType`, `Field`, `Metadata`.
    ///
    pub fn format_fields(fields: &[Field]) -> String {
        display::Fields(fields).to_string()
    }

    pub use super::schema::find_field_mut;
}