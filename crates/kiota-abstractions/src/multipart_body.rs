use crate::KiotaError;
use crate::serialization::serialization_writer::SerializationWriter;

/// Placeholder for multipart form-data request bodies.
/// TODO: full implementation with part management.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MultipartBody {
    parts: Vec<(String, String, Vec<u8>)>, // (name, content_type, data)
}

impl MultipartBody {
    pub fn add_or_replace(&mut self, name: &str, content_type: &str, data: Vec<u8>) {
        self.parts.retain(|(n, _, _)| n != name);
        self.parts.push((name.to_string(), content_type.to_string(), data));
    }
}

impl crate::Parsable for MultipartBody {
    fn field_names(&self) -> Vec<&'static str> { vec![] }
    fn assign_field(&mut self, _: &str, _: &dyn crate::ParseNode) -> Result<(), KiotaError> { Ok(()) }
    fn serialize(&self, _writer: &mut dyn SerializationWriter) -> Result<(), KiotaError> {
        // TODO: write multipart boundary + parts
        Ok(())
    }
    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> { self }
}
