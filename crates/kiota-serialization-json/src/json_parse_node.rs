use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime};
use serde_json::Value;
use uuid::Uuid;

use kiota_abstractions::iso_duration::IsoDuration;
use kiota_abstractions::serialization::{
    EnumFactory, FromParseNode, ParsableAction, Parsable, ParsableFactory, ParseNode,
};
use kiota_abstractions::KiotaError;

pub struct JsonParseNode {
    value: Value,
    on_before: Option<ParsableAction>,
    on_after: Option<ParsableAction>,
}

impl JsonParseNode {
    pub fn new(value: Value) -> Self {
        Self {
            value,
            on_before: None,
            on_after: None,
        }
    }

    fn err(msg: impl Into<String>) -> KiotaError {
        KiotaError::Deserialization(msg.into())
    }
}

impl ParseNode for JsonParseNode {
    fn get_string_value(&self) -> Result<Option<String>, KiotaError> {
        match &self.value {
            Value::String(s) => Ok(Some(s.clone())),
            Value::Null => Ok(None),
            _ => Err(Self::err("expected string")),
        }
    }

    fn get_bool_value(&self) -> Result<Option<bool>, KiotaError> {
        match &self.value {
            Value::Bool(b) => Ok(Some(*b)),
            Value::Null => Ok(None),
            _ => Err(Self::err("expected bool")),
        }
    }

    fn get_i8_value(&self) -> Result<Option<i8>, KiotaError> {
        match &self.value {
            Value::Number(n) => n
                .as_i64()
                .and_then(|v| i8::try_from(v).ok())
                .map(|v| Ok(Some(v)))
                .unwrap_or_else(|| Err(Self::err("number out of i8 range"))),
            Value::Null => Ok(None),
            _ => Err(Self::err("expected number")),
        }
    }

    fn get_u8_value(&self) -> Result<Option<u8>, KiotaError> {
        match &self.value {
            Value::Number(n) => n
                .as_u64()
                .and_then(|v| u8::try_from(v).ok())
                .map(|v| Ok(Some(v)))
                .unwrap_or_else(|| Err(Self::err("number out of u8 range"))),
            Value::Null => Ok(None),
            _ => Err(Self::err("expected number")),
        }
    }

    fn get_i32_value(&self) -> Result<Option<i32>, KiotaError> {
        match &self.value {
            Value::Number(n) => n
                .as_i64()
                .and_then(|v| i32::try_from(v).ok())
                .map(|v| Ok(Some(v)))
                .unwrap_or_else(|| Err(Self::err("number out of i32 range"))),
            Value::Null => Ok(None),
            _ => Err(Self::err("expected number")),
        }
    }

    fn get_i64_value(&self) -> Result<Option<i64>, KiotaError> {
        match &self.value {
            Value::Number(n) => n
                .as_i64()
                .map(|v| Ok(Some(v)))
                .unwrap_or_else(|| Err(Self::err("expected integer"))),
            Value::Null => Ok(None),
            _ => Err(Self::err("expected number")),
        }
    }

    fn get_f32_value(&self) -> Result<Option<f32>, KiotaError> {
        match &self.value {
            Value::Number(n) => n
                .as_f64()
                .map(|v| Ok(Some(v as f32)))
                .unwrap_or_else(|| Err(Self::err("expected number"))),
            Value::Null => Ok(None),
            _ => Err(Self::err("expected number")),
        }
    }

    fn get_f64_value(&self) -> Result<Option<f64>, KiotaError> {
        match &self.value {
            Value::Number(n) => n
                .as_f64()
                .map(|v| Ok(Some(v)))
                .unwrap_or_else(|| Err(Self::err("expected number"))),
            Value::Null => Ok(None),
            _ => Err(Self::err("expected number")),
        }
    }

    fn get_uuid_value(&self) -> Result<Option<Uuid>, KiotaError> {
        match &self.value {
            Value::String(s) => {
                let uuid = Uuid::parse_str(s)
                    .map_err(|e| Self::err(format!("invalid UUID: {e}")))?;
                Ok(Some(uuid))
            }
            Value::Null => Ok(None),
            _ => Err(Self::err("expected string for UUID")),
        }
    }

    fn get_date_time_value(&self) -> Result<Option<DateTime<FixedOffset>>, KiotaError> {
        match &self.value {
            Value::String(s) => {
                let dt = DateTime::parse_from_rfc3339(s)
                    .map_err(|e| Self::err(format!("invalid datetime: {e}")))?;
                Ok(Some(dt))
            }
            Value::Null => Ok(None),
            _ => Err(Self::err("expected string for datetime")),
        }
    }

    fn get_date_only_value(&self) -> Result<Option<NaiveDate>, KiotaError> {
        match &self.value {
            Value::String(s) => {
                let d = NaiveDate::parse_from_str(s, "%Y-%m-%d")
                    .map_err(|e| Self::err(format!("invalid date: {e}")))?;
                Ok(Some(d))
            }
            Value::Null => Ok(None),
            _ => Err(Self::err("expected string for date")),
        }
    }

    fn get_time_only_value(&self) -> Result<Option<NaiveTime>, KiotaError> {
        match &self.value {
            Value::String(s) => {
                let t = NaiveTime::parse_from_str(s, "%H:%M:%S")
                    .map_err(|e| Self::err(format!("invalid time: {e}")))?;
                Ok(Some(t))
            }
            Value::Null => Ok(None),
            _ => Err(Self::err("expected string for time")),
        }
    }

    fn get_duration_value(&self) -> Result<Option<IsoDuration>, KiotaError> {
        match &self.value {
            Value::String(s) => Ok(Some(IsoDuration::parse(s)?)),
            Value::Null => Ok(None),
            _ => Err(Self::err("expected string for duration")),
        }
    }

    fn get_byte_array_value(&self) -> Result<Option<Vec<u8>>, KiotaError> {
        match &self.value {
            Value::Array(arr) => {
                let mut bytes = Vec::with_capacity(arr.len());
                for v in arr {
                    let b = v
                        .as_u64()
                        .and_then(|n| u8::try_from(n).ok())
                        .ok_or_else(|| Self::err("expected byte value in array"))?;
                    bytes.push(b);
                }
                Ok(Some(bytes))
            }
            Value::Null => Ok(None),
            _ => Err(Self::err("expected array for byte array")),
        }
    }

    fn get_child_node(&self, identifier: &str) -> Result<Option<Box<dyn ParseNode>>, KiotaError> {
        match &self.value {
            Value::Object(map) => match map.get(identifier) {
                Some(v) => Ok(Some(Box::new(JsonParseNode::new(v.clone())))),
                None => Ok(None),
            },
            _ => Ok(None),
        }
    }

    fn get_object_value<T: Parsable + Default>(
        &self,
        factory: ParsableFactory<T>,
    ) -> Result<Option<T>, KiotaError> {
        if self.value.is_null() {
            return Ok(None);
        }

        let mut item = factory(self)?;

        if let Some(ref cb) = self.on_before {
            cb(&item);
        }

        if let Value::Object(map) = &self.value {
            for (key, val) in map {
                let child = JsonParseNode::new(val.clone());
                item.assign_field(key, &child)?;
            }
        }

        if let Some(ref cb) = self.on_after {
            cb(&item);
        }

        Ok(Some(item))
    }

    fn get_collection_of_object_values<T: Parsable + Default>(
        &self,
        factory: ParsableFactory<T>,
    ) -> Result<Vec<T>, KiotaError> {
        match &self.value {
            Value::Array(arr) => {
                let mut result = Vec::with_capacity(arr.len());
                for val in arr {
                    let node = JsonParseNode {
                        value: val.clone(),
                        on_before: None,
                        on_after: None,
                    };
                    if let Some(item) = node.get_object_value(factory)? {
                        result.push(item);
                    }
                }
                Ok(result)
            }
            Value::Null => Ok(Vec::new()),
            _ => Err(Self::err("expected array")),
        }
    }

    fn get_collection_of_primitive_values<T: FromParseNode>(&self) -> Result<Vec<T>, KiotaError> {
        match &self.value {
            Value::Array(arr) => {
                let mut result = Vec::with_capacity(arr.len());
                for val in arr {
                    let node = JsonParseNode::new(val.clone());
                    if let Some(item) = T::from_parse_node(&node)? {
                        result.push(item);
                    }
                }
                Ok(result)
            }
            Value::Null => Ok(Vec::new()),
            _ => Err(Self::err("expected array")),
        }
    }

    fn get_enum_value<T: Clone>(&self, factory: EnumFactory<T>) -> Result<Option<T>, KiotaError> {
        let s = self.get_string_value()?;
        match s {
            Some(s) => Ok(factory(&s)),
            None => Ok(None),
        }
    }

    fn get_collection_of_enum_values<T: Clone>(
        &self,
        factory: EnumFactory<T>,
    ) -> Result<Vec<T>, KiotaError> {
        match &self.value {
            Value::Array(arr) => {
                let mut result = Vec::with_capacity(arr.len());
                for val in arr {
                    let node = JsonParseNode::new(val.clone());
                    if let Some(item) = node.get_enum_value(factory)? {
                        result.push(item);
                    }
                }
                Ok(result)
            }
            Value::Null => Ok(Vec::new()),
            _ => Err(Self::err("expected array")),
        }
    }

    fn on_before_assign_field_values(&self) -> Option<&ParsableAction> {
        self.on_before.as_ref()
    }

    fn set_on_before_assign_field_values(&mut self, action: Option<ParsableAction>) {
        self.on_before = action;
    }

    fn on_after_assign_field_values(&self) -> Option<&ParsableAction> {
        self.on_after.as_ref()
    }

    fn set_on_after_assign_field_values(&mut self, action: Option<ParsableAction>) {
        self.on_after = action;
    }
}
