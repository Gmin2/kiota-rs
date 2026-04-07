use kiota_abstractions::serialization::{SerializationWriter, SerializationWriterFactory};
use kiota_abstractions::KiotaError;
use crate::MultipartSerializationWriter;

pub struct MultipartSerializationWriterFactory;

impl SerializationWriterFactory for MultipartSerializationWriterFactory {
    fn valid_content_type(&self) -> &str {
        "multipart/form-data"
    }
    fn get_serialization_writer(&self, content_type: &str) -> Result<Box<dyn SerializationWriter>, KiotaError> {
        if !content_type.contains("multipart") {
            return Err(KiotaError::Serialization(format!("expected multipart/form-data, got {content_type}")));
        }
        Ok(Box::new(MultipartSerializationWriter::new()))
    }
}
