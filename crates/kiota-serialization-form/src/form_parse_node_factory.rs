use kiota_abstractions::serialization::{ParseNode, ParseNodeFactory};
use kiota_abstractions::KiotaError;
use crate::FormParseNode;

pub struct FormParseNodeFactory;

impl ParseNodeFactory for FormParseNodeFactory {
    fn valid_content_type(&self) -> &str {
        "application/x-www-form-urlencoded"
    }
    fn get_root_parse_node(&self, content_type: &str, content: &[u8]) -> Result<Box<dyn ParseNode>, KiotaError> {
        if !content_type.contains("form-urlencoded") {
            return Err(KiotaError::Deserialization(format!("expected form-urlencoded, got {content_type}")));
        }
        let text = String::from_utf8_lossy(content);
        Ok(Box::new(FormParseNode::new(&text)))
    }
}
