use std::any::Any;

/// Marker trait for request options attached to individual requests.
pub trait RequestOption: Any + Send + Sync {
    fn key(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
}
