use crate::models::suite::TestSuite;
use crate::models::test::Test;
use reqwest::{Client, Response};
use serde_json::Value;
use std::collections::HashMap;

pub struct HttpClient {
    client: Client,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl HttpClient {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn execute_request(
        &self,
        test: &Test,
        test_suite: &TestSuite,
        variables: &HashMap<String, String>,
    ) -> Result<Response, reqwest::Error> {
        let base_url = &test_suite.config.base_url;
        let endpoint = replace_variables(&test.endpoint, variables);
        let url = format!("{}{}", base_url, endpoint);

        let mut request_builder = match test.method.to_uppercase().as_str() {
            "GET" => self.client.get(&url),
            "POST" => self.client.post(&url),
            "PUT" => self.client.put(&url),
            "DELETE" => self.client.delete(&url),
            "PATCH" => self.client.patch(&url),
            "HEAD" => self.client.head(&url),
            "OPTIONS" => self.client.request(reqwest::Method::OPTIONS, &url),
            _ => panic!("Unsupported HTTP method: {}", test.method),
        };

        // Add default headers from config
        if let Some(default_headers) = &test_suite.config.default_headers {
            for (key, value) in default_headers {
                request_builder = request_builder.header(key, replace_variables(value, variables));
            }
        }

        // Add test-specific headers
        if let Some(headers) = &test.headers {
            for (key, value) in headers {
                request_builder = request_builder.header(key, replace_variables(value, variables));
            }
        }

        // Add query parameters
        if let Some(query_params) = &test.query_params {
            let mut processed_params = HashMap::new();
            for (key, value) in query_params {
                processed_params.insert(key, replace_variables(value, variables));
            }
            request_builder = request_builder.query(&processed_params);
        }

        // Add authentication if specified
        if let Some(auth_method) = &test_suite.config.auth_method {
            if let Some(auth_token) = &test_suite.config.auth_token {
                let token = replace_variables(auth_token, variables);
                match auth_method.to_lowercase().as_str() {
                    "bearer" => {
                        request_builder =
                            request_builder.header("Authorization", format!("Bearer {}", token));
                    }
                    "basic" => {
                        request_builder =
                            request_builder.header("Authorization", format!("Basic {}", token));
                    }
                    _ => {}
                }
            }
        }

        // Add body if present
        if let Some(body) = &test.body {
            let body_str = serde_json::to_string(body).unwrap_or_default();
            let processed_body: Value =
                serde_json::from_str(&replace_variables(&body_str, variables))
                    .unwrap_or(Value::Null);
            request_builder = request_builder.json(&processed_body);
        }

        request_builder.send().await
    }
}

// Helper function to replace variables in strings
pub fn replace_variables(input: &str, variables: &HashMap<String, String>) -> String {
    let mut result = input.to_string();
    for (key, value) in variables {
        let pattern = format!("{{{{{}}}}}", key);
        result = result.replace(&pattern, value);
    }
    result
}
