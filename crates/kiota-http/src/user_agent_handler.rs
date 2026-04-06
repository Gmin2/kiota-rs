use kiota_abstractions::KiotaError;
use crate::middleware::{Middleware, MiddlewarePipeline};

const PRODUCT_NAME: &str = "kiota-rust";
const PRODUCT_VERSION: &str = "0.1.0";

pub struct UserAgentHandler;

#[async_trait::async_trait]
impl Middleware for UserAgentHandler {
    async fn intercept(
        &self,
        mut request: reqwest::Request,
        next: &MiddlewarePipeline,
        index: usize,
    ) -> Result<reqwest::Response, KiotaError> {
        let value = format!("{PRODUCT_NAME}/{PRODUCT_VERSION}");
        if !request.headers().contains_key("user-agent") {
            request.headers_mut().insert(
                "user-agent",
                value.parse().unwrap(),
            );
        }
        next.next(request, index).await
    }
}
