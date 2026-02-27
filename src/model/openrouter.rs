//! OpenRouter model provider implementation
//!
//! Implements OpenRouter client using openai-api-rs crate

use super::r#trait::*;
use crate::config::settings::OpenRouterConfig;
use anyhow::Result;

/// OpenRouter model provider
pub struct OpenRouterProvider {
    /// Configuration
    config: OpenRouterConfig,
}

impl OpenRouterProvider {
    /// Create new OpenRouter provider
    pub fn new(config: OpenRouterConfig) -> Result<Self> {
        if config.api_key.is_empty() {
            return Err(anyhow::anyhow!("OpenRouter API Key cannot be empty"));
        }

        Ok(Self { config })
    }
}

#[async_trait::async_trait]
impl ModelProvider for OpenRouterProvider {
    async fn chat_completion(
        &self,
        _request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse> {
        // For now, return a mock response
        // TODO: Implement actual OpenRouter API integration

        let mock_message = ChatMessage {
            role: MessageRole::Assistant,
            content: "This is a mock response. OpenRouter integration is not yet implemented."
                .to_string(),
            tool_calls: None,
        };

        let mock_usage = Usage {
            prompt_tokens: 10,
            completion_tokens: 20,
            total_tokens: 30,
        };

        Ok(ChatCompletionResponse {
            message: mock_message,
            usage: Some(mock_usage),
        })
    }

    fn name(&self) -> &str {
        "openrouter"
    }

    fn supported_models(&self) -> Vec<String> {
        vec![
            "openai/gpt-4o".to_string(),
            "openai/gpt-4o-mini".to_string(),
            "anthropic/claude-3.5-sonnet".to_string(),
            "google/gemini-2.0-flash-exp".to_string(),
            "meta-llama/llama-3.3-70b-instruct".to_string(),
        ]
    }

    fn validate_config(&self) -> Result<()> {
        if self.config.api_key.is_empty() {
            return Err(anyhow::anyhow!("OpenRouter API Key is not set"));
        }

        if self.config.endpoint.is_empty() {
            return Err(anyhow::anyhow!("OpenRouter endpoint is not set"));
        }

        Ok(())
    }
}
