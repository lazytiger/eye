//! OpenAI provider implementation
//!
//! This module provides the OpenAI provider with API type definitions and conversions.

mod convert;
mod types;

pub use types::*;

use crate::provider::{call_chat_completions, call_embedding};

/// OpenAI provider struct
pub struct OpenaiProvider {
    api_key: String,
    model: String,
    base_url: String,
}

impl OpenaiProvider {
    /// Create a new OpenAI provider
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            base_url: "https://api.openai.com/v1".to_string(),
        }
    }

    /// Create a new OpenAI provider with custom base URL
    pub fn new_with_base_url(api_key: String, model: String, base_url: String) -> Self {
        Self {
            api_key,
            model,
            base_url,
        }
    }
}

#[async_trait::async_trait]
impl crate::provider::Provider for OpenaiProvider {
    fn name(&self) -> &str {
        "openai"
    }

    async fn chat(
        &self,
        mut request: crate::provider::types::ChatRequest,
    ) -> anyhow::Result<crate::provider::types::ChatResponse> {
        request.model = Some(self.model.clone());
        let url = format!("{}/chat/completions", self.base_url);
        call_chat_completions::<CreateChatCompletionRequest, CreateChatCompletionResponse>(
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
        call_embedding::<CreateEmbeddingRequest, CreateEmbeddingResponse>(
            &url,
            &self.api_key,
            request,
        )
        .await
    }

    fn capabilities(&self) -> crate::provider::types::ProviderCapabilities {
        let model_lower = self.model.to_lowercase();
        let mut capabilities = crate::provider::types::ProviderCapabilities::CHAT
            | crate::provider::types::ProviderCapabilities::STREAMING;

        if model_lower.contains("gpt-4") || model_lower.contains("gpt-3.5") {
            capabilities |= crate::provider::types::ProviderCapabilities::FUNCTION_CALLING;
        }

        if model_lower.contains("vision") || model_lower.contains("gpt-4") {
            capabilities |= crate::provider::types::ProviderCapabilities::VISION;
        }

        if model_lower.contains("whisper") || model_lower.contains("audio") {
            capabilities |= crate::provider::types::ProviderCapabilities::AUDIO_INPUT;
        }

        if model_lower.contains("json") {
            capabilities |= crate::provider::types::ProviderCapabilities::JSON_MODE;
        }

        capabilities
    }

    fn max_context_length(&self) -> usize {
        let model_lower = self.model.to_lowercase();

        if model_lower.contains("gpt-4") {
            if model_lower.contains("32k") {
                32768
            } else if model_lower.contains("128k") {
                131072
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
            4096
        }
    }
}
