pub mod endpoints;

use std::sync::Arc;

static BASE_URL: &str = "https://picsum.photos";

#[derive(Debug, Clone)]
pub struct PicsumClient {
    inner: Arc<PicsumClientInner>,
}

#[derive(Debug)]
struct PicsumClientInner {
    client: reqwest::Client,
}

impl PicsumClient {}

#[derive(Default)]
pub struct PicsumClientBuilder {
    client: Option<reqwest::Client>,
}

impl PicsumClientBuilder {
    pub fn new() -> Self {
        Self { client: None }
    }

    pub fn client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn build(&self) -> PicsumClient {
        let inner = PicsumClientInner {
            client: self.client.clone().unwrap_or_default(),
        };

        PicsumClient {
            inner: Arc::new(inner),
        }
    }
}
