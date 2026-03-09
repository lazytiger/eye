//! OpenRouter API embedding types
//!
//! This module contains schema definitions for OpenRouter embedding requests and responses.

use serde::{Deserialize, Serialize};

use crate::provider::openrouter::chat_types::ProviderPreferences;

/// Embedding input content item for multimodal embeddings
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EmbeddingContentItem {
    Text { text: String },
    ImageUrl { image_url: EmbeddingImageUrl },
}

/// Image URL for embedding content
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingImageUrl {
    pub url: String,
}

/// Embedding input type
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum EmbeddingInput {
    String(String),
    StringArray(Vec<String>),
    NumberArray(Vec<f64>),
    NumberArray2D(Vec<Vec<f64>>),
    ContentArray(Vec<EmbeddingContentItem>),
}

/// Embeddings request
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingsRequest {
    pub input: EmbeddingInput,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<EmbeddingEncodingFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<ProviderPreferences>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
}

/// Embedding encoding format
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddingEncodingFormat {
    Float,
    Base64,
}

/// Embeddings response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub object: String,
    pub data: Vec<EmbeddingObject>,
    pub model: String,
    pub usage: EmbeddingUsage,
}

/// Embedding object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingObject {
    pub object: String,
    pub embedding: EmbeddingData,
    pub index: u32,
}

/// Embedding data (array or base64 string)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum EmbeddingData {
    Array(Vec<f32>),
    Base64(String),
}

/// Embedding usage
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingUsage {
    pub prompt_tokens: u32,
    pub total_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
}
