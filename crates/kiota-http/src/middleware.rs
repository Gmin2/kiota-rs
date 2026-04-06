use kiota_abstractions::KiotaError;

/// Middleware trait for the HTTP pipeline.
/// Each middleware processes the request and calls `next` to pass it along.
#[async_trait::async_trait]
pub trait Middleware: Send + Sync {
    async fn intercept(
        &self,
        request: reqwest::Request,
        next: &MiddlewarePipeline,
        index: usize,
    ) -> Result<reqwest::Response, KiotaError>;
}

pub struct MiddlewarePipeline {
    middlewares: Vec<Box<dyn Middleware>>,
    client: reqwest::Client,
}

impl MiddlewarePipeline {
    pub fn new(client: reqwest::Client, middlewares: Vec<Box<dyn Middleware>>) -> Self {
        Self { middlewares, client }
    }

    pub async fn execute(&self, request: reqwest::Request) -> Result<reqwest::Response, KiotaError> {
        self.next(request, 0).await
    }

    pub async fn next(
        &self,
        request: reqwest::Request,
        index: usize,
    ) -> Result<reqwest::Response, KiotaError> {
        if index < self.middlewares.len() {
            return self.middlewares[index].intercept(request, self, index + 1).await;
        }
        self.client
            .execute(request)
            .await
            .map_err(|e| KiotaError::Http(e.to_string()))
    }
}
