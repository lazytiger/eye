//! Embedding types
//!
//! This module defines types for embedding requests and responses.

use serde::{Deserialize, Serialize};

/// Embedding request
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingRequest {
    /// Input text(s) to embed
    pub input: EmbeddingInput,
    /// Model to use
    pub model: String,
    /// Encoding format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<EmbeddingEncodingFormat>,
    /// Number of dimensions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<u32>,
    /// End user identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Embedding input - supports single or multiple inputs
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum EmbeddingInput {
    /// Single text input
    String(String),
    /// Multiple text inputs
    StringArray(Vec<String>),
}

/// Embedding encoding format
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddingEncodingFormat {
    Float,
    Base64,
}

/// Embedding response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingResponse {
    pub object: String,
    pub data: Vec<EmbeddingObject>,
    pub model: String,
    pub usage: EmbeddingUsage,
}

/// Embedding object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingObject {
    pub index: u32,
    pub embedding: Vec<f32>,
    pub object: String,
}

/// Embedding usage
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingUsage {
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}
