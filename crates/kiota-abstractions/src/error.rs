use crate::headers::ResponseHeaders;

#[derive(Debug, Clone, thiserror::Error)]
#[error("{message}")]
pub struct ApiError {
    pub message: String,
    pub response_status_code: i32,
    pub response_headers: ResponseHeaders,
}

#[derive(Debug, thiserror::Error)]
pub enum KiotaError {
    #[error("API error: {0}")]
    Api(#[from] ApiError),
    #[error("serialization: {0}")]
    Serialization(String),
    #[error("deserialization: {0}")]
    Deserialization(String),
    #[error("authentication: {0}")]
    Authentication(String),
    #[error("invalid URL: {0}")]
    Url(String),
    #[error("HTTP: {0}")]
    Http(String),
    #[error("{0}")]
    General(String),
}
