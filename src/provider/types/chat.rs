//! Chat completion types
//!
//! This module defines types for chat completion requests and responses.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Chat completion request
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ChatRequest {
    /// Messages in the conversation
    pub messages: Vec<ChatMessage>,
    /// Model to use (e.g., "gpt-4", "claude-3-opus", "gemini-pro")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Temperature (0-2). Higher values = more random
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Top-p (nucleus sampling). 0-1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Maximum tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Enable streaming response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Stop sequences
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Stop>,
    /// Frequency penalty (-2 to 2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    /// Presence penalty (-2 to 2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    /// Random seed for reproducibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Tools available for function calling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// Tool choice strategy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    /// Response format specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    /// Number of completions to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    /// Logit bias adjustments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, f32>>,
    /// Return log probabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    /// Number of top logprobs to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,
    /// End user identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Parallel tool calls enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
}

/// Chat message with role-based discrimination
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum ChatMessage {
    System(SystemMessage),
    User(UserMessage),
    Assistant(AssistantMessage),
    Tool(ToolMessage),
}

/// System message
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemMessage {
    pub content: MessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// User message - supports multimodal content
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserMessage {
    pub content: MessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Assistant message
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssistantMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<MessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
}

/// Tool response message
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolMessage {
    pub content: MessageContent,
    pub tool_call_id: String,
}

/// Message content - supports text and multimodal parts
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MessageContent {
    /// Simple text content
    Text(String),
    /// Multimodal content parts
    Parts(Vec<ContentPart>),
}

/// Content part for multimodal messages
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {
    /// Text content
    Text { text: String },
    /// Image content
    ImageUrl { image_url: ImageUrl },
    /// Audio content (data URI or URL)
    InputAudio { input_audio: InputAudio },
    /// Video content (data URI or URL)
    InputVideo { input_video: InputVideo },
    /// Document content (PDF, etc.)
    Document { document: Document },
}

/// Image URL 信息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageUrl {
    /// URL of the image (supports data: URIs and HTTP URLs)
    pub url: String,
    /// Detail level for vision models
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<ImageDetail>,
}

/// Image detail level
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ImageDetail {
    Low,
    High,
    Auto,
}

/// Audio input information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputAudio {
    /// Base64 encoded audio data or URL
    pub data: String,
    /// Audio format
    pub format: AudioFormat,
}

/// Audio format
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AudioFormat {
    Wav,
    Mp3,
    Flac,
    Opus,
    Pcm16,
}

/// Video input information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputVideo {
    /// Base64 encoded video data or URL
    pub data: String,
    /// Video format
    pub format: VideoFormat,
}

/// Video format
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum VideoFormat {
    Mp4,
    Webm,
    Mov,
    Avi,
}

/// Document information (for PDF, etc.)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Document {
    /// Base64 encoded document data or URL
    pub data: String,
    /// Document type
    pub file_type: DocumentType,
}

/// Document type
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DocumentType {
    Pdf,
    Txt,
    Md,
    Html,
}

/// Tool definition for function calling
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tool {
    #[serde(rename = "type")]
    pub type_: ToolType,
    pub function: FunctionDefinition,
}

/// Tool type
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ToolType {
    Function,
}

/// Function definition
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// JSON Schema parameters
    pub parameters: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// Tool call in assistant message
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: ToolType,
    pub function: FunctionCall,
}

/// Function call details
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    /// JSON arguments string
    pub arguments: String,
}

/// Tool choice specification
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ToolChoice {
    /// "none", "auto", or "required"
    Auto(String),
    /// Specific tool choice
    Named(NamedToolChoice),
}

/// Named tool choice
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NamedToolChoice {
    #[serde(rename = "type")]
    pub type_: ToolType,
    pub function: NamedFunction,
}

/// Named function reference
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NamedFunction {
    pub name: String,
}

/// Response format specification
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseFormat {
    Text,
    JsonObject,
    JsonSchema { json_schema: JsonSchemaConfig },
}

/// JSON Schema configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonSchemaConfig {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub schema: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// Stop sequences
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Stop {
    Single(String),
    Multiple(Vec<String>),
}

/// Chat completion response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
}

/// Chat completion choice
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatChoice {
    pub index: u32,
    pub message: AssistantMessage,
    pub finish_reason: FinishReason,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<Logprobs>,
}

/// Finish reason
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    Stop,
    Length,
    ToolCalls,
    ContentFilter,
    FunctionCall,
    Error,
}

/// Token usage statistics
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens_details: Option<PromptTokensDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens_details: Option<CompletionTokensDetails>,
}

/// Prompt tokens details
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromptTokensDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u32>,
}

/// Completion tokens details
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionTokensDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u32>,
}

/// Log probabilities
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Logprobs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TokenLogprob>>,
}

/// Token log probability
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenLogprob {
    pub token: String,
    pub logprob: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<Vec<TopLogprob>>,
}

/// Top log probability
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopLogprob {
    pub token: String,
    pub logprob: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}
