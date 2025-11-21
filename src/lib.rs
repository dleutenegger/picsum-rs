pub mod api;

use std::sync::Arc;

static BASE_URL: &str = "https://picsum.photos";

#[derive(Debug, Clone, Default)]
pub struct PicsumClient {
    inner: Arc<PicsumClientInner>,
}

#[derive(Debug, Clone)]
struct PicsumClientInner {
    client: reqwest::Client,
    base_url: String,
}

impl Default for PicsumClientInner {
    fn default() -> Self {
        Self {
            client: reqwest::Client::default(),
            base_url: BASE_URL.to_string(),
        }
    }
}

impl PicsumClient {
    pub fn builder() -> PicsumClientBuilder {
        PicsumClientBuilder::new()
    }
}

#[derive(Debug, Clone)]
pub struct PicsumClientBuilder {
    client: Option<reqwest::Client>,
    base_url: String,
}

impl Default for PicsumClientBuilder {
    fn default() -> Self {
        Self {
            client: Some(reqwest::Client::default()),
            base_url: BASE_URL.to_string(),
        }
    }
}

impl PicsumClientBuilder {
    pub fn new() -> Self {
        Self {
            client: None,
            base_url: BASE_URL.to_string(),
        }
    }

    pub fn client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }

    pub fn build(&self) -> PicsumClient {
        let inner = PicsumClientInner {
            client: self.client.clone().unwrap_or_default(),
            base_url: self.base_url.clone(),
        };

        PicsumClient {
            inner: Arc::new(inner),
        }
    }
}
