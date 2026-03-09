//! OpenAI API types
//!
//! This module contains schema definitions for OpenAI API requests and responses.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Chat completion request message
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum ChatCompletionRequestMessage {
    System {
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    User {
        content: ChatCompletionRequestMessageContent,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    Assistant {
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<ChatCompletionRequestMessageContent>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<ChatCompletionMessageToolCall>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        refusal: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reasoning: Option<String>,
    },
    Tool {
        tool_call_id: String,
        content: String,
    },
}

/// Chat completion request message content
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ChatCompletionRequestMessageContent {
    Text(String),
    Array(Vec<ChatCompletionRequestMessageContentPart>),
}

/// Chat completion request message content part
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChatCompletionRequestMessageContentPart {
    Text { text: String },
    ImageUrl { image_url: ChatCompletionRequestMessageContentPartImageUrl },
    InputAudio { data: String, format: String },
}

/// Image URL for chat completion request
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionRequestMessageContentPartImageUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<ImageDetail>,
}

/// Image detail level
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ImageDetail {
    Auto,
    Low,
    High,
}

/// Tool call in a chat completion message
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionMessageToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: ChatCompletionMessageToolCallFunction,
}

/// Function call in a tool call
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionMessageToolCallFunction {
    pub name: String,
    pub arguments: String,
}

/// Create chat completion request
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateChatCompletionRequest {
    pub messages: Vec<ChatCompletionRequestMessage>,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<ResponseModality>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_search_options: Option<WebSearchOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<AudioOutputParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction: Option<PredictionContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<ChatCompletionStreamOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ChatCompletionTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ChatCompletionToolChoiceOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<ChatCompletionFunctionCallOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<ChatCompletionFunction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

/// Response modality
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ResponseModality {
    Text,
    Audio,
    Image,
}

/// Reasoning effort level
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningEffort {
    Low,
    Medium,
    High,
}

/// Web search options
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebSearchOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_location: Option<WebSearchUserLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_context_size: Option<WebSearchContextSize>,
}

/// Web search user location
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebSearchUserLocation {
    #[serde(rename = "type")]
    pub location_type: String,
    pub approximate: WebSearchLocation,
}

/// Web search location
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebSearchLocation {
    pub country: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
}

/// Web search context size
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum WebSearchContextSize {
    Low,
    Medium,
    High,
}

/// Response format
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseFormat {
    Text,
    JsonObject,
    JsonSchema { json_schema: JsonSchemaFormat },
}

/// JSON schema format
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonSchemaFormat {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub schema: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// Audio output parameters
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioOutputParameters {
    pub voice: VoiceId,
    pub format: AudioFormat,
}

/// Voice identifier
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum VoiceId {
    Alloy,
    Ash,
    Ballad,
    Coral,
    Echo,
    Fable,
    Nova,
    Onyx,
    Sage,
    Shimmer,
}

/// Audio format
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AudioFormat {
    Wav,
    Aac,
    Mp3,
    Flac,
    Opus,
    Pcm16,
}

/// Stop configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum StopConfiguration {
    Single(String),
    Multiple(Vec<String>),
}

/// Prediction content
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum PredictionContent {
    Text(String),
    Tokens(Vec<i32>),
}

/// Chat completion stream options
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionStreamOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_usage: Option<bool>,
}

/// Chat completion tool
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionTool {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: ChatCompletionToolFunction,
}

/// Chat completion tool function
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionToolFunction {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub parameters: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// Chat completion tool choice option
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ChatCompletionToolChoiceOption {
    String(String),
    Object(ChatCompletionNamedToolChoice),
}

/// Chat completion named tool choice
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionNamedToolChoice {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: ChatCompletionNamedToolChoiceFunction,
}

/// Chat completion named tool choice function
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionNamedToolChoiceFunction {
    pub name: String,
}

/// Chat completion function call option (deprecated)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ChatCompletionFunctionCallOption {
    String(String),
    Object(ChatCompletionFunctionCallOptionObject),
}

/// Chat completion function call option object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionFunctionCallOptionObject {
    pub name: String,
}

/// Chat completion function (deprecated)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionFunction {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub parameters: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// Create chat completion response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateChatCompletionResponse {
    pub id: String,
    pub choices: Vec<ChatCompletionChoice>,
    pub created: u64,
    pub model: String,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

/// Chat completion choice
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionChoice {
    pub finish_reason: String,
    pub index: usize,
    pub message: ChatCompletionResponseMessage,
    pub logprobs: Option<ChatCompletionChoiceLogprobs>,
}

/// Chat completion response message
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionResponseMessage {
    pub role: String,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ChatCompletionMessageToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
}

/// Chat completion choice log probabilities
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionChoiceLogprobs {
    pub content: Option<Vec<ChatCompletionTokenLogprob>>,
}

/// Chat completion token log probability
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionTokenLogprob {
    pub token: String,
    pub logprob: f32,
    pub top_logprobs: Vec<ChatCompletionTokenLogprob>,
}

/// Token usage statistics
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    pub completion_tokens: u32,
    pub prompt_tokens: u32,
    pub total_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens_details: Option<CompletionTokensDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens_details: Option<PromptTokensDetail>,
}

/// Completion token details
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionTokensDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_prediction_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_prediction_tokens: Option<u32>,
}

/// Prompt token details
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromptTokensDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_write_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_tokens: Option<u32>,
}

// Embedding types

/// Create embedding request
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateEmbeddingRequest {
    pub input: EmbeddingInput,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<EmbeddingEncodingFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Embedding input
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum EmbeddingInput {
    Text(String),
    Array(Vec<String>),
    ArrayOfArrays(Vec<Vec<String>>),
}

/// Embedding encoding format
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddingEncodingFormat {
    Float,
    Base64,
}

/// Create embedding response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateEmbeddingResponse {
    pub data: Vec<Embedding>,
    pub model: String,
    pub object: String,
    pub usage: EmbeddingUsage,
}

/// Embedding object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Embedding {
    pub index: usize,
    pub embedding: Vec<f32>,
    pub object: String,
}

/// Embedding usage
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingUsage {
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}
