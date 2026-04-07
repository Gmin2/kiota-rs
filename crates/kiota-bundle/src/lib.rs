//! Default bundle for Kiota-generated Rust API clients.
//!
//! Re-exports core types and registers all default serialization factories.

use std::sync::Arc;

pub use kiota_abstractions;
pub use kiota_http;
pub use kiota_serialization_json;
pub use kiota_serialization_form;
pub use kiota_serialization_text;
pub use kiota_serialization_multipart;

use kiota_abstractions::authentication::AuthenticationProvider;
use kiota_abstractions::serialization::registry::{
    PARSE_NODE_FACTORY_REGISTRY, SERIALIZATION_WRITER_FACTORY_REGISTRY,
};
use kiota_http::HttpClientRequestAdapter;

/// Creates a fully configured request adapter with all default serializers registered.
pub fn create_default_request_adapter(
    auth_provider: Arc<dyn AuthenticationProvider>,
) -> Result<HttpClientRequestAdapter, kiota_abstractions::KiotaError> {
    register_defaults();
    HttpClientRequestAdapter::new(auth_provider)
}

/// Registers all default serialization and deserialization factories.
pub fn register_defaults() {
    SERIALIZATION_WRITER_FACTORY_REGISTRY
        .register(Arc::new(kiota_serialization_json::JsonSerializationWriterFactory));
    SERIALIZATION_WRITER_FACTORY_REGISTRY
        .register(Arc::new(kiota_serialization_text::TextSerializationWriterFactory));
    SERIALIZATION_WRITER_FACTORY_REGISTRY
        .register(Arc::new(kiota_serialization_form::FormSerializationWriterFactory));
    SERIALIZATION_WRITER_FACTORY_REGISTRY
        .register(Arc::new(kiota_serialization_multipart::MultipartSerializationWriterFactory));

    PARSE_NODE_FACTORY_REGISTRY
        .register(Arc::new(kiota_serialization_json::JsonParseNodeFactory));
    PARSE_NODE_FACTORY_REGISTRY
        .register(Arc::new(kiota_serialization_text::TextParseNodeFactory));
    PARSE_NODE_FACTORY_REGISTRY
        .register(Arc::new(kiota_serialization_form::FormParseNodeFactory));
}
