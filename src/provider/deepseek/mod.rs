//! DeepSeek provider implementation
//!
//! This module provides the DeepSeek provider with API type definitions and conversions.

mod convert;
pub mod types;

pub use types::*;

use crate::provider::call_chat_completions;

/// DeepSeek provider struct
pub struct DeepseekProvider {
    api_key: String,
    model: String,
    base_url: String,
    max_context_length: Option<usize>,
}

impl DeepseekProvider {
    /// Create a new DeepSeek provider
    pub fn new(api_key: String, model: String, max_context_length: Option<usize>) -> Self {
        Self {
            api_key,
            model,
            base_url: "https://api.deepseek.com".to_string(),
            max_context_length,
        }
    }

    /// Create a new DeepSeek provider with custom base URL
    pub fn new_with_base_url(api_key: String, model: String, base_url: String) -> Self {
        Self {
            api_key,
            model,
            base_url,
            max_context_length: None,
        }
    }
}

#[async_trait::async_trait]
impl crate::provider::Provider for DeepseekProvider {
    fn name(&self) -> &str {
        "deepseek"
    }

    async fn chat(
        &self,
        mut request: crate::provider::types::ChatRequest,
    ) -> anyhow::Result<crate::provider::types::ChatResponse> {
        request.model = Some(self.model.clone());
        request.parallel_tool_calls = Some(true);
        let url = format!("{}/chat/completions", self.base_url);
        call_chat_completions::<ChatRequest, ChatResponse>(&url, &self.api_key, request).await
    }

    async fn embedding(
        &self,
        _request: crate::provider::types::EmbeddingRequest,
    ) -> anyhow::Result<crate::provider::types::EmbeddingResponse> {
        Err(anyhow::anyhow!("DeepSeek does not support embeddings API"))
    }

    fn capabilities(&self) -> crate::provider::types::ProviderCapabilities {
        crate::provider::types::ProviderCapabilities::CHAT
            | crate::provider::types::ProviderCapabilities::FUNCTION_CALLING
            | crate::provider::types::ProviderCapabilities::STREAMING
    }

    fn max_context_length(&self) -> usize {
        // Use configured max_context_length if provided, otherwise use default
        self.max_context_length.unwrap_or(131072) // 128K tokens default
    }
}
