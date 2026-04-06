use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime};
use indexmap::IndexMap;
use std::any::Any;
use uuid::Uuid;

use crate::iso_duration::IsoDuration;
use crate::KiotaError;
use super::parsable::Parsable;

pub type ParsableWriter =
    Box<dyn Fn(&dyn Parsable, &mut dyn SerializationWriter) + Send + Sync>;

/// Serialization interface for writing model properties.
pub trait SerializationWriter: Send + Sync {
    fn write_string_value(&mut self, key: Option<&str>, value: &str) -> Result<(), KiotaError>;
    fn write_bool_value(&mut self, key: Option<&str>, value: bool) -> Result<(), KiotaError>;
    fn write_i8_value(&mut self, key: Option<&str>, value: i8) -> Result<(), KiotaError>;
    fn write_u8_value(&mut self, key: Option<&str>, value: u8) -> Result<(), KiotaError>;
    fn write_i32_value(&mut self, key: Option<&str>, value: i32) -> Result<(), KiotaError>;
    fn write_i64_value(&mut self, key: Option<&str>, value: i64) -> Result<(), KiotaError>;
    fn write_f32_value(&mut self, key: Option<&str>, value: f32) -> Result<(), KiotaError>;
    fn write_f64_value(&mut self, key: Option<&str>, value: f64) -> Result<(), KiotaError>;
    fn write_uuid_value(&mut self, key: Option<&str>, value: &Uuid) -> Result<(), KiotaError>;
    fn write_date_time_value(
        &mut self,
        key: Option<&str>,
        value: &DateTime<FixedOffset>,
    ) -> Result<(), KiotaError>;
    fn write_date_only_value(
        &mut self,
        key: Option<&str>,
        value: &NaiveDate,
    ) -> Result<(), KiotaError>;
    fn write_time_only_value(
        &mut self,
        key: Option<&str>,
        value: &NaiveTime,
    ) -> Result<(), KiotaError>;
    fn write_duration_value(
        &mut self,
        key: Option<&str>,
        value: &IsoDuration,
    ) -> Result<(), KiotaError>;
    fn write_byte_array_value(
        &mut self,
        key: Option<&str>,
        value: &[u8],
    ) -> Result<(), KiotaError>;
    fn write_null_value(&mut self, key: Option<&str>) -> Result<(), KiotaError>;

    fn write_object_value(
        &mut self,
        key: Option<&str>,
        value: &dyn Parsable,
        additional_values_to_merge: &[&dyn Parsable],
    ) -> Result<(), KiotaError>;

    fn write_collection_of_object_values(
        &mut self,
        key: Option<&str>,
        values: &[&dyn Parsable],
    ) -> Result<(), KiotaError>;

    fn write_collection_of_string_values(
        &mut self,
        key: Option<&str>,
        values: &[String],
    ) -> Result<(), KiotaError>;

    fn write_collection_of_bool_values(
        &mut self,
        key: Option<&str>,
        values: &[bool],
    ) -> Result<(), KiotaError>;

    fn write_collection_of_i32_values(
        &mut self,
        key: Option<&str>,
        values: &[i32],
    ) -> Result<(), KiotaError>;

    fn write_collection_of_i64_values(
        &mut self,
        key: Option<&str>,
        values: &[i64],
    ) -> Result<(), KiotaError>;

    fn write_collection_of_f32_values(
        &mut self,
        key: Option<&str>,
        values: &[f32],
    ) -> Result<(), KiotaError>;

    fn write_collection_of_f64_values(
        &mut self,
        key: Option<&str>,
        values: &[f64],
    ) -> Result<(), KiotaError>;

    fn write_enum_value<T: std::fmt::Display>(
        &mut self,
        key: Option<&str>,
        value: &T,
    ) -> Result<(), KiotaError>
    where
        Self: Sized;

    fn write_collection_of_enum_values<T: std::fmt::Display>(
        &mut self,
        key: Option<&str>,
        values: &[T],
    ) -> Result<(), KiotaError>
    where
        Self: Sized;

    fn write_additional_data(
        &mut self,
        data: &IndexMap<String, Box<dyn Any + Send + Sync>>,
    ) -> Result<(), KiotaError>;

    fn get_serialized_content(&mut self) -> Result<Vec<u8>, KiotaError>;

    fn on_before_serialization(&self) -> Option<&super::parse_node::ParsableAction>;
    fn set_on_before_serialization(
        &mut self,
        action: Option<super::parse_node::ParsableAction>,
    );
    fn on_after_serialization(&self) -> Option<&super::parse_node::ParsableAction>;
    fn set_on_after_serialization(
        &mut self,
        action: Option<super::parse_node::ParsableAction>,
    );
    fn on_start_serialization(&self) -> Option<&ParsableWriter>;
    fn set_on_start_serialization(&mut self, action: Option<ParsableWriter>);
}
