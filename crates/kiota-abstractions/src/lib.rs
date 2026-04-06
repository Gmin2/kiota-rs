//! Core abstractions for Kiota-generated Rust API clients.
//!
//! This crate provides the foundational traits and types that all
//! Kiota-generated SDK code depends on at compile time.

pub mod authentication;
pub mod error;
pub mod headers;
pub mod http_method;
pub mod iso_duration;
pub mod nullable;
pub mod request_information;
pub mod request_option;
pub mod serialization;
pub mod store;
pub mod untyped;

mod base_request_builder;

pub use base_request_builder::BaseRequestBuilder;
pub use error::{ApiError, KiotaError};
pub use headers::{RequestHeaders, ResponseHeaders};
pub use http_method::HttpMethod;
pub use iso_duration::IsoDuration;
pub use nullable::Nullable;
pub use request_information::{
    DefaultQueryParameters, QueryParameters, RequestConfiguration, RequestInformation,
};
pub use request_option::RequestOption;
pub use serialization::{
    AdditionalDataHolder, FieldDeserializers, Parsable, ParsableFactory, ParseNode,
    ParseNodeFactory, SerializationWriter, SerializationWriterFactory,
};

/// Type alias for error mappings: HTTP status pattern -> parsable factory.
pub type ErrorMappings = std::collections::HashMap<
    String,
    Box<dyn Fn(&dyn ParseNode) -> Result<Box<dyn Parsable>, KiotaError> + Send + Sync>,
>;

/// Type-erased parsable factory for use in the request adapter.
pub type ErasedParsableFactory =
    Box<dyn Fn(&dyn ParseNode) -> Result<Box<dyn Parsable>, KiotaError> + Send + Sync>;

/// The request adapter trait for executing HTTP requests.
///
/// Generated code calls these methods through type-erased factories.
/// The generated request builders handle the type casting.
#[async_trait::async_trait]
pub trait RequestAdapter: Send + Sync {
    /// Sends a request and deserializes the response body.
    async fn send(
        &self,
        request_info: &RequestInformation,
        factory: &ErasedParsableFactory,
        error_mappings: Option<&ErrorMappings>,
    ) -> Result<Option<Box<dyn Parsable>>, KiotaError>;

    /// Sends a request expecting no response body.
    async fn send_no_content(
        &self,
        request_info: &RequestInformation,
        error_mappings: Option<&ErrorMappings>,
    ) -> Result<(), KiotaError>;

    /// Sends a request and returns the raw response bytes.
    async fn send_primitive(
        &self,
        request_info: &RequestInformation,
        error_mappings: Option<&ErrorMappings>,
    ) -> Result<Option<Vec<u8>>, KiotaError>;

    fn serialization_writer_factory(&self) -> &dyn SerializationWriterFactory;
    fn base_url(&self) -> &str;
    fn set_base_url(&mut self, base_url: &str);
}
