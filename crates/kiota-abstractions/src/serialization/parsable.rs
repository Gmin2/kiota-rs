use indexmap::IndexMap;
use std::any::Any;

use crate::KiotaError;
use super::parse_node::ParseNode;
use super::serialization_writer::SerializationWriter;

/// Type alias for field deserializer closures.
pub type FieldDeserializers =
    IndexMap<String, Box<dyn Fn(&dyn ParseNode) -> Result<(), KiotaError> + Send + Sync>>;

/// Factory function type for creating Parsable instances.
pub type ParsableFactory<T> = fn(parse_node: &dyn ParseNode) -> Result<T, KiotaError>;

/// The core trait that all generated model types implement.
pub trait Parsable: Send + Sync + std::fmt::Debug {
    fn get_field_deserializers(&self) -> FieldDeserializers;
    fn serialize(&self, writer: &mut dyn SerializationWriter) -> Result<(), KiotaError>;
}

/// For models that carry additional undeclared properties.
pub trait AdditionalDataHolder {
    fn additional_data(&self) -> &IndexMap<String, Box<dyn Any + Send + Sync>>;
    fn set_additional_data(&mut self, data: IndexMap<String, Box<dyn Any + Send + Sync>>);
}

/// Marker trait for composed type wrappers (oneOf/anyOf).
pub trait ComposedTypeWrapper {
    fn is_composed_type(&self) -> bool {
        true
    }
}
