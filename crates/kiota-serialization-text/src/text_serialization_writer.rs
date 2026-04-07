use std::collections::HashMap;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime};
use uuid::Uuid;

use kiota_abstractions::iso_duration::IsoDuration;
use kiota_abstractions::serialization::{Parsable, ParsableAction, ParsableWriter, SerializationWriter};
use kiota_abstractions::KiotaError;

pub struct TextSerializationWriter {
    value: Option<String>,
    on_before: Option<ParsableAction>,
    on_after: Option<ParsableAction>,
    on_start: Option<ParsableWriter>,
}

impl TextSerializationWriter {
    pub fn new() -> Self {
        Self { value: None, on_before: None, on_after: None, on_start: None }
    }

    fn set(&mut self, key: Option<&str>, val: String) -> Result<(), KiotaError> {
        if key.is_some() {
            return Err(KiotaError::Serialization("text writer does not support keyed values".to_string()));
        }
        if self.value.is_some() {
            return Err(KiotaError::Serialization("text writer only supports a single value".to_string()));
        }
        self.value = Some(val);
        Ok(())
    }

    fn no_structured() -> KiotaError {
        KiotaError::Serialization("text writer does not support structured data".to_string())
    }
}

impl Default for TextSerializationWriter {
    fn default() -> Self { Self::new() }
}

impl SerializationWriter for TextSerializationWriter {
    fn write_string_value(&mut self, key: Option<&str>, value: &str) -> Result<(), KiotaError> {
        self.set(key, value.to_string())
    }
    fn write_bool_value(&mut self, key: Option<&str>, value: bool) -> Result<(), KiotaError> {
        self.set(key, value.to_string())
    }
    fn write_i8_value(&mut self, key: Option<&str>, value: i8) -> Result<(), KiotaError> {
        self.set(key, value.to_string())
    }
    fn write_u8_value(&mut self, key: Option<&str>, value: u8) -> Result<(), KiotaError> {
        self.set(key, value.to_string())
    }
    fn write_i32_value(&mut self, key: Option<&str>, value: i32) -> Result<(), KiotaError> {
        self.set(key, value.to_string())
    }
    fn write_i64_value(&mut self, key: Option<&str>, value: i64) -> Result<(), KiotaError> {
        self.set(key, value.to_string())
    }
    fn write_f32_value(&mut self, key: Option<&str>, value: f32) -> Result<(), KiotaError> {
        self.set(key, value.to_string())
    }
    fn write_f64_value(&mut self, key: Option<&str>, value: f64) -> Result<(), KiotaError> {
        self.set(key, value.to_string())
    }
    fn write_uuid_value(&mut self, key: Option<&str>, value: &Uuid) -> Result<(), KiotaError> {
        self.set(key, value.to_string())
    }
    fn write_date_time_value(&mut self, key: Option<&str>, value: &DateTime<FixedOffset>) -> Result<(), KiotaError> {
        self.set(key, value.to_rfc3339())
    }
    fn write_date_only_value(&mut self, key: Option<&str>, value: &NaiveDate) -> Result<(), KiotaError> {
        self.set(key, value.format("%Y-%m-%d").to_string())
    }
    fn write_time_only_value(&mut self, key: Option<&str>, value: &NaiveTime) -> Result<(), KiotaError> {
        self.set(key, value.format("%H:%M:%S").to_string())
    }
    fn write_duration_value(&mut self, key: Option<&str>, value: &IsoDuration) -> Result<(), KiotaError> {
        self.set(key, value.to_string())
    }
    fn write_byte_array_value(&mut self, _: Option<&str>, _: &[u8]) -> Result<(), KiotaError> {
        Err(Self::no_structured())
    }
    fn write_null_value(&mut self, key: Option<&str>) -> Result<(), KiotaError> {
        self.set(key, "null".to_string())
    }
    fn write_object_value(&mut self, _: Option<&str>, _: &dyn Parsable, _: &[&dyn Parsable]) -> Result<(), KiotaError> {
        Err(Self::no_structured())
    }
    fn write_collection_of_object_values(&mut self, _: Option<&str>, _: &[&dyn Parsable]) -> Result<(), KiotaError> {
        Err(Self::no_structured())
    }
    fn write_collection_of_string_values(&mut self, _: Option<&str>, _: &[String]) -> Result<(), KiotaError> {
        Err(Self::no_structured())
    }
    fn write_collection_of_bool_values(&mut self, _: Option<&str>, _: &[bool]) -> Result<(), KiotaError> {
        Err(Self::no_structured())
    }
    fn write_collection_of_i32_values(&mut self, _: Option<&str>, _: &[i32]) -> Result<(), KiotaError> {
        Err(Self::no_structured())
    }
    fn write_collection_of_i64_values(&mut self, _: Option<&str>, _: &[i64]) -> Result<(), KiotaError> {
        Err(Self::no_structured())
    }
    fn write_collection_of_f32_values(&mut self, _: Option<&str>, _: &[f32]) -> Result<(), KiotaError> {
        Err(Self::no_structured())
    }
    fn write_collection_of_f64_values(&mut self, _: Option<&str>, _: &[f64]) -> Result<(), KiotaError> {
        Err(Self::no_structured())
    }
    fn write_enum_value<T: std::fmt::Display>(&mut self, key: Option<&str>, value: &T) -> Result<(), KiotaError> where Self: Sized {
        self.set(key, value.to_string())
    }
    fn write_collection_of_enum_values<T: std::fmt::Display>(&mut self, _: Option<&str>, _: &[T]) -> Result<(), KiotaError> where Self: Sized {
        Err(Self::no_structured())
    }
    fn write_additional_data(&mut self, _: &HashMap<String, serde_json::Value>) -> Result<(), KiotaError> {
        Ok(()) // no-op for text
    }
    fn get_serialized_content(&mut self) -> Result<Vec<u8>, KiotaError> {
        Ok(self.value.take().unwrap_or_default().into_bytes())
    }
    fn on_before_serialization(&self) -> Option<&ParsableAction> { self.on_before.as_ref() }
    fn set_on_before_serialization(&mut self, action: Option<ParsableAction>) { self.on_before = action; }
    fn on_after_serialization(&self) -> Option<&ParsableAction> { self.on_after.as_ref() }
    fn set_on_after_serialization(&mut self, action: Option<ParsableAction>) { self.on_after = action; }
    fn on_start_serialization(&self) -> Option<&ParsableWriter> { self.on_start.as_ref() }
    fn set_on_start_serialization(&mut self, action: Option<ParsableWriter>) { self.on_start = action; }
}
