use std::any::Any;

use crate::KiotaError;
use super::parse_node::ParseNode;
use super::serialization_writer::SerializationWriter;

/// Factory function type for creating Parsable instances.
pub type ParsableFactory<T> = fn(parse_node: &dyn ParseNode) -> Result<T, KiotaError>;

/// The core trait that all generated model types implement.
pub trait Parsable: Send + Sync + std::fmt::Debug + Any {
    fn field_names(&self) -> Vec<&'static str>;
    fn assign_field(&mut self, field: &str, node: &dyn ParseNode) -> Result<(), KiotaError>;
    fn serialize(&self, writer: &mut dyn SerializationWriter) -> Result<(), KiotaError>;

    /// For downcasting `Box<dyn Parsable>` back to the concrete type.
    fn as_any(self: Box<Self>) -> Box<dyn Any>;
}

/// Marker trait for composed type wrappers (oneOf/anyOf).
pub trait ComposedTypeWrapper {
    fn is_composed_type(&self) -> bool {
        true
    }
}
