//! Provider-agnostic types for chat and embeddings
//!
//! This module defines unified abstract types for chat completions and embeddings
//! that work across all major LLM providers (OpenAI, Anthropic, Google, xAI, DeepSeek, etc.)
//!
//! These types serve as the common interface layer - provider-specific implementations
//! will convert from/to these types.

mod capabilities;
mod chat;
mod embedding;
mod helpers;

pub use capabilities::*;
pub use chat::*;
pub use embedding::*;
pub use helpers::*;
