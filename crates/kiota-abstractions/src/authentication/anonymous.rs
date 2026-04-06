use std::collections::HashMap;

use crate::request_information::RequestInformation;
use crate::KiotaError;
use super::AuthenticationProvider;

/// No-op authentication provider for unauthenticated APIs.
pub struct AnonymousAuthenticationProvider;

#[async_trait::async_trait]
impl AuthenticationProvider for AnonymousAuthenticationProvider {
    async fn authenticate_request(
        &self,
        _request: &mut RequestInformation,
        _additional_context: Option<&HashMap<String, String>>,
    ) -> Result<(), KiotaError> {
        Ok(())
    }
}
