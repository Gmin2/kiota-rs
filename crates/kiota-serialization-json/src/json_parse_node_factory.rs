use kiota_abstractions::serialization::{ParseNode, ParseNodeFactory};
use kiota_abstractions::KiotaError;

use crate::json_parse_node::JsonParseNode;

pub struct JsonParseNodeFactory;

impl JsonParseNodeFactory {
    pub fn new() -> Self {
        Self
    }
}

impl Default for JsonParseNodeFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl ParseNodeFactory for JsonParseNodeFactory {
    fn valid_content_type(&self) -> &str {
        "application/json"
    }

    fn get_root_parse_node(
        &self,
        content_type: &str,
        content: &[u8],
    ) -> Result<Box<dyn ParseNode>, KiotaError> {
        if content_type != self.valid_content_type() {
            return Err(KiotaError::Deserialization(format!(
                "expected {}, got {content_type}",
                self.valid_content_type()
            )));
        }
        let value: serde_json::Value = serde_json::from_slice(content)
            .map_err(|e| KiotaError::Deserialization(format!("invalid JSON: {e}")))?;
        Ok(Box::new(JsonParseNode::new(value)))
    }
}
