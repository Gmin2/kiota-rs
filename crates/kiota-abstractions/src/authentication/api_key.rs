use std::collections::HashMap;

use crate::request_information::RequestInformation;
use crate::KiotaError;
use super::{AllowedHostsValidator, AuthenticationProvider};

#[derive(Debug, Clone, Copy)]
pub enum ApiKeyLocation {
    Header,
    QueryParameter,
}

pub struct ApiKeyAuthenticationProvider {
    api_key: String,
    parameter_name: String,
    location: ApiKeyLocation,
    allowed_hosts: AllowedHostsValidator,
}

impl ApiKeyAuthenticationProvider {
    pub fn new(
        api_key: String,
        parameter_name: String,
        location: ApiKeyLocation,
        allowed_hosts: Vec<String>,
    ) -> Self {
        Self {
            api_key,
            parameter_name,
            location,
            allowed_hosts: AllowedHostsValidator::new(allowed_hosts),
        }
    }
}

#[async_trait::async_trait]
impl AuthenticationProvider for ApiKeyAuthenticationProvider {
    async fn authenticate_request(
        &self,
        request: &mut RequestInformation,
        _additional_context: Option<&HashMap<String, String>>,
    ) -> Result<(), KiotaError> {
        let uri = request.get_uri()?;
        if !self.allowed_hosts.is_url_host_valid(&uri) {
            return Ok(());
        }
        match self.location {
            ApiKeyLocation::Header => {
                request.headers.try_add(&self.parameter_name, &self.api_key);
            }
            ApiKeyLocation::QueryParameter => {
                request
                    .query_parameters
                    .insert(self.parameter_name.clone(), self.api_key.clone());
            }
        }
        Ok(())
    }
}
