use crate::KiotaError;
use super::serialization_writer::SerializationWriter;

pub trait SerializationWriterFactory: Send + Sync {
    fn valid_content_type(&self) -> &str;
    fn get_serialization_writer(
        &self,
        content_type: &str,
    ) -> Result<Box<dyn SerializationWriter>, KiotaError>;
}
