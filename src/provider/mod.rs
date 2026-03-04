//! Model module
//!
//! Manages LLM provider providers, including:
//! - ModelProvider trait definition
//! - OpenRouter implementation
//! - Message and tool call type definitions

pub mod compatible;
pub mod types;

pub use self::types::*;

#[async_trait::async_trait]
pub trait Provider {
    fn name(&self) -> &str;

    async fn chat(&self, request: Request) -> anyhow::Result<Response>;
}
