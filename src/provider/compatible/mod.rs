//! OpenAI-compatible provider implementation
//!
//! This module provides the OpenAI-compatible provider with API type definitions and conversions.

mod convert;
pub mod types;

pub use types::*;

use crate::provider::call_chat_completions;

/// OpenAI-compatible provider
pub struct OpenaiCompatibleProvider {
    name: String,
    model: String,
    endpoint: String,
    api_key: String,
    max_context_length: usize,
}

impl OpenaiCompatibleProvider {
    /// Create a new OpenAI-compatible provider
    pub fn new(
        name: impl Into<String>,
        model: impl Into<String>,
        endpoint: impl Into<String>,
        api_key: impl Into<String>,
        max_context_length: usize,
    ) -> Self {
        Self {
            name: name.into(),
            model: model.into(),
            endpoint: endpoint.into(),
            api_key: api_key.into(),
            max_context_length,
        }
    }
}

#[async_trait::async_trait]
impl crate::provider::Provider for OpenaiCompatibleProvider {
    fn name(&self) -> &str {
        &self.name
    }

    async fn chat(
        &self,
        mut request: crate::provider::types::ChatRequest,
    ) -> anyhow::Result<crate::provider::types::ChatResponse> {
        let url = format!("{}/chat/completions", self.endpoint);
        request.model = Some(self.model.clone());
        request.parallel_tool_calls = Some(true);
        call_chat_completions::<ChatCompletionRequest, ChatCompletionResponse>(
            &url,
            &self.api_key,
            request,
        )
        .await
    }

    async fn embedding(
        &self,
        _request: crate::provider::types::EmbeddingRequest,
    ) -> anyhow::Result<crate::provider::types::EmbeddingResponse> {
        anyhow::bail!("OpenaiCompatibleProvider does not support embeddings")
    }

    fn capabilities(&self) -> crate::provider::types::ProviderCapabilities {
        crate::provider::types::ProviderCapabilities::CHAT
    }

    fn max_context_length(&self) -> usize {
        self.max_context_length
    }
}
