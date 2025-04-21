use super::{variables, verify};
use crate::http::client::{HttpClient, RequestData};
use crate::models::test::Test;
use crate::utils::string::replace_variables;
use serde_json::Value;
use std::collections::HashMap;
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
    vars: &HashMap<String, String>,
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

    let request = RequestData {
        method: test.method.clone(),
        url: replace_variables(&test.endpoint, vars),
        headers,
        params,
        body: test
            .body
            .as_ref()
            .map(|b| variables::replace_variables_in_json(b, vars)),
    };

    match client.execute(request).await {
        Ok((status, body, headers)) => {
            let time_ms = start.elapsed().as_millis() as u64;

            if let Some(store) = &test.store {
                variables::store_variables(
                    &body,
                    store,
                    &headers,
                    &test.get_cookie,
                    time_ms,
                    &mut HashMap::new(),
                );
            }

            let validation = verify::check(test, status, &body, time_ms, vars);

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
