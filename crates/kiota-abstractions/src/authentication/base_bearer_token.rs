use std::collections::HashMap;

use crate::request_information::RequestInformation;
use crate::KiotaError;
use super::{AccessTokenProvider, AuthenticationProvider};

const AUTHORIZATION_HEADER: &str = "Authorization";

pub struct BaseBearerTokenAuthenticationProvider<T: AccessTokenProvider> {
    token_provider: T,
}

impl<T: AccessTokenProvider> BaseBearerTokenAuthenticationProvider<T> {
    pub fn new(token_provider: T) -> Self {
        Self { token_provider }
    }
}

#[async_trait::async_trait]
impl<T: AccessTokenProvider> AuthenticationProvider
    for BaseBearerTokenAuthenticationProvider<T>
{
    async fn authenticate_request(
        &self,
        request: &mut RequestInformation,
        additional_context: Option<&HashMap<String, String>>,
    ) -> Result<(), KiotaError> {
        let uri = request.get_uri()?;
        if !self
            .token_provider
            .allowed_hosts_validator()
            .is_url_host_valid(&uri)
        {
            return Ok(());
        }
        if request.headers.contains_key(AUTHORIZATION_HEADER) {
            return Ok(());
        }
        let token = self
            .token_provider
            .get_authorization_token(&uri, additional_context)
            .await?;
        if let Some(token) = token {
            request
                .headers
                .add(AUTHORIZATION_HEADER, &format!("Bearer {token}"));
        }
        Ok(())
    }
}
