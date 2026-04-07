use kiota_abstractions::serialization::{ParseNode, ParseNodeFactory};
use kiota_abstractions::KiotaError;

use crate::TextParseNode;

pub struct TextParseNodeFactory;

impl ParseNodeFactory for TextParseNodeFactory {
    fn valid_content_type(&self) -> &str {
        "text/plain"
    }
    fn get_root_parse_node(&self, content_type: &str, content: &[u8]) -> Result<Box<dyn ParseNode>, KiotaError> {
        if !content_type.starts_with("text/plain") {
            return Err(KiotaError::Deserialization(format!("expected text/plain, got {content_type}")));
        }
        let text = String::from_utf8_lossy(content).to_string();
        Ok(Box::new(TextParseNode::new(text)))
    }
}
