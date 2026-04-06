use crate::KiotaError;
use super::parse_node::ParseNode;

pub trait ParseNodeFactory: Send + Sync {
    fn valid_content_type(&self) -> &str;
    fn get_root_parse_node(
        &self,
        content_type: &str,
        content: &[u8],
    ) -> Result<Box<dyn ParseNode>, KiotaError>;
}
