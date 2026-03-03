use crate::provider::{Provider, Request, Response};
use reqwest::Client;
use std::sync::Arc;

pub struct OpenaiCompatibleProvider {
    name: String,
    model: String,
    endpoint: String,
    api_key: String,
    client: Arc<Client>,
}

impl OpenaiCompatibleProvider {
    pub fn new(
        name: String,
        model: String,
        endpoint: String,
        api_key: String,
        client: Arc<Client>,
    ) -> Self {
        Self {
            name,
            model,
            endpoint,
            api_key,
            client,
        }
    }
}

#[async_trait::async_trait]
impl Provider for OpenaiCompatibleProvider {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    async fn chat(&self, mut request: Request) -> anyhow::Result<Response> {
        request.model = Some(self.model.clone());
        let url = format!("{}/chat/completions", self.endpoint);
        println!("request:{}", serde_json::to_string(&request)?);
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;
        println!("waiting for response");
        if response.status().is_success() {
            let response = response.json::<Response>().await?;
            Ok(response)
        } else {
            let text = response.text().await?;
            anyhow::bail!(text)
        }
    }
}
