//! Model module
//!
//! Manages LLM provider providers, including:
//! - ModelProvider trait definition
//! - OpenRouter implementation
//! - Message and tool call type definitions

pub mod compatible;
pub mod deepseek;
pub mod openai;
pub mod openrouter;
pub mod types;

pub use self::types::*;
use anyhow::{anyhow, Result};

#[async_trait::async_trait]
pub trait Provider: Send + Sync {
    /// Returns the unique identifier/name of the provider (e.g., "openai", "openrouter").
    fn name(&self) -> &str;

    /// Sends a chat completion request and returns the full response.
    /// Used for non-streaming interactions.
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse>;

    /// Generates embeddings for the given input text.
    /// Essential for RAG (Retrieval-Augmented Generation) features.
    async fn embedding(&self, _request: EmbeddingRequest) -> Result<EmbeddingResponse> {
        anyhow::bail!("Embedding not supported by provider {}", self.name());
    }

    /// Returns the capabilities of the currently configured model (e.g., vision, function calling).
    /// This helps the client know what features are available without try-and-error.
    fn capabilities(&self) -> ModelCapabilities;

    /// Returns the maximum context length (in tokens) for the currently configured model.
    /// Used for context window management to avoid overflow errors.
    fn max_context_length(&self) -> usize;
}

/// Factory function to create a Provider instance based on provider name.
///
/// # Arguments
/// * `provider_name` - The name of the provider. Can be:
///   - Standard names: "openai", "openrouter", "deepseek"
///   - Custom endpoint: "name:https://endpoint.com/path"
/// * `model_name` - The model name to use
/// * `api_key` - The API key (can be overridden by environment variable)
///
/// # Returns
/// A boxed Provider trait object
///
/// # API Key Priority
/// 1. Environment variable: `PROVIDER_API_KEY` (where PROVIDER is uppercase provider name)
/// 2. The api_key parameter passed to this function
///
/// # Errors
/// Returns an error if the provider name is not recognized
pub fn create_provider(
    provider_name: &str,
    model_name: &str,
    api_key: &str,
) -> Result<Box<dyn Provider>> {
    // Check if provider_name contains ":" indicating custom endpoint format
    if provider_name.contains(':') {
        return create_compatible_provider(provider_name, model_name, api_key);
    }

    // Get API key with environment variable priority
    let api_key = get_api_key_with_env_priority(provider_name, api_key);

    match provider_name.to_lowercase().as_str() {
        "openai" => Ok(Box::new(openai::OpenaiProvider::new(
            api_key,
            model_name.to_string(),
        ))),
        "openrouter" => Ok(Box::new(openrouter::OpenrouterProvider::new(
            api_key,
            model_name.to_string(),
        ))),
        "deepseek" => Ok(Box::new(deepseek::DeepseekProvider::new(
            api_key,
            model_name.to_string(),
        ))),
        _ => Err(anyhow!(
            "Unknown provider: {}. Supported providers: openai, openrouter, deepseek, or custom format 'name:endpoint'",
            provider_name
        )),
    }
}

/// Creates a compatible provider from a name:endpoint format string.
/// Format: "name:https://api.example.com/v1"
fn create_compatible_provider(
    provider_spec: &str,
    model_name: &str,
    api_key: &str,
) -> Result<Box<dyn Provider>> {
    let parts: Vec<&str> = provider_spec.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(anyhow!(
            "Invalid provider format: {}. Expected format: 'name:https://endpoint'",
            provider_spec
        ));
    }

    let name = parts[0];
    let endpoint = parts[1];

    // Validate endpoint URL
    if !endpoint.starts_with("http://") && !endpoint.starts_with("https://") {
        return Err(anyhow!(
            "Invalid endpoint URL: {}. Must start with http:// or https://",
            endpoint
        ));
    }

    // Get API key with environment variable priority
    let api_key = get_api_key_with_env_priority(name, api_key);

    // For compatible provider, we use a reasonable default for max_context_length
    // This could be made configurable in the future
    const DEFAULT_MAX_CONTEXT_LENGTH: usize = 4096;

    Ok(Box::new(compatible::OpenaiCompatibleProvider::new(
        name,
        model_name,
        endpoint,
        api_key,
        DEFAULT_MAX_CONTEXT_LENGTH,
    )))
}

/// Gets API key with environment variable priority.
/// Environment variable name format: PROVIDER_API_KEY (uppercase provider name)
fn get_api_key_with_env_priority(provider_name: &str, default_api_key: &str) -> String {
    let env_var_name = format!("{}_API_KEY", provider_name.to_uppercase());

    std::env::var(&env_var_name)
        .ok()
        .filter(|key| !key.is_empty())
        .unwrap_or_else(|| default_api_key.to_string())
}
