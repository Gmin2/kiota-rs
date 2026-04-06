use std::collections::HashMap;

use crate::error::KiotaError;
use crate::headers::RequestHeaders;
use crate::http_method::HttpMethod;
use crate::request_option::RequestOption;

pub struct RequestInformation {
    pub method: HttpMethod,
    pub url_template: String,
    pub path_parameters: HashMap<String, String>,
    pub query_parameters: HashMap<String, String>,
    pub headers: RequestHeaders,
    pub content: Option<Vec<u8>>,
    options: HashMap<String, Box<dyn RequestOption>>,
}

impl RequestInformation {
    pub fn new() -> Self {
        Self {
            method: HttpMethod::Get,
            url_template: String::new(),
            path_parameters: HashMap::new(),
            query_parameters: HashMap::new(),
            headers: RequestHeaders::new(),
            content: None,
            options: HashMap::new(),
        }
    }

    pub fn new_with_method_and_url_template(
        method: HttpMethod,
        url_template: &str,
        path_parameters: HashMap<String, String>,
    ) -> Self {
        Self {
            method,
            url_template: url_template.to_string(),
            path_parameters,
            query_parameters: HashMap::new(),
            headers: RequestHeaders::new(),
            content: None,
            options: HashMap::new(),
        }
    }

    /// Expands the URL template with path and query parameters.
    pub fn get_uri(&self) -> Result<url::Url, KiotaError> {
        // TODO: use std-uritemplate for RFC 6570 expansion
        let mut uri = self.url_template.clone();
        for (key, value) in &self.path_parameters {
            uri = uri.replace(&format!("{{{key}}}"), value);
            uri = uri.replace(&format!("{{+{key}}}"), value);
        }
        // Strip remaining template expressions
        if let Some(idx) = uri.find('{') {
            uri.truncate(idx);
        }
        url::Url::parse(&uri).map_err(|e| KiotaError::Url(e.to_string()))
    }

    pub fn add_request_options(&mut self, options: Vec<Box<dyn RequestOption>>) {
        for opt in options {
            self.options.insert(opt.key().to_string(), opt);
        }
    }

    pub fn get_request_option<T: RequestOption + 'static>(&self, key: &str) -> Option<&T> {
        self.options
            .get(key)
            .and_then(|o| o.as_any().downcast_ref::<T>())
    }

    pub fn add_query_parameters<T: QueryParameters>(&mut self, source: &T) {
        for (key, value) in source.to_query_parameters() {
            self.query_parameters.insert(key, value);
        }
    }

    pub fn set_stream_content(&mut self, content: Vec<u8>, content_type: &str) {
        self.content = Some(content);
        self.headers.try_add("Content-Type", content_type);
    }

    pub fn configure<Q: QueryParameters>(&mut self, config: &RequestConfiguration<Q>) {
        if let Some(ref qp) = config.query_parameters {
            self.add_query_parameters(qp);
        }
        self.headers.add_all(&config.headers);
        // TODO: add options from config
    }
}

impl Default for RequestInformation {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait implemented by generated query parameter structs.
pub trait QueryParameters {
    fn to_query_parameters(&self) -> HashMap<String, String>;
}

pub struct RequestConfiguration<Q: QueryParameters> {
    pub headers: RequestHeaders,
    pub options: Vec<Box<dyn RequestOption>>,
    pub query_parameters: Option<Q>,
}

impl<Q: QueryParameters + Default> Default for RequestConfiguration<Q> {
    fn default() -> Self {
        Self {
            headers: RequestHeaders::new(),
            options: Vec::new(),
            query_parameters: None,
        }
    }
}

/// Default empty query parameters for request builders without query params.
#[derive(Debug, Default)]
pub struct DefaultQueryParameters;

impl QueryParameters for DefaultQueryParameters {
    fn to_query_parameters(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}
