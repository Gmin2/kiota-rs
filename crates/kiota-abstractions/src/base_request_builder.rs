use std::collections::HashMap;
use std::sync::Arc;

use crate::RequestAdapter;

pub struct BaseRequestBuilder {
    pub path_parameters: HashMap<String, String>,
    pub request_adapter: Arc<dyn RequestAdapter>,
    pub url_template: String,
}

impl BaseRequestBuilder {
    pub fn new(
        request_adapter: Arc<dyn RequestAdapter>,
        url_template: String,
        path_parameters: HashMap<String, String>,
    ) -> Self {
        Self {
            path_parameters,
            request_adapter,
            url_template,
        }
    }
}

impl Clone for BaseRequestBuilder {
    fn clone(&self) -> Self {
        Self {
            path_parameters: self.path_parameters.clone(),
            request_adapter: self.request_adapter.clone(),
            url_template: self.url_template.clone(),
        }
    }
}

impl std::fmt::Debug for BaseRequestBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BaseRequestBuilder")
            .field("url_template", &self.url_template)
            .field("path_parameters", &self.path_parameters)
            .finish()
    }
}

impl Default for BaseRequestBuilder {
    fn default() -> Self {
        // This is only used by derive(Default) on generated request builders.
        // Real construction goes through BaseRequestBuilder::new().
        Self {
            path_parameters: HashMap::new(),
            request_adapter: Arc::new(NullAdapter),
            url_template: String::new(),
        }
    }
}

impl PartialEq for BaseRequestBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.url_template == other.url_template && self.path_parameters == other.path_parameters
    }
}

/// Placeholder adapter used only by Default impl. Panics if actually called.
struct NullAdapter;

#[async_trait::async_trait]
impl crate::RequestAdapter for NullAdapter {
    async fn send(
        &self,
        _: &crate::RequestInformation,
        _: &crate::ErasedParsableFactory,
        _: Option<&crate::ErrorMappings>,
    ) -> Result<Option<Box<dyn crate::Parsable>>, crate::KiotaError> {
        panic!("NullAdapter: no request adapter configured")
    }
    async fn send_no_content(
        &self,
        _: &crate::RequestInformation,
        _: Option<&crate::ErrorMappings>,
    ) -> Result<(), crate::KiotaError> {
        panic!("NullAdapter: no request adapter configured")
    }
    async fn send_primitive(
        &self,
        _: &crate::RequestInformation,
        _: Option<&crate::ErrorMappings>,
    ) -> Result<Option<Vec<u8>>, crate::KiotaError> {
        panic!("NullAdapter: no request adapter configured")
    }
    fn serialization_writer_factory(&self) -> &dyn crate::SerializationWriterFactory {
        panic!("NullAdapter: no request adapter configured")
    }
    fn base_url(&self) -> &str { "" }
    fn set_base_url(&mut self, _: &str) {}
}
