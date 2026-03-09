//! OpenRouter API client
//!
//! This module provides the OpenRouter client implementation.

use super::chat_types::*;
use super::embedding_types::*;
use anyhow::{Result, anyhow};
use crate::utils;

/// OpenRouter API client configuration
#[derive(Clone, Debug)]
pub struct OpenRouterConfig {
    pub api_key: String,
    pub base_url: String,
    pub timeout_secs: Option<u64>,
}

impl Default for OpenRouterConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://openrouter.ai/api/v1".to_string(),
            timeout_secs: Some(30),
        }
    }
}

impl OpenRouterConfig {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            ..Default::default()
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = Some(timeout_secs);
        self
    }
}

/// OpenRouter API client
#[derive(Clone)]
pub struct OpenRouterClient {
    config: OpenRouterConfig,
}

impl OpenRouterClient {
    pub fn new(config: OpenRouterConfig) -> Result<Self> {
        Ok(Self {
            config,
        })
    }

    pub fn with_api_key(api_key: impl Into<String>) -> Result<Self> {
        let config = OpenRouterConfig::new(api_key);
        Self::new(config)
    }

    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Result<Self> {
        let config = OpenRouterConfig::new(api_key).with_base_url(base_url);
        Self::new(config)
    }

    pub fn api_key(&self) -> &str {
        &self.config.api_key
    }

    pub fn base_url(&self) -> &str {
        &self.config.base_url
    }

    fn build_url(&self, path: &str) -> String {
        format!("{}{}", self.config.base_url, path)
    }

    pub async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let url = self.build_url("/chat/completions");

        let response = utils::reqwest_client()
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key()))
            .header("Content-Type", "application/json")
            .header("HTTP-Referer", "https://github.com/eye/eye")
            .header("X-Title", "Eye Assistant")
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json::<ChatResponse>().await?)
        } else {
            let error = response.text().await?;
            Err(anyhow!("Chat completion failed: {}", error))
        }
    }

    pub async fn embeddings(&self, request: EmbeddingsRequest) -> Result<EmbeddingsResponse> {
        let url = self.build_url("/embeddings");

        let response = utils::reqwest_client()
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key()))
            .header("Content-Type", "application/json")
            .header("HTTP-Referer", "https://github.com/eye/eye")
            .header("X-Title", "Eye Assistant")
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json::<EmbeddingsResponse>().await?)
        } else {
            let error = response.text().await?;
            Err(anyhow!("Embeddings request failed: {}", error))
        }
    }
}

/// Helper function to create a new OpenRouter client
pub fn create_client(api_key: impl Into<String>) -> Result<OpenRouterClient> {
    OpenRouterClient::with_api_key(api_key)
}

/// Helper function to create a new OpenRouter client with custom base URL
pub fn create_client_with_base_url(
    api_key: impl Into<String>,
    base_url: impl Into<String>,
) -> Result<OpenRouterClient> {
    OpenRouterClient::with_base_url(api_key, base_url)
}
