

//!
//! This module defines common types used across all model providers,
//! including chat requests/responses, embedding requests/responses,
//! and model capabilities.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Chat completion request message role
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

/// Chat completion request message
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    /// Role of the message author
    pub role: Role,
    /// Content of the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Optional name for the participant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tool calls made by the assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    /// Tool call ID (required when role is tool)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

/// Tool definition
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tool {
    /// Type of the tool
    #[serde(rename = "type")]
    pub tool_type: String,
    /// Function definition
    pub function: FunctionDefinition,
}

/// Function definition
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionDefinition {
    /// Name of the function
    pub name: String,
    /// Description of the function
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Parameters of the function (JSON Schema)
    pub parameters: serde_json::Value,
    /// Whether to enforce strict parameter validation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// Tool call in a chat completion message
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {
    /// ID of the tool call
    pub id: String,
    /// Type of the tool
    #[serde(rename = "type")]
    pub tool_type: String,
    /// Function call details
    pub function: ToolCallFunction,
}

/// Function call in a tool call
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCallFunction {
    /// Name of the function to call
    pub name: String,
    /// Arguments to call the function with, as JSON string
    pub arguments: String,
}

/// Tool choice option
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ToolChoice {
    /// Automatic tool choice
    String(String),
    /// Named tool choice
    Object(NamedToolChoice),
}

/// Named tool choice
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NamedToolChoice {
    /// Type of the tool choice
    #[serde(rename = "type")]
    pub tool_type: String,
    /// Function to call
    pub function: NamedToolChoiceFunction,
}

/// Named tool choice function
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NamedToolChoiceFunction {
    /// Name of the function to call
    pub name: String,
}

/// Response format type
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormatType {
    Text,
    JsonObject,
    JsonSchema,
}

/// Response format specification
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseFormat {
    /// Text response format
    Text,
    /// JSON object response format
    JsonObject,
    /// JSON schema response format
    JsonSchema {
        /// JSON schema definition
        json_schema: JsonSchemaFormat,
    },
}

/// JSON schema format
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonSchemaFormat {
    /// Name of the JSON schema
    pub name: String,
    /// Description of the JSON schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// JSON schema definition
    pub schema: serde_json::Value,
    /// Whether to enforce strict schema validation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// Stop configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Stop {
    /// Single stop sequence
    Single(String),
    /// Multiple stop sequences
    Multiple(Vec<String>),
}

/// Stream options
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamOptions {
    /// Whether to include usage in stream
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_usage: Option<bool>,
}

/// Unified chat request
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatRequest {
    /// A list of messages comprising the conversation so far
    pub messages: Vec<ChatMessage>,
    /// Model ID used to generate the response
    pub model: String,
    /// Temperature sampling parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Top-p sampling parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Whether to stream the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// List of tools the model may call
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// Tool choice option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    /// Maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    /// Number of chat completion choices to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    /// Stop sequences
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Stop>,
    /// Frequency penalty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    /// Presence penalty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    /// Logit bias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i32>>,
    /// Whether to return log probabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    /// Number of most likely tokens to return at each position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<i32>,
    /// Random seed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Unique identifier for the end-user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Response format specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    /// Whether to enable parallel tool calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    /// Stream options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,
}

/// Finish reason
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum FinishReason {
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "tool_calls")]
    ToolCalls,
    #[serde(rename = "content_filter")]
    ContentFilter,
    #[serde(rename = "function_call")]
    FunctionCall,
}

/// Log probability content
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogprobContent {
    /// Token
    pub token: String,
    /// Log probability
    pub logprob: f64,
    /// Bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<i32>>,
    /// Top log probabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<Vec<TopLogprob>>,
}

/// Top log probability
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopLogprob {
    /// Token
    pub token: String,
    /// Log probability
    pub logprob: f64,
    /// Bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<i32>>,
}

/// Log probabilities
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Logprobs {
    /// Content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<LogprobContent>>,
}

/// Chat completion choice
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatChoice {
    /// Index of the choice
    pub index: i32,
    /// Message generated by the model
    pub message: ChatMessage,
    /// Reason the model stopped generating tokens
    pub finish_reason: FinishReason,
    /// Log probabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<Logprobs>,
}

/// Token usage statistics
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    /// Number of tokens in the prompt
    pub prompt_tokens: i32,
    /// Number of tokens in the completion
    pub completion_tokens: i32,
    /// Total number of tokens
    pub total_tokens: i32,
}

/// Unified chat response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatResponse {
    /// Unique identifier for the chat completion
    pub id: String,
    /// Object type
    pub object: String,
    /// Unix timestamp of creation
    pub created: i64,
    /// Model used for completion
    pub model: String,
    /// List of chat completion choices
    pub choices: Vec<ChatChoice>,
    /// Token usage statistics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
    /// System fingerprint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
}

/// Embedding encoding format
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddingEncodingFormat {
    Float,
    Base64,
}

/// Unified embedding request
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingRequest {
    /// Input text to embed
    pub input: Vec<String>,
    /// Model ID used to generate embeddings
    pub model: String,
    /// Encoding format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<EmbeddingEncodingFormat>,
    /// Dimensions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<i32>,
    /// User identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Embedding object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingObject {
    /// Index of the embedding
    pub index: usize,
    /// The embedding vector
    pub embedding: Vec<f32>,
    /// Object type
    pub object: String,
}

/// Embedding usage statistics
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingUsage {
    /// Number of tokens in the prompt
    pub prompt_tokens: u32,
    /// Total number of tokens
    pub total_tokens: u32,
}

/// Unified embedding response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingResponse {
    /// List of embedding objects
    pub data: Vec<EmbeddingObject>,
    /// Model used for embedding
    pub model: String,
    /// Object type
    pub object: String,
    /// Token usage statistics
    pub usage: EmbeddingUsage,
}

bitflags::bitflags! {
    /// Model capabilities bitflags
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ModelCapabilities: u32 {
        const TEXT_GENERATION = 1 << 0;
        const FUNCTION_CALLING = 1 << 1;
        const VISION = 1 << 2;
        const AUDIO_INPUT = 1 << 3;
        const OBJECT_GENERATION = 1 << 4; // JSON mode
    }
}