use super::{variables, verify};
use crate::debug;
use crate::http::client::{HttpClient, RequestData};
use crate::models::test::Test;
use crate::utils::{file::load_body_from_file, string::replace_variables};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

pub struct ExecutionResult {
    pub success: bool,
    pub status: (u16, u16),
    pub time_ms: u64,
    pub body: Option<Value>,
    pub headers: HashMap<String, String>,
    pub errors: Vec<String>,
}

pub async fn run(
    client: &HttpClient,
    test: &Test,
    test_file_dir: &Path,
    vars: &mut HashMap<String, String>,
) -> ExecutionResult {
    let start = Instant::now();

    let headers = test
        .headers
        .as_ref()
        .map(|h| {
            h.iter()
                .map(|(k, v)| (k.clone(), replace_variables(v, vars)))
                .collect()
        })
        .unwrap_or_default();

    let params = test
        .query_params
        .as_ref()
        .map(|p| {
            p.iter()
                .map(|(k, v)| (k.clone(), replace_variables(v, vars)))
                .collect()
        })
        .unwrap_or_default();

    let body = if let Some(inline_body) = &test.body {
        Some(variables::replace_variables_in_json(inline_body, vars))
    } else if let Some(body_file) = &test.body_file {
        match load_body_from_file(body_file, test_file_dir, vars) {
            Ok(content) => Some(content),
            Err(e) => {
                eprintln!("Error loading body file '{}': {}", body_file, e);
                None
            }
        }
    } else {
        None
    };

    let request = RequestData {
        method: test.method.clone(),
        url: replace_variables(&test.endpoint, vars),
        headers,
        params,
        body,
    };

    debug!(
        "Request for '{}': headers = {:?}, body = {:?}",
        test.name, request.headers, request.body
    );

    match client.execute(request).await {
        Ok((status, body, mut headers)) => {
            let time_ms = start.elapsed().as_millis() as u64;

            let header_keys: Vec<String> = headers.keys().cloned().collect();
            for key in header_keys {
                if let Some(value) = headers.get(&key) {
                    headers.insert(key.to_lowercase(), value.clone());
                }
            }

            debug!("Response headers: {:?}", headers);

            if let Some(cookie_map) = &test.get_cookie {
                debug!("Attempting to extract cookies: {:?}", cookie_map);
                variables::store_variables(
                    &body,
                    &HashMap::new(),
                    &headers,
                    &Some(cookie_map.clone()),
                    time_ms,
                    vars,
                );
                debug!("Variables after cookie extraction: {:?}", vars);
            }

            let validation = verify::check(test, status, &body, time_ms, vars);

            if validation.ok && test.store.is_some() {
                variables::store_variables(
                    &body,
                    test.store.as_ref().unwrap(),
                    &headers,
                    &None,
                    time_ms,
                    vars,
                );
            }

            ExecutionResult {
                success: validation.ok,
                status: (test.expected_status, status),
                time_ms,
                body: Some(body),
                headers,
                errors: validation.errors,
            }
        }
        Err(err) => ExecutionResult {
            success: false,
            status: (test.expected_status, 0),
            time_ms: 0,
            body: None,
            headers: HashMap::new(),
            errors: vec![err.to_string()],
        },
    }
}
