use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime};
use uuid::Uuid;

use crate::iso_duration::IsoDuration;
use crate::KiotaError;
use super::parsable::{Parsable, ParsableFactory};

pub type ParsableAction = Box<dyn Fn(&dyn Parsable) + Send + Sync>;
pub type EnumFactory<T> = fn(&str) -> Option<T>;

/// Deserialization node in a parse tree.
pub trait ParseNode: Send + Sync {
    fn get_string_value(&self) -> Result<Option<String>, KiotaError>;
    fn get_bool_value(&self) -> Result<Option<bool>, KiotaError>;
    fn get_i8_value(&self) -> Result<Option<i8>, KiotaError>;
    fn get_u8_value(&self) -> Result<Option<u8>, KiotaError>;
    fn get_i32_value(&self) -> Result<Option<i32>, KiotaError>;
    fn get_i64_value(&self) -> Result<Option<i64>, KiotaError>;
    fn get_f32_value(&self) -> Result<Option<f32>, KiotaError>;
    fn get_f64_value(&self) -> Result<Option<f64>, KiotaError>;
    fn get_uuid_value(&self) -> Result<Option<Uuid>, KiotaError>;
    fn get_date_time_value(&self) -> Result<Option<DateTime<FixedOffset>>, KiotaError>;
    fn get_date_only_value(&self) -> Result<Option<NaiveDate>, KiotaError>;
    fn get_time_only_value(&self) -> Result<Option<NaiveTime>, KiotaError>;
    fn get_duration_value(&self) -> Result<Option<IsoDuration>, KiotaError>;
    fn get_byte_array_value(&self) -> Result<Option<Vec<u8>>, KiotaError>;

    fn get_child_node(&self, identifier: &str) -> Result<Option<Box<dyn ParseNode>>, KiotaError>;

    /// Returns child nodes for array iteration. Dyn-compatible alternative to get_collection_of_*.
    fn get_child_nodes(&self) -> Result<Vec<Box<dyn ParseNode>>, KiotaError>;

    /// Gets a collection of string values from an array node.
    fn get_collection_of_string_values(&self) -> Result<Vec<String>, KiotaError>;

    fn get_object_value<T: Parsable + Default>(
        &self,
        factory: ParsableFactory<T>,
    ) -> Result<Option<T>, KiotaError>
    where
        Self: Sized;

    fn get_collection_of_object_values<T: Parsable + Default>(
        &self,
        factory: ParsableFactory<T>,
    ) -> Result<Vec<T>, KiotaError>
    where
        Self: Sized;

    fn get_collection_of_primitive_values<T: FromParseNode>(
        &self,
    ) -> Result<Vec<T>, KiotaError>
    where
        Self: Sized;

    fn get_enum_value<T: Clone>(
        &self,
        factory: EnumFactory<T>,
    ) -> Result<Option<T>, KiotaError>
    where
        Self: Sized;

    fn get_collection_of_enum_values<T: Clone>(
        &self,
        factory: EnumFactory<T>,
    ) -> Result<Vec<T>, KiotaError>
    where
        Self: Sized;

    fn on_before_assign_field_values(&self) -> Option<&ParsableAction>;
    fn set_on_before_assign_field_values(&mut self, action: Option<ParsableAction>);
    fn on_after_assign_field_values(&self) -> Option<&ParsableAction>;
    fn set_on_after_assign_field_values(&mut self, action: Option<ParsableAction>);
}

/// Helper trait for extracting primitives from a ParseNode generically.
pub trait FromParseNode: Sized + Send {
    fn from_parse_node(node: &dyn ParseNode) -> Result<Option<Self>, KiotaError>;
}

impl FromParseNode for String {
    fn from_parse_node(node: &dyn ParseNode) -> Result<Option<Self>, KiotaError> {
        node.get_string_value()
    }
}

impl FromParseNode for bool {
    fn from_parse_node(node: &dyn ParseNode) -> Result<Option<Self>, KiotaError> {
        node.get_bool_value()
    }
}

impl FromParseNode for i32 {
    fn from_parse_node(node: &dyn ParseNode) -> Result<Option<Self>, KiotaError> {
        node.get_i32_value()
    }
}

impl FromParseNode for i64 {
    fn from_parse_node(node: &dyn ParseNode) -> Result<Option<Self>, KiotaError> {
        node.get_i64_value()
    }
}

impl FromParseNode for f32 {
    fn from_parse_node(node: &dyn ParseNode) -> Result<Option<Self>, KiotaError> {
        node.get_f32_value()
    }
}

impl FromParseNode for f64 {
    fn from_parse_node(node: &dyn ParseNode) -> Result<Option<Self>, KiotaError> {
        node.get_f64_value()
    }
}
