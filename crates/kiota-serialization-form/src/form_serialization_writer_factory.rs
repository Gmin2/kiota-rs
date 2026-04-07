use kiota_abstractions::serialization::{SerializationWriter, SerializationWriterFactory};
use kiota_abstractions::KiotaError;
use crate::FormSerializationWriter;

pub struct FormSerializationWriterFactory;

impl SerializationWriterFactory for FormSerializationWriterFactory {
    fn valid_content_type(&self) -> &str {
        "application/x-www-form-urlencoded"
    }
    fn get_serialization_writer(&self, content_type: &str) -> Result<Box<dyn SerializationWriter>, KiotaError> {
        if !content_type.contains("form-urlencoded") {
            return Err(KiotaError::Serialization(format!("expected form-urlencoded, got {content_type}")));
        }
        Ok(Box::new(FormSerializationWriter::new()))
    }
}
