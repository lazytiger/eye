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
use anyhow::Result;

#[async_trait::async_trait]
pub trait Provider: Send + Sync {
    /// Returns the unique identifier/name of the provider (e.g., "openai", "openrouter").
    fn name(&self) -> &str;

    /// Sends a chat completion request and returns the full response.
    /// Used for non-streaming interactions.
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse>;

    /// Generates embeddings for the given input text.
    /// Essential for RAG (Retrieval-Augmented Generation) features.
    async fn embedding(&self, request: EmbeddingRequest) -> Result<EmbeddingResponse>;

    /// Returns the capabilities of the currently configured model (e.g., vision, function calling).
    /// This helps the client know what features are available without try-and-error.
    fn capabilities(&self) -> ModelCapabilities;

    /// Returns the maximum context length (in tokens) for the currently configured model.
    /// Used for context window management to avoid overflow errors.
    fn max_context_length(&self) -> usize;
}
