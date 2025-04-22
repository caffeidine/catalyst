use crate::models::config::Config;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

pub struct RequestData {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub params: Vec<(String, String)>,
    pub body: Option<Value>,
}

pub struct HttpClient {
    client: Client,
    config: Config,
}

impl HttpClient {
    pub fn new(config: &Config) -> Self {
        HttpClient {
            client: Client::new(),
            config: config.clone(),
        }
    }

    pub async fn execute(
        &self,
        request: RequestData,
    ) -> Result<(u16, Value, HashMap<String, String>), String> {
        let url = format!("{}{}", self.config.base_url, request.url);

        let mut builder = self
            .client
            .request(request.method.parse().unwrap(), url)
            .query(&request.params);

        if let Some(default_headers) = &self.config.default_headers {
            for (k, v) in default_headers {
                builder = builder.header(k, v);
            }
        }

        for (k, v) in request.headers {
            builder = builder.header(k, v);
        }

        if let Some(body) = request.body {
            builder = builder.json(&body);
        }

        let response = builder.send().await.map_err(|e| e.to_string())?;
        let status = response.status().as_u16();

        let headers: HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or_default().to_string()))
            .collect();

        let body = if response.content_length().unwrap_or(0) == 0 {
            Value::Null
        } else {
            match response.text().await {
                Ok(text) => match serde_json::from_str(&text) {
                    Ok(json) => json,
                    Err(_) => Value::String(text),
                },
                Err(_) => Value::String("Failed to read response body".into()),
            }
        };

        Ok((status, body, headers))
    }
}
