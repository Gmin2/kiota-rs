use std::collections::HashMap;
use std::sync::Arc;

use crate::RequestAdapter;

pub struct BaseRequestBuilder {
    pub path_parameters: HashMap<String, String>,
    pub request_adapter: Arc<dyn RequestAdapter>,
    pub url_template: String,
}

impl BaseRequestBuilder {
    pub fn new(
        request_adapter: Arc<dyn RequestAdapter>,
        url_template: &str,
        path_parameters: HashMap<String, String>,
    ) -> Self {
        Self {
            path_parameters,
            request_adapter,
            url_template: url_template.to_string(),
        }
    }
}
