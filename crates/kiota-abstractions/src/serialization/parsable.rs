use indexmap::IndexMap;
use std::any::Any;

use crate::KiotaError;
use super::parse_node::ParseNode;
use super::serialization_writer::SerializationWriter;

/// Factory function type for creating Parsable instances.
pub type ParsableFactory<T> = fn(parse_node: &dyn ParseNode) -> Result<T, KiotaError>;

/// The core trait that all generated model types implement.
pub trait Parsable: Send + Sync + std::fmt::Debug {
    /// Returns the list of field names this model knows how to deserialize.
    fn field_names(&self) -> Vec<&'static str>;

    /// Assigns a field value from a parse node during deserialization.
    /// The parse node is positioned at the field's value.
    fn assign_field(&mut self, field: &str, node: &dyn ParseNode) -> Result<(), KiotaError>;

    /// Serializes the model to the writer.
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
