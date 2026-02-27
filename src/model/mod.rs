//! Model module
//!
//! Manages LLM model providers, including:
//! - ModelProvider trait definition
//! - OpenRouter implementation
//! - Message and tool call type definitions

pub mod openrouter;
pub mod r#trait;

pub use self::{openrouter::OpenRouterProvider, r#trait::*};

use crate::config::settings::OpenRouterConfig;
use anyhow::Result;

/// Create a model provider
///
/// Create a model provider instance based on configuration
pub fn create_model_provider(config: &OpenRouterConfig) -> Result<Box<dyn ModelProvider>> {
    let provider = OpenRouterProvider::new(config.clone())?;
    Ok(Box::new(provider))
}
