use std::collections::HashMap;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime};
use uuid::Uuid;

use kiota_abstractions::iso_duration::IsoDuration;
use kiota_abstractions::serialization::{Parsable, ParsableAction, ParsableWriter, SerializationWriter};
use kiota_abstractions::KiotaError;

pub struct MultipartSerializationWriter {
    buf: Vec<u8>,
    on_before: Option<ParsableAction>,
    on_after: Option<ParsableAction>,
    on_start: Option<ParsableWriter>,
}

impl MultipartSerializationWriter {
    pub fn new() -> Self {
        Self { buf: Vec::new(), on_before: None, on_after: None, on_start: None }
    }

    fn not_supported(method: &str) -> KiotaError {
        KiotaError::Serialization(format!("multipart writer: {method} not supported, use write_byte_array_value"))
    }
}

impl Default for MultipartSerializationWriter {
    fn default() -> Self { Self::new() }
}

impl SerializationWriter for MultipartSerializationWriter {
    fn write_string_value(&mut self, _: Option<&str>, _: &str) -> Result<(), KiotaError> { Err(Self::not_supported("write_string_value")) }
    fn write_bool_value(&mut self, _: Option<&str>, _: bool) -> Result<(), KiotaError> { Err(Self::not_supported("write_bool_value")) }
    fn write_i8_value(&mut self, _: Option<&str>, _: i8) -> Result<(), KiotaError> { Err(Self::not_supported("write_i8_value")) }
    fn write_u8_value(&mut self, _: Option<&str>, _: u8) -> Result<(), KiotaError> { Err(Self::not_supported("write_u8_value")) }
    fn write_i32_value(&mut self, _: Option<&str>, _: i32) -> Result<(), KiotaError> { Err(Self::not_supported("write_i32_value")) }
    fn write_i64_value(&mut self, _: Option<&str>, _: i64) -> Result<(), KiotaError> { Err(Self::not_supported("write_i64_value")) }
    fn write_f32_value(&mut self, _: Option<&str>, _: f32) -> Result<(), KiotaError> { Err(Self::not_supported("write_f32_value")) }
    fn write_f64_value(&mut self, _: Option<&str>, _: f64) -> Result<(), KiotaError> { Err(Self::not_supported("write_f64_value")) }
    fn write_uuid_value(&mut self, _: Option<&str>, _: &Uuid) -> Result<(), KiotaError> { Err(Self::not_supported("write_uuid_value")) }
    fn write_date_time_value(&mut self, _: Option<&str>, _: &DateTime<FixedOffset>) -> Result<(), KiotaError> { Err(Self::not_supported("write_date_time_value")) }
    fn write_date_only_value(&mut self, _: Option<&str>, _: &NaiveDate) -> Result<(), KiotaError> { Err(Self::not_supported("write_date_only_value")) }
    fn write_time_only_value(&mut self, _: Option<&str>, _: &NaiveTime) -> Result<(), KiotaError> { Err(Self::not_supported("write_time_only_value")) }
    fn write_duration_value(&mut self, _: Option<&str>, _: &IsoDuration) -> Result<(), KiotaError> { Err(Self::not_supported("write_duration_value")) }
    fn write_null_value(&mut self, _: Option<&str>) -> Result<(), KiotaError> { Ok(()) }
    fn write_byte_array_value(&mut self, _: Option<&str>, value: &[u8]) -> Result<(), KiotaError> {
        self.buf.extend_from_slice(value);
        Ok(())
    }
    fn write_object_value(&mut self, _: Option<&str>, value: &dyn Parsable, _: &[&dyn Parsable]) -> Result<(), KiotaError> {
        value.serialize(self)
    }
    fn write_collection_of_object_values(&mut self, _: Option<&str>, _: &[&dyn Parsable]) -> Result<(), KiotaError> { Err(Self::not_supported("collections")) }
    fn write_collection_of_string_values(&mut self, _: Option<&str>, _: &[String]) -> Result<(), KiotaError> { Err(Self::not_supported("collections")) }
    fn write_collection_of_bool_values(&mut self, _: Option<&str>, _: &[bool]) -> Result<(), KiotaError> { Err(Self::not_supported("collections")) }
    fn write_collection_of_i32_values(&mut self, _: Option<&str>, _: &[i32]) -> Result<(), KiotaError> { Err(Self::not_supported("collections")) }
    fn write_collection_of_i64_values(&mut self, _: Option<&str>, _: &[i64]) -> Result<(), KiotaError> { Err(Self::not_supported("collections")) }
    fn write_collection_of_f32_values(&mut self, _: Option<&str>, _: &[f32]) -> Result<(), KiotaError> { Err(Self::not_supported("collections")) }
    fn write_collection_of_f64_values(&mut self, _: Option<&str>, _: &[f64]) -> Result<(), KiotaError> { Err(Self::not_supported("collections")) }
    fn write_enum_value<T: std::fmt::Display>(&mut self, _: Option<&str>, _: &T) -> Result<(), KiotaError> where Self: Sized { Err(Self::not_supported("write_enum_value")) }
    fn write_collection_of_enum_values<T: std::fmt::Display>(&mut self, _: Option<&str>, _: &[T]) -> Result<(), KiotaError> where Self: Sized { Err(Self::not_supported("collections")) }
    fn write_additional_data(&mut self, _: &HashMap<String, serde_json::Value>) -> Result<(), KiotaError> { Ok(()) }
    fn get_serialized_content(&mut self) -> Result<Vec<u8>, KiotaError> {
        Ok(std::mem::take(&mut self.buf))
    }
    fn on_before_serialization(&self) -> Option<&ParsableAction> { self.on_before.as_ref() }
    fn set_on_before_serialization(&mut self, action: Option<ParsableAction>) { self.on_before = action; }
    fn on_after_serialization(&self) -> Option<&ParsableAction> { self.on_after.as_ref() }
    fn set_on_after_serialization(&mut self, action: Option<ParsableAction>) { self.on_after = action; }
    fn on_start_serialization(&self) -> Option<&ParsableWriter> { self.on_start.as_ref() }
    fn set_on_start_serialization(&mut self, action: Option<ParsableWriter>) { self.on_start = action; }
}
