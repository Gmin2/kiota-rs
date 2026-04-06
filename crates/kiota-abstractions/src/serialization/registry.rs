use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use once_cell::sync::Lazy;

use crate::KiotaError;
use super::parse_node::ParseNode;
use super::parse_node_factory::ParseNodeFactory;
use super::serialization_writer::SerializationWriter;
use super::serialization_writer_factory::SerializationWriterFactory;

pub static PARSE_NODE_FACTORY_REGISTRY: Lazy<ParseNodeFactoryRegistry> =
    Lazy::new(ParseNodeFactoryRegistry::new);

pub static SERIALIZATION_WRITER_FACTORY_REGISTRY: Lazy<SerializationWriterFactoryRegistry> =
    Lazy::new(SerializationWriterFactoryRegistry::new);

pub struct ParseNodeFactoryRegistry {
    factories: RwLock<HashMap<String, Arc<dyn ParseNodeFactory>>>,
}

// Safety: RwLock provides interior mutability with Send+Sync
unsafe impl Send for ParseNodeFactoryRegistry {}
unsafe impl Sync for ParseNodeFactoryRegistry {}

impl ParseNodeFactoryRegistry {
    pub fn new() -> Self {
        Self {
            factories: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(&self, factory: Arc<dyn ParseNodeFactory>) {
        let ct = factory.valid_content_type().to_string();
        self.factories.write().unwrap().insert(ct, factory);
    }

    pub fn get_root_parse_node(
        &self,
        content_type: &str,
        content: &[u8],
    ) -> Result<Box<dyn ParseNode>, KiotaError> {
        // Strip parameters from content type (e.g., "application/json; charset=utf-8")
        let ct = content_type
            .split(';')
            .next()
            .unwrap_or(content_type)
            .trim();

        let factories = self.factories.read().unwrap();
        let factory = factories.get(ct).ok_or_else(|| {
            KiotaError::Deserialization(format!("no factory registered for content type: {ct}"))
        })?;
        factory.get_root_parse_node(ct, content)
    }
}

impl Default for ParseNodeFactoryRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SerializationWriterFactoryRegistry {
    factories: RwLock<HashMap<String, Arc<dyn SerializationWriterFactory>>>,
}

unsafe impl Send for SerializationWriterFactoryRegistry {}
unsafe impl Sync for SerializationWriterFactoryRegistry {}

impl SerializationWriterFactoryRegistry {
    pub fn new() -> Self {
        Self {
            factories: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(&self, factory: Arc<dyn SerializationWriterFactory>) {
        let ct = factory.valid_content_type().to_string();
        self.factories.write().unwrap().insert(ct, factory);
    }

    pub fn get_serialization_writer(
        &self,
        content_type: &str,
    ) -> Result<Box<dyn SerializationWriter>, KiotaError> {
        let ct = content_type
            .split(';')
            .next()
            .unwrap_or(content_type)
            .trim();

        let factories = self.factories.read().unwrap();
        let factory = factories.get(ct).ok_or_else(|| {
            KiotaError::Serialization(format!("no factory registered for content type: {ct}"))
        })?;
        factory.get_serialization_writer(ct)
    }
}

impl Default for SerializationWriterFactoryRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl SerializationWriterFactory for SerializationWriterFactoryRegistry {
    fn valid_content_type(&self) -> &str {
        ""
    }

    fn get_serialization_writer(
        &self,
        content_type: &str,
    ) -> Result<Box<dyn SerializationWriter>, KiotaError> {
        SerializationWriterFactoryRegistry::get_serialization_writer(self, content_type)
    }
}

impl ParseNodeFactory for ParseNodeFactoryRegistry {
    fn valid_content_type(&self) -> &str {
        ""
    }

    fn get_root_parse_node(
        &self,
        content_type: &str,
        content: &[u8],
    ) -> Result<Box<dyn ParseNode>, KiotaError> {
        ParseNodeFactoryRegistry::get_root_parse_node(self, content_type, content)
    }
}
