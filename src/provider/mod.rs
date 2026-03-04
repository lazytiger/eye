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
use crate::provider::compatible::OpenaiCompatibleProvider;
use anyhow::Context;
use std::env;

#[async_trait::async_trait]
pub trait Provider {
    fn name(&self) -> &str;

    async fn chat(&self, request: Request) -> anyhow::Result<Response>;
}

pub fn create_openai_compatible(
    name: impl Into<String>,
    api_key: impl Into<String>,
    model: impl Into<String>,
) -> anyhow::Result<OpenaiCompatibleProvider> {
    let name = name.into();
    let mut api_key = api_key.into();
    match name.as_str() {
        "deepseek" => {
            if api_key.is_empty() {
                api_key = env::var("DEEPSEEK_API_KEY")
                    .context("DEEPSEEK_API_KEY env var is not set")?
                    .into();
            }
            Ok(OpenaiCompatibleProvider::new(
                name,
                model.into(),
                "https://api.deepseek.com/",
                api_key,
            ))
        }
        "openai" => {
            if api_key.is_empty() {
                api_key = env::var("OPENAI_API_KEY")
                    .context("OPENAI_API_KEY env var is not set")?
                    .into();
            }
            Ok(OpenaiCompatibleProvider::new(
                name,
                model.into(),
                "https://api.openai.com/v1",
                api_key,
            ))
        }
        "openrouter" => {
            if api_key.is_empty() {
                api_key = env::var("OPENROUTER_API_KEY")
                    .context("OPENROUTER_API_KEY env var is not set")?
                    .into();
            }
            Ok(OpenaiCompatibleProvider::new(
                name,
                model.into(),
                "https://openrouter.ai/api/v1",
                api_key,
            ))
        }
        _ => anyhow::bail!("Unsupported provider name: {}", name),
    }
}
