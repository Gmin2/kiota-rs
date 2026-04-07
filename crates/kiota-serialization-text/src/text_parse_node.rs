use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime};
use uuid::Uuid;

use kiota_abstractions::iso_duration::IsoDuration;
use kiota_abstractions::serialization::{
    EnumFactory, FromParseNode, ParsableAction, Parsable, ParsableFactory, ParseNode,
};
use kiota_abstractions::KiotaError;

pub struct TextParseNode {
    text: Option<String>,
    on_before: Option<ParsableAction>,
    on_after: Option<ParsableAction>,
}

impl TextParseNode {
    pub fn new(text: String) -> Self {
        Self {
            text: if text.is_empty() { None } else { Some(text) },
            on_before: None,
            on_after: None,
        }
    }

    fn no_structured_data() -> KiotaError {
        KiotaError::Deserialization("text parse node does not support structured data".to_string())
    }
}

impl ParseNode for TextParseNode {
    fn get_string_value(&self) -> Result<Option<String>, KiotaError> {
        Ok(self.text.clone())
    }
    fn get_bool_value(&self) -> Result<Option<bool>, KiotaError> {
        Ok(self.text.as_deref().and_then(|s| s.parse().ok()))
    }
    fn get_i8_value(&self) -> Result<Option<i8>, KiotaError> {
        Ok(self.text.as_deref().and_then(|s| s.parse().ok()))
    }
    fn get_u8_value(&self) -> Result<Option<u8>, KiotaError> {
        Ok(self.text.as_deref().and_then(|s| s.parse().ok()))
    }
    fn get_i32_value(&self) -> Result<Option<i32>, KiotaError> {
        Ok(self.text.as_deref().and_then(|s| s.parse().ok()))
    }
    fn get_i64_value(&self) -> Result<Option<i64>, KiotaError> {
        Ok(self.text.as_deref().and_then(|s| s.parse().ok()))
    }
    fn get_f32_value(&self) -> Result<Option<f32>, KiotaError> {
        Ok(self.text.as_deref().and_then(|s| s.parse().ok()))
    }
    fn get_f64_value(&self) -> Result<Option<f64>, KiotaError> {
        Ok(self.text.as_deref().and_then(|s| s.parse().ok()))
    }
    fn get_uuid_value(&self) -> Result<Option<Uuid>, KiotaError> {
        Ok(self.text.as_deref().and_then(|s| Uuid::parse_str(s).ok()))
    }
    fn get_date_time_value(&self) -> Result<Option<DateTime<FixedOffset>>, KiotaError> {
        Ok(self.text.as_deref().and_then(|s| DateTime::parse_from_rfc3339(s).ok()))
    }
    fn get_date_only_value(&self) -> Result<Option<NaiveDate>, KiotaError> {
        Ok(self.text.as_deref().and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()))
    }
    fn get_time_only_value(&self) -> Result<Option<NaiveTime>, KiotaError> {
        Ok(self.text.as_deref().and_then(|s| NaiveTime::parse_from_str(s, "%H:%M:%S").ok()))
    }
    fn get_duration_value(&self) -> Result<Option<IsoDuration>, KiotaError> {
        Ok(self.text.as_deref().and_then(|s| IsoDuration::parse(s).ok()))
    }
    fn get_byte_array_value(&self) -> Result<Option<Vec<u8>>, KiotaError> {
        Ok(self.text.as_ref().map(|s| s.as_bytes().to_vec()))
    }
    fn get_child_node(&self, _: &str) -> Result<Option<Box<dyn ParseNode>>, KiotaError> {
        Err(Self::no_structured_data())
    }
    fn get_child_nodes(&self) -> Result<Vec<Box<dyn ParseNode>>, KiotaError> {
        Err(Self::no_structured_data())
    }
    fn get_collection_of_string_values(&self) -> Result<Vec<String>, KiotaError> {
        Err(Self::no_structured_data())
    }
    fn get_object_value<T: Parsable + Default>(&self, _: ParsableFactory<T>) -> Result<Option<T>, KiotaError> where Self: Sized {
        Err(Self::no_structured_data())
    }
    fn get_collection_of_object_values<T: Parsable + Default>(&self, _: ParsableFactory<T>) -> Result<Vec<T>, KiotaError> where Self: Sized {
        Err(Self::no_structured_data())
    }
    fn get_collection_of_primitive_values<T: FromParseNode>(&self) -> Result<Vec<T>, KiotaError> where Self: Sized {
        Err(Self::no_structured_data())
    }
    fn get_enum_value<T: Clone>(&self, factory: EnumFactory<T>) -> Result<Option<T>, KiotaError> where Self: Sized {
        Ok(self.text.as_deref().and_then(factory))
    }
    fn get_collection_of_enum_values<T: Clone>(&self, _: EnumFactory<T>) -> Result<Vec<T>, KiotaError> where Self: Sized {
        Err(Self::no_structured_data())
    }
    fn on_before_assign_field_values(&self) -> Option<&ParsableAction> { self.on_before.as_ref() }
    fn set_on_before_assign_field_values(&mut self, action: Option<ParsableAction>) { self.on_before = action; }
    fn on_after_assign_field_values(&self) -> Option<&ParsableAction> { self.on_after.as_ref() }
    fn set_on_after_assign_field_values(&mut self, action: Option<ParsableAction>) { self.on_after = action; }
}
