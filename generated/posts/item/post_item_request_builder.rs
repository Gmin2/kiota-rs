/// Builds and executes requests for operations under \posts\{post-id}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostItemRequestBuilder {
    /// Instantiates a new PostItemRequestBuilder and sets the default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Instantiates a new PostItemRequestBuilder and sets the default values.
    pub fn with_url(raw_url: &str, request_adapter: std::sync::Arc<dyn RequestAdapter>) -> Self {
        let mut path_parameters = std::collections::HashMap::new();
        path_parameters.insert("request-raw-url".to_string(), raw_url.to_string());
        Self {
            base: BaseRequestBuilder::new(request_adapter, "".to_string(), path_parameters),
        }
    }
    /// Delete post
    pub async fn delete(&self    , request_configuration: Option<&RequestConfiguration<DefaultQueryParameters>>    ) -> Result<Option<Vec<u8>>, KiotaError> {
        let request_info = self.to_delete_request_information(        request_configuration        )?;
        let _result = self.base.request_adapter.send(&request_info, &Box::new(|_| Err(KiotaError::General("not implemented".to_string()))), None).await?;
        todo!("deserialize response")
    }
    /// Get post by ID
    pub async fn get(&self    , request_configuration: Option<&RequestConfiguration<DefaultQueryParameters>>    ) -> Result<Option<Post>, KiotaError> {
        let request_info = self.to_get_request_information(        request_configuration        )?;
        let _result = self.base.request_adapter.send(&request_info, &Box::new(|_| Err(KiotaError::General("not implemented".to_string()))), None).await?;
        todo!("deserialize response")
    }
    /// Update post
    pub async fn patch(&self    , body: &Option<Post>    , request_configuration: Option<&RequestConfiguration<DefaultQueryParameters>>    ) -> Result<Option<Post>, KiotaError> {
        let request_info = self.to_patch_request_information(        body,         request_configuration        )?;
        let _result = self.base.request_adapter.send(&request_info, &Box::new(|_| Err(KiotaError::General("not implemented".to_string()))), None).await?;
        todo!("deserialize response")
    }
    /// Delete post
    pub fn to_to_delete_request_information_request_information(&self    , request_configuration: Option<&RequestConfiguration<DefaultQueryParameters>>    ) -> Result<RequestInformation, KiotaError> {
        let mut request_info = RequestInformation::new_with_method_and_url_template(HttpMethod::DELETE, &self.base.url_template, self.base.path_parameters.clone());
        if let Some(config) = request_configuration {
            request_info.configure(config);
        }
        Ok(request_info)
    }
    /// Get post by ID
    pub fn to_to_get_request_information_request_information(&self    , request_configuration: Option<&RequestConfiguration<DefaultQueryParameters>>    ) -> Result<RequestInformation, KiotaError> {
        let mut request_info = RequestInformation::new_with_method_and_url_template(HttpMethod::GET, &self.base.url_template, self.base.path_parameters.clone());
        if let Some(config) = request_configuration {
            request_info.configure(config);
        }
        request_info.headers.try_add("Accept", "application/json");
        Ok(request_info)
    }
    /// Update post
    pub fn to_to_patch_request_information_request_information(&self    , body: &Option<Post>    , request_configuration: Option<&RequestConfiguration<DefaultQueryParameters>>    ) -> Result<RequestInformation, KiotaError> {
        let mut request_info = RequestInformation::new_with_method_and_url_template(HttpMethod::PATCH, &self.base.url_template, self.base.path_parameters.clone());
        if let Some(config) = request_configuration {
            request_info.configure(config);
        }
        request_info.headers.try_add("Accept", "application/json");
        Ok(request_info)
    }
    /// Returns a request builder with the provided arbitrary URL. Using this method means any other path or query parameters are ignored.
    pub fn with_url(&self) -> PostItemRequestBuilder {
        todo!()
    }
}
