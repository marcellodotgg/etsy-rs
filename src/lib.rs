use reqwest::{Body, Error, Response};

pub mod other;
pub mod user;

const BASE_URL: &str = "https://openapi.etsy.com";

pub struct EtsyClient {
    client: reqwest::Client,
    api_key: String,
    pub base_url: String,
}

impl EtsyClient {
    pub fn new(api_key: &str) -> Self {
        EtsyClient {
            client: reqwest::Client::new(),
            api_key: api_key.to_string(),
            base_url: format!("{}/{}/{}", BASE_URL, "v3", "application"),
        }
    }

    pub async fn get(&self, endpoint: &str) -> Result<Response, Error> {
        self.client
            .get(self.get_endpoint_url(endpoint))
            .header("x-api-key", &self.api_key)
            .send()
            .await
    }

    pub async fn put(&self, endpoint: &str, body: Option<Body>) -> Result<Response, Error> {
        // TODO, need to handle option gracefully
        self.client
            .put(self.get_endpoint_url(endpoint))
            .header("x-api-key", &self.api_key)
            .body(body.unwrap())
            .send()
            .await
    }

    pub async fn post(&self, endpoint: &str, body: Option<Body>) -> Result<Response, Error> {
        // TODO, need to handle option gracefully
        self.client
            .post(self.get_endpoint_url(endpoint))
            .header("x-api-key", &self.api_key)
            .body(body.unwrap())
            .send()
            .await
    }

    fn get_endpoint_url(&self, endpoint: &str) -> String {
        format!("{}/{}", self.base_url, endpoint)
    }
}
