use crate::provider::{Provider, Request, Response};

pub struct OpenaiCompatibleProvider {
    name: String,
    model: String,
    endpoint: String,
    api_key: String,
}

impl OpenaiCompatibleProvider {
    pub fn new(name: String, model: String, endpoint: String, api_key: String) -> Self {
        Self {
            name,
            model,
            endpoint,
            api_key,
        }
    }
}

#[async_trait::async_trait]
impl Provider for OpenaiCompatibleProvider {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    async fn chat(&self, request: &Request) -> anyhow::Result<Response> {
        let url = format!("{}/chat/completions", self.endpoint);
        println!("request:{}", serde_json::to_string(&request)?);
        let response = reqwest::Client::new()
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
