use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime};
use indexmap::IndexMap;
use serde_json::{Map, Value};
use std::any::Any;
use uuid::Uuid;

use kiota_abstractions::iso_duration::IsoDuration;
use kiota_abstractions::serialization::{
    Parsable, ParsableAction, ParsableWriter, SerializationWriter,
};
use kiota_abstractions::KiotaError;

pub struct JsonSerializationWriter {
    current: Map<String, Value>,
    stack: Vec<Map<String, Value>>,
    root_value: Option<Value>,
    on_before: Option<ParsableAction>,
    on_after: Option<ParsableAction>,
    on_start: Option<ParsableWriter>,
}

impl JsonSerializationWriter {
    pub fn new() -> Self {
        Self {
            current: Map::new(),
            stack: Vec::new(),
            root_value: None,
            on_before: None,
            on_after: None,
            on_start: None,
        }
    }

    fn write_value(&mut self, key: Option<&str>, value: Value) -> Result<(), KiotaError> {
        match key {
            Some(k) => {
                self.current.insert(k.to_string(), value);
            }
            None => {
                self.root_value = Some(value);
            }
        }
        Ok(())
    }

    fn err(msg: impl Into<String>) -> KiotaError {
        KiotaError::Serialization(msg.into())
    }
}

impl Default for JsonSerializationWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl SerializationWriter for JsonSerializationWriter {
    fn write_string_value(&mut self, key: Option<&str>, value: &str) -> Result<(), KiotaError> {
        self.write_value(key, Value::String(value.to_string()))
    }

    fn write_bool_value(&mut self, key: Option<&str>, value: bool) -> Result<(), KiotaError> {
        self.write_value(key, Value::Bool(value))
    }

    fn write_i8_value(&mut self, key: Option<&str>, value: i8) -> Result<(), KiotaError> {
        self.write_value(key, Value::from(value))
    }

    fn write_u8_value(&mut self, key: Option<&str>, value: u8) -> Result<(), KiotaError> {
        self.write_value(key, Value::from(value))
    }

    fn write_i32_value(&mut self, key: Option<&str>, value: i32) -> Result<(), KiotaError> {
        self.write_value(key, Value::from(value))
    }

    fn write_i64_value(&mut self, key: Option<&str>, value: i64) -> Result<(), KiotaError> {
        self.write_value(key, Value::from(value))
    }

    fn write_f32_value(&mut self, key: Option<&str>, value: f32) -> Result<(), KiotaError> {
        self.write_value(
            key,
            serde_json::Number::from_f64(value as f64)
                .map(Value::Number)
                .ok_or_else(|| Self::err("invalid f32 value (NaN/Inf)"))?,
        )
    }

    fn write_f64_value(&mut self, key: Option<&str>, value: f64) -> Result<(), KiotaError> {
        self.write_value(
            key,
            serde_json::Number::from_f64(value)
                .map(Value::Number)
                .ok_or_else(|| Self::err("invalid f64 value (NaN/Inf)"))?,
        )
    }

    fn write_uuid_value(&mut self, key: Option<&str>, value: &Uuid) -> Result<(), KiotaError> {
        self.write_value(key, Value::String(value.to_string()))
    }

    fn write_date_time_value(
        &mut self,
        key: Option<&str>,
        value: &DateTime<FixedOffset>,
    ) -> Result<(), KiotaError> {
        self.write_value(key, Value::String(value.to_rfc3339()))
    }

    fn write_date_only_value(
        &mut self,
        key: Option<&str>,
        value: &NaiveDate,
    ) -> Result<(), KiotaError> {
        self.write_value(key, Value::String(value.format("%Y-%m-%d").to_string()))
    }

    fn write_time_only_value(
        &mut self,
        key: Option<&str>,
        value: &NaiveTime,
    ) -> Result<(), KiotaError> {
        self.write_value(key, Value::String(value.format("%H:%M:%S").to_string()))
    }

    fn write_duration_value(
        &mut self,
        key: Option<&str>,
        value: &IsoDuration,
    ) -> Result<(), KiotaError> {
        self.write_value(key, Value::String(value.to_string()))
    }

    fn write_byte_array_value(
        &mut self,
        key: Option<&str>,
        value: &[u8],
    ) -> Result<(), KiotaError> {
        let arr: Vec<Value> = value.iter().map(|&b| Value::from(b)).collect();
        self.write_value(key, Value::Array(arr))
    }

    fn write_null_value(&mut self, key: Option<&str>) -> Result<(), KiotaError> {
        self.write_value(key, Value::Null)
    }

    fn write_object_value(
        &mut self,
        key: Option<&str>,
        value: &dyn Parsable,
        additional_values_to_merge: &[&dyn Parsable],
    ) -> Result<(), KiotaError> {
        if let Some(ref cb) = self.on_before {
            cb(value);
        }
        if let Some(cb) = self.on_start.take() {
            cb(value, self);
            self.on_start = Some(cb);
        }

        let parent = std::mem::replace(&mut self.current, Map::new());
        self.stack.push(parent);

        value.serialize(self)?;
        for additional in additional_values_to_merge {
            additional.serialize(self)?;
        }

        let nested = std::mem::replace(
            &mut self.current,
            self.stack.pop().unwrap_or_default(),
        );

        self.write_value(key, Value::Object(nested))?;

        if let Some(ref cb) = self.on_after {
            cb(value);
        }
        Ok(())
    }

    fn write_collection_of_object_values(
        &mut self,
        key: Option<&str>,
        values: &[&dyn Parsable],
    ) -> Result<(), KiotaError> {
        let mut arr = Vec::with_capacity(values.len());
        for v in values {
            let parent = std::mem::replace(&mut self.current, Map::new());
            self.stack.push(parent);

            if let Some(ref cb) = self.on_before {
                cb(*v);
            }
            if let Some(cb) = self.on_start.take() {
                cb(*v, self);
                self.on_start = Some(cb);
            }

            v.serialize(self)?;

            if let Some(ref cb) = self.on_after {
                cb(*v);
            }

            let nested = std::mem::replace(
                &mut self.current,
                self.stack.pop().unwrap_or_default(),
            );
            arr.push(Value::Object(nested));
        }
        self.write_value(key, Value::Array(arr))
    }

    fn write_collection_of_string_values(
        &mut self,
        key: Option<&str>,
        values: &[String],
    ) -> Result<(), KiotaError> {
        let arr: Vec<Value> = values.iter().map(|s| Value::String(s.clone())).collect();
        self.write_value(key, Value::Array(arr))
    }

    fn write_collection_of_bool_values(
        &mut self,
        key: Option<&str>,
        values: &[bool],
    ) -> Result<(), KiotaError> {
        let arr: Vec<Value> = values.iter().map(|&b| Value::Bool(b)).collect();
        self.write_value(key, Value::Array(arr))
    }

    fn write_collection_of_i32_values(
        &mut self,
        key: Option<&str>,
        values: &[i32],
    ) -> Result<(), KiotaError> {
        let arr: Vec<Value> = values.iter().map(|&v| Value::from(v)).collect();
        self.write_value(key, Value::Array(arr))
    }

    fn write_collection_of_i64_values(
        &mut self,
        key: Option<&str>,
        values: &[i64],
    ) -> Result<(), KiotaError> {
        let arr: Vec<Value> = values.iter().map(|&v| Value::from(v)).collect();
        self.write_value(key, Value::Array(arr))
    }

    fn write_collection_of_f32_values(
        &mut self,
        key: Option<&str>,
        values: &[f32],
    ) -> Result<(), KiotaError> {
        let mut arr = Vec::with_capacity(values.len());
        for &v in values {
            let n = serde_json::Number::from_f64(v as f64)
                .ok_or_else(|| Self::err("invalid f32 value (NaN/Inf)"))?;
            arr.push(Value::Number(n));
        }
        self.write_value(key, Value::Array(arr))
    }

    fn write_collection_of_f64_values(
        &mut self,
        key: Option<&str>,
        values: &[f64],
    ) -> Result<(), KiotaError> {
        let mut arr = Vec::with_capacity(values.len());
        for &v in values {
            let n = serde_json::Number::from_f64(v)
                .ok_or_else(|| Self::err("invalid f64 value (NaN/Inf)"))?;
            arr.push(Value::Number(n));
        }
        self.write_value(key, Value::Array(arr))
    }

    fn write_enum_value<T: std::fmt::Display>(
        &mut self,
        key: Option<&str>,
        value: &T,
    ) -> Result<(), KiotaError> {
        self.write_string_value(key, &value.to_string())
    }

    fn write_collection_of_enum_values<T: std::fmt::Display>(
        &mut self,
        key: Option<&str>,
        values: &[T],
    ) -> Result<(), KiotaError> {
        let arr: Vec<Value> = values
            .iter()
            .map(|v| Value::String(v.to_string()))
            .collect();
        self.write_value(key, Value::Array(arr))
    }

    fn write_additional_data(
        &mut self,
        data: &IndexMap<String, Box<dyn Any + Send + Sync>>,
    ) -> Result<(), KiotaError> {
        for (key, val) in data {
            if let Some(s) = val.downcast_ref::<String>() {
                self.write_string_value(Some(key), s)?;
            } else if let Some(&b) = val.downcast_ref::<bool>() {
                self.write_bool_value(Some(key), b)?;
            } else if let Some(&n) = val.downcast_ref::<i64>() {
                self.write_i64_value(Some(key), n)?;
            } else if let Some(&n) = val.downcast_ref::<i32>() {
                self.write_i32_value(Some(key), n)?;
            } else if let Some(&n) = val.downcast_ref::<f64>() {
                self.write_f64_value(Some(key), n)?;
            } else if let Some(&n) = val.downcast_ref::<f32>() {
                self.write_f32_value(Some(key), n)?;
            } else {
                self.write_null_value(Some(key))?;
            }
        }
        Ok(())
    }

    fn get_serialized_content(&mut self) -> Result<Vec<u8>, KiotaError> {
        let value = if let Some(root) = self.root_value.take() {
            root
        } else {
            Value::Object(self.current.clone())
        };
        serde_json::to_vec(&value).map_err(|e| Self::err(format!("JSON serialization failed: {e}")))
    }

    fn on_before_serialization(&self) -> Option<&ParsableAction> {
        self.on_before.as_ref()
    }

    fn set_on_before_serialization(&mut self, action: Option<ParsableAction>) {
        self.on_before = action;
    }

    fn on_after_serialization(&self) -> Option<&ParsableAction> {
        self.on_after.as_ref()
    }

    fn set_on_after_serialization(&mut self, action: Option<ParsableAction>) {
        self.on_after = action;
    }

    fn on_start_serialization(&self) -> Option<&ParsableWriter> {
        self.on_start.as_ref()
    }

    fn set_on_start_serialization(&mut self, action: Option<ParsableWriter>) {
        self.on_start = action;
    }
}
