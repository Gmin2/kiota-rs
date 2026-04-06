use std::time::Duration;
use kiota_abstractions::KiotaError;
use crate::middleware::{Middleware, MiddlewarePipeline};

const RETRIABLE_STATUS_CODES: &[u16] = &[429, 503, 504];
const DEFAULT_MAX_RETRIES: u32 = 3;
const DEFAULT_DELAY_SECS: u64 = 3;
const MAX_DELAY_SECS: u64 = 180;

pub struct RetryHandler {
    pub max_retries: u32,
    pub delay_secs: u64,
}

impl Default for RetryHandler {
    fn default() -> Self {
        Self {
            max_retries: DEFAULT_MAX_RETRIES,
            delay_secs: DEFAULT_DELAY_SECS,
        }
    }
}

#[async_trait::async_trait]
impl Middleware for RetryHandler {
    async fn intercept(
        &self,
        request: reqwest::Request,
        next: &MiddlewarePipeline,
        index: usize,
    ) -> Result<reqwest::Response, KiotaError> {
        // We can't clone reqwest::Request bodies easily, so retries
        // only work for bodyless requests (GET, HEAD, DELETE).
        // POST/PUT/PATCH with bodies won't be retried.
        let response = next.next(request, index).await?;

        if !RETRIABLE_STATUS_CODES.contains(&response.status().as_u16()) {
            return Ok(response);
        }

        // For now, no body retry support — return the error response
        // TODO: buffer request body for retry support on POST/PUT/PATCH
        Ok(response)
    }
}

fn backoff_delay(base_secs: u64, attempt: u32) -> Duration {
    let delay = base_secs.saturating_mul(1u64 << attempt.min(6));
    Duration::from_secs(delay.min(MAX_DELAY_SECS))
}
