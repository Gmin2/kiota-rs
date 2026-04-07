use kiota_abstractions::serialization::{SerializationWriter, SerializationWriterFactory};
use kiota_abstractions::KiotaError;

use crate::TextSerializationWriter;

pub struct TextSerializationWriterFactory;

impl SerializationWriterFactory for TextSerializationWriterFactory {
    fn valid_content_type(&self) -> &str {
        "text/plain"
    }
    fn get_serialization_writer(&self, content_type: &str) -> Result<Box<dyn SerializationWriter>, KiotaError> {
        if !content_type.starts_with("text/plain") {
            return Err(KiotaError::Serialization(format!("expected text/plain, got {content_type}")));
        }
        Ok(Box::new(TextSerializationWriter::new()))
    }
}
