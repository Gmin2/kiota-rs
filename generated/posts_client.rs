/// The main entry point of the SDK, exposes the configuration and the fluent API.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostsClient {
    /// Instantiates a new PostsClient and sets the default values.
    pub fn new(request_adapter: std::sync::Arc<dyn RequestAdapter>) -> Self {
        let mut path_parameters = std::collections::HashMap::new();
        Self {
            base: BaseRequestBuilder::new(request_adapter, "{+baseurl}".to_string(), path_parameters),
        }
    }
}
