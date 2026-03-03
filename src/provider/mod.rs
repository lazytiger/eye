//! Model module
//!
//! Manages LLM provider providers, including:
//! - ModelProvider trait definition
//! - OpenRouter implementation
//! - Message and tool call type definitions

pub mod openai;
pub mod types;

pub use self::types::*;

