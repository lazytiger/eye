use crate::provider::{Provider, Request, Response};
use crate::utils::{reqwest_client, user_agent};

pub struct OpenaiCompatibleProvider {
    name: String,
    model: String,
    endpoint: String,
    api_key: String,
}

impl OpenaiCompatibleProvider {
    pub fn new(
        name: impl Into<String>,
        model: impl Into<String>,
        endpoint: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            model: model.into(),
            endpoint: endpoint.into(),
            api_key: api_key.into(),
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
        tracing::debug!("request:{}", serde_json::to_string(&request)?);
        let response = reqwest_client()
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("User-Agent", user_agent())
            .json(&request)
            .send()
            .await?;
        if response.status().is_success() {
            let response = response.json::<Response>().await?;
            tracing::debug!("response:{}", serde_json::to_string(&response)?);
            Ok(response)
        } else {
            let text = response.text().await?;
            anyhow::bail!(text)
        }
    }
}
