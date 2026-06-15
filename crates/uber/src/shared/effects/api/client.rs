use reqwest::Client;

#[derive(Debug, Clone)]
pub struct ApiClient {
    pub inner: Client,
    pub base_url: String,
    pub auth_token: Option<String>,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            inner: Client::new(),
            base_url,
            auth_token: None,
        }
    }
}
