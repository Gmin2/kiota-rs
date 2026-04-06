/// Builds and executes requests for operations under \posts
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PostsRequestBuilder {
    pub fn by_post_id(&self    , post_id: i32    ) -> PostItemRequestBuilder {
        let mut url_tpl_params = self.base.path_parameters.clone();
        url_tpl_params.insert("post%2Did".to_string(), post_id.to_string());
        PostItemRequestBuilder::new(url_tpl_params, self.base.request_adapter.clone())
    }
    /// Instantiates a new PostsRequestBuilder and sets the default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Instantiates a new PostsRequestBuilder and sets the default values.
    pub fn with_url(raw_url: &str, request_adapter: std::sync::Arc<dyn RequestAdapter>) -> Self {
        let mut path_parameters = std::collections::HashMap::new();
        path_parameters.insert("request-raw-url".to_string(), raw_url.to_string());
        Self {
            base: BaseRequestBuilder::new(request_adapter, "".to_string(), path_parameters),
        }
    }
    /// Get posts
    pub async fn get(&self    , request_configuration: Option<&RequestConfiguration<DefaultQueryParameters>>    ) -> Result<Vec<Post>, KiotaError> {
        let request_info = self.to_get_request_information(        request_configuration        )?;
        let _result = self.base.request_adapter.send(&request_info, &Box::new(|_| Err(KiotaError::General("not implemented".to_string()))), None).await?;
        todo!("deserialize response")
    }
    /// Create post
    pub async fn post(&self    , body: &Option<Post>    , request_configuration: Option<&RequestConfiguration<DefaultQueryParameters>>    ) -> Result<Option<Post>, KiotaError> {
        let request_info = self.to_post_request_information(        body,         request_configuration        )?;
        let _result = self.base.request_adapter.send(&request_info, &Box::new(|_| Err(KiotaError::General("not implemented".to_string()))), None).await?;
        todo!("deserialize response")
    }
    /// Get posts
    pub fn to_to_get_request_information_request_information(&self    , request_configuration: Option<&RequestConfiguration<DefaultQueryParameters>>    ) -> Result<RequestInformation, KiotaError> {
        let mut request_info = RequestInformation::new_with_method_and_url_template(HttpMethod::GET, &self.base.url_template, self.base.path_parameters.clone());
        if let Some(config) = request_configuration {
            request_info.configure(config);
        }
        request_info.headers.try_add("Accept", "application/json");
        Ok(request_info)
    }
    /// Create post
    pub fn to_to_post_request_information_request_information(&self    , body: &Option<Post>    , request_configuration: Option<&RequestConfiguration<DefaultQueryParameters>>    ) -> Result<RequestInformation, KiotaError> {
        let mut request_info = RequestInformation::new_with_method_and_url_template(HttpMethod::POST, &self.base.url_template, self.base.path_parameters.clone());
        if let Some(config) = request_configuration {
            request_info.configure(config);
        }
        request_info.headers.try_add("Accept", "application/json");
        Ok(request_info)
    }
    /// Returns a request builder with the provided arbitrary URL. Using this method means any other path or query parameters are ignored.
    pub fn with_url(&self) -> PostsRequestBuilder {
        todo!()
    }
    /// Get posts
    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct PostsRequestBuilderGetQueryParameters {
        /// Filter results by title
        pub title: Option<String>,
        /// Filter results by user ID
        #[serde(rename = "userId")]
        pub user_id: Option<i32>,
    }
}
