//! OpenRouter provider implementation
//!
//! This module provides the OpenRouter provider with API type definitions and conversions.

mod chat_types;
mod client;
mod convert;
mod embedding_types;

pub use chat_types::*;
pub use embedding_types::*;

use crate::provider::{call_chat_completions, call_embedding};

/// OpenRouter provider struct
pub struct OpenrouterProvider {
    api_key: String,
    model: String,
    base_url: String,
}

impl OpenrouterProvider {
    /// Create a new OpenRouter provider
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            base_url: "https://openrouter.ai/api/v1".to_string(),
        }
    }

    /// Create a new OpenRouter provider with custom base URL
    pub fn new_with_base_url(api_key: String, model: String, base_url: String) -> Self {
        Self {
            api_key,
            model,
            base_url,
        }
    }
}

#[async_trait::async_trait]
impl crate::provider::Provider for OpenrouterProvider {
    fn name(&self) -> &str {
        "openrouter"
    }

    async fn chat(
        &self,
        mut request: crate::provider::types::ChatRequest,
    ) -> anyhow::Result<crate::provider::types::ChatResponse> {
        request.model = Some(self.model.clone());
        let url = format!("{}/chat/completions", self.base_url);
        call_chat_completions::<ChatRequest, ChatResponse>(
            &url,
            &self.api_key,
            request,
        )
        .await
    }

    async fn embedding(
        &self,
        request: crate::provider::types::EmbeddingRequest,
    ) -> anyhow::Result<crate::provider::types::EmbeddingResponse> {
        let url = format!("{}/embeddings", self.base_url);
        call_embedding::<EmbeddingsRequest, EmbeddingsResponse>(
            &url,
            &self.api_key,
            request,
        )
        .await
    }

    fn capabilities(&self) -> crate::provider::types::ProviderCapabilities {
        let mut capabilities = crate::provider::types::ProviderCapabilities::CHAT
            | crate::provider::types::ProviderCapabilities::STREAMING;

        let model_lower = self.model.to_lowercase();
        capabilities |= crate::provider::types::ProviderCapabilities::FUNCTION_CALLING;

        if model_lower.contains("vision") || model_lower.contains("gpt-4")
            || model_lower.contains("claude-3") || model_lower.contains("gemini") {
            capabilities |= crate::provider::types::ProviderCapabilities::VISION;
        }

        if model_lower.contains("json") {
            capabilities |= crate::provider::types::ProviderCapabilities::JSON_MODE;
        }

        capabilities
    }

    fn max_context_length(&self) -> usize {
        let model_lower = self.model.to_lowercase();

        if model_lower.contains("claude-3") {
            if model_lower.contains("opus") || model_lower.contains("sonnet") {
                200000
            } else {
                100000
            }
        } else if model_lower.contains("gpt-4") {
            if model_lower.contains("turbo") {
                128000
            } else if model_lower.contains("32k") {
                32768
            } else {
                8192
            }
        } else if model_lower.contains("gpt-3.5") {
            if model_lower.contains("16k") {
                16384
            } else {
                4096
            }
        } else {
            8192
        }
    }
}
