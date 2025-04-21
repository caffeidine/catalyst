#[cfg(test)]
mod tests {
    use catalyst::http::client::{HttpClient, RequestData};
    use catalyst::models::config::Config;

    fn create_test_config() -> Config {
        Config {
            base_url: "https://httpbin.org".to_string(),
            auth_method: None,
            auth_token: None,
            default_headers: None,
            env: None,
        }
    }

    #[tokio::test]
    async fn test_get_request() {
        let config = create_test_config();
        let client = HttpClient::new(&config);
        let data = RequestData {
            method: "GET".to_string(),
            url: "/get".to_string(),
            headers: vec![],
            params: vec![],
            body: None,
        };

        let (status, _, _) = client.execute(data).await.unwrap();
        assert_eq!(status, 200);
    }

    #[tokio::test]
    async fn test_post_request() {
        let config = create_test_config();
        let client = HttpClient::new(&config);
        let data = RequestData {
            method: "POST".to_string(),
            url: "/post".to_string(),
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
            params: vec![],
            body: Some(serde_json::json!({"test": "value"})),
        };

        let (status, _, _) = client.execute(data).await.unwrap();
        assert_eq!(status, 200);
    }

    #[tokio::test]
    async fn test_with_params() {
        let config = create_test_config();
        let client = HttpClient::new(&config);
        let data = RequestData {
            method: "GET".to_string(),
            url: "/get".to_string(),
            headers: vec![],
            params: vec![("key".to_string(), "value".to_string())],
            body: None,
        };

        let (status, body, _) = client.execute(data).await.unwrap();
        assert_eq!(status, 200);

        if let Some(args) = body.get("args") {
            assert_eq!(args.get("key").unwrap(), "value");
        }
    }
}
