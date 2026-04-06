use kiota_abstractions::serialization::{SerializationWriter, SerializationWriterFactory};
use kiota_abstractions::KiotaError;

use crate::json_serialization_writer::JsonSerializationWriter;

pub struct JsonSerializationWriterFactory;

impl JsonSerializationWriterFactory {
    pub fn new() -> Self {
        Self
    }
}

impl Default for JsonSerializationWriterFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl SerializationWriterFactory for JsonSerializationWriterFactory {
    fn valid_content_type(&self) -> &str {
        "application/json"
    }

    fn get_serialization_writer(
        &self,
        content_type: &str,
    ) -> Result<Box<dyn SerializationWriter>, KiotaError> {
        if content_type != self.valid_content_type() {
            return Err(KiotaError::Serialization(format!(
                "expected {}, got {content_type}",
                self.valid_content_type()
            )));
        }
        Ok(Box::new(JsonSerializationWriter::new()))
    }
}
