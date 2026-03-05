//! OpenAI provider implementation
//!
//! This module provides structs and enums for OpenAI API requests and responses
//! based on the OpenAPI specification.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Chat completion request message
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum ChatCompletionRequestMessage {
    System {
        /// The contents of the system message
        content: String,
        /// An optional name for the participant
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    User {
        /// The contents of the user message
        content: ChatCompletionRequestMessageContent,
        /// An optional name for the participant
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    Assistant {
        /// The contents of the assistant message
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<ChatCompletionRequestMessageContent>,
        /// An optional name for the participant
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        /// Tool calls made by the assistant
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<ChatCompletionMessageToolCall>>,
        /// Refusal message if content was refused
        #[serde(skip_serializing_if = "Option::is_none")]
        refusal: Option<String>,
        /// Reasoning output
        #[serde(skip_serializing_if = "Option::is_none")]
        reasoning: Option<String>,
    },
    Tool {
        /// Tool call that this message is responding to
        tool_call_id: String,
        /// The contents of the tool message
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
    Text {
        text: String,
    },
    ImageUrl {
        image_url: ChatCompletionRequestMessageContentPartImageUrl,
    },
    InputAudio {
        data: String,
        format: String,
    },
}

/// Image URL for chat completion request
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionRequestMessageContentPartImageUrl {
    /// URL or base64-encoded image data
    pub url: String,
    /// Specifies the detail level of the image
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
    /// The ID of the tool call
    pub id: String,
    /// The type of the tool
    #[serde(rename = "type")]
    pub tool_type: String,
    /// The function that the model called
    pub function: ChatCompletionMessageToolCallFunction,
}

/// Function call in a tool call
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionMessageToolCallFunction {
    /// The name of the function to call
    pub name: String,
    /// The arguments to call the function with, as JSON string
    pub arguments: String,
}

/// Create chat completion request
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateChatCompletionRequest {
    /// A list of messages comprising the conversation so far
    pub messages: Vec<ChatCompletionRequestMessage>,
    /// Model ID used to generate the response
    pub model: String,
    /// Output modalities for the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<ResponseModality>>,
    /// Reasoning effort level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,
    /// Maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<i32>,
    /// Frequency penalty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    /// Presence penalty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    /// Web search options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_search_options: Option<WebSearchOptions>,
    /// Number of most likely tokens to return at each position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<i32>,
    /// Response format specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    /// Audio output parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<AudioOutputParameters>,
    /// Whether to store the output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    /// Whether to stream the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Stop sequences
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopConfiguration>,
    /// Logit bias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i32>>,
    /// Whether to return log probabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    /// Maximum tokens (deprecated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    /// Number of chat completion choices to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    /// Prediction configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction: Option<PredictionContent>,
    /// Random seed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Stream options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<ChatCompletionStreamOptions>,
    /// List of tools the model may call
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ChatCompletionTool>>,
    /// Tool choice option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ChatCompletionToolChoiceOption>,
    /// Whether to enable parallel tool calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    /// Function call (deprecated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<ChatCompletionFunctionCallOption>,
    /// Functions (deprecated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<ChatCompletionFunction>>,
    /// Temperature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Top-p sampling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// User identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Session identifier
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
    /// User location for search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_location: Option<WebSearchUserLocation>,
    /// Search context size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_context_size: Option<WebSearchContextSize>,
}

/// Web search user location
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebSearchUserLocation {
    /// Type of location approximation
    #[serde(rename = "type")]
    pub location_type: String,
    /// Approximate location
    pub approximate: WebSearchLocation,
}

/// Web search location
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebSearchLocation {
    /// Country code
    pub country: String,
    /// Region code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// City name
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
    JsonSchema {
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

/// Audio output parameters
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioOutputParameters {
    /// Voice to use for audio output
    pub voice: VoiceId,
    /// Audio format
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
    /// Whether to include usage in stream
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_usage: Option<bool>,
}

/// Chat completion tool
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionTool {
    /// Type of the tool
    #[serde(rename = "type")]
    pub tool_type: String,
    /// Function definition
    pub function: ChatCompletionToolFunction,
}

/// Chat completion tool function
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionToolFunction {
    /// Name of the function
    pub name: String,
    /// Description of the function
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Parameters of the function
    pub parameters: serde_json::Value,
    /// Whether to enforce strict parameter validation
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
    /// Type of the tool choice
    #[serde(rename = "type")]
    pub tool_type: String,
    /// Function to call
    pub function: ChatCompletionNamedToolChoiceFunction,
}

/// Chat completion named tool choice function
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionNamedToolChoiceFunction {
    /// Name of the function to call
    pub name: String,
}

/// Chat completion function call option (deprecated)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ChatCompletionFunctionCallOption {
    String(String),
    Object(ChatCompletionFunctionCallOptionObject),
}

/// Chat completion function call option object (deprecated)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionFunctionCallOptionObject {
    /// Name of the function to call
    pub name: String,
}

/// Chat completion function (deprecated)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionFunction {
    /// Name of the function
    pub name: String,
    /// Description of the function
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Parameters of the function
    pub parameters: serde_json::Value,
    /// Whether to enforce strict parameter validation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// Create chat completion response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateChatCompletionResponse {
    /// Unique identifier for the chat completion
    pub id: String,
    /// List of chat completion choices
    pub choices: Vec<ChatCompletionChoice>,
    /// Unix timestamp of creation
    pub created: u64,
    /// Model used for completion
    pub model: String,
    /// Object type
    pub object: String,
    /// System fingerprint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
    /// Token usage statistics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

/// Chat completion choice
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionChoice {
    /// Reason the model stopped generating tokens
    pub finish_reason: String,
    /// Index of the choice
    pub index: usize,
    /// Message generated by the model
    pub message: ChatCompletionResponseMessage,
    /// Log probabilities
    pub logprobs: Option<ChatCompletionChoiceLogprobs>,
}

/// Chat completion response message
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionResponseMessage {
    /// Role of the message author
    pub role: String,
    /// Content of the message
    pub content: Option<String>,
    /// Tool calls made by the assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ChatCompletionMessageToolCall>>,
    /// Refusal message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
    /// Reasoning output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
}

/// Chat completion choice log probabilities
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionChoiceLogprobs {
    /// Log probabilities of content tokens
    pub content: Option<Vec<ChatCompletionTokenLogprob>>,
}

/// Chat completion token log probability
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionTokenLogprob {
    /// The token
    pub token: String,
    /// The log probability of this token
    pub logprob: f32,
    /// List of most likely tokens and their log probabilities
    pub top_logprobs: Vec<ChatCompletionTokenLogprob>,
}

/// Token usage statistics
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    /// Number of tokens in the completion
    pub completion_tokens: u32,
    /// Number of tokens in the prompt
    pub prompt_tokens: u32,
    /// Total number of tokens
    pub total_tokens: u32,
    /// Completion token details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens_details: Option<CompletionTokensDetail>,
    /// Prompt token details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens_details: Option<PromptTokensDetail>,
}

/// Completion token details
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionTokensDetail {
    /// Tokens used for reasoning
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_tokens: Option<u32>,
    /// Tokens used for audio output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u32>,
    /// Accepted prediction tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_prediction_tokens: Option<u32>,
    /// Rejected prediction tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_prediction_tokens: Option<u32>,
}

/// Prompt token details
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromptTokensDetail {
    /// Cached prompt tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_tokens: Option<u32>,
    /// Tokens written to cache
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_write_tokens: Option<u32>,
    /// Audio input tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u32>,
    /// Video input tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_tokens: Option<u32>,
}

/// Create completion request (legacy)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCompletionRequest {
    /// ID of the model to use
    pub model: String,
    /// The prompt(s) to generate completions for
    pub prompt: CompletionPrompt,
    /// Suffix that comes after a completion of inserted text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    /// Maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    /// Temperature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Top-p sampling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Number of completions to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    /// Whether to stream the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Log probabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<i32>,
    /// Echo the prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,
    /// Stop sequences
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopConfiguration>,
    /// Presence penalty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    /// Frequency penalty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    /// Best of
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<i32>,
    /// Logit bias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i32>>,
    /// User identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Seed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

/// Completion prompt
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum CompletionPrompt {
    Text(String),
    Array(Vec<String>),
    ArrayOfArrays(Vec<Vec<String>>),
}

/// Create completion response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCompletionResponse {
    /// Unique identifier for the completion
    pub id: String,
    /// List of completion choices
    pub choices: Vec<CompletionChoice>,
    /// Unix timestamp of creation
    pub created: u64,
    /// Model used for completion
    pub model: String,
    /// Object type
    pub object: String,
    /// System fingerprint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
    /// Token usage statistics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

/// Completion choice
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionChoice {
    /// Text of the completion
    pub text: String,
    /// Index of the choice
    pub index: usize,
    /// Log probabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<CompletionChoiceLogprobs>,
    /// Reason the model stopped generating
    pub finish_reason: String,
}

/// Completion choice log probabilities
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionChoiceLogprobs {
    /// Log probabilities of tokens
    pub tokens: Vec<String>,
    /// Log probabilities of token log probabilities
    pub token_logprobs: Vec<f32>,
    /// Top log probabilities
    pub top_logprobs: Vec<HashMap<String, f32>>,
    /// Text offsets
    pub text_offset: Vec<usize>,
}

/// Create embedding request
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateEmbeddingRequest {
    /// Input text to embed
    pub input: EmbeddingInput,
    /// ID of the model to use
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
    /// List of embedding objects
    pub data: Vec<Embedding>,
    /// Model used for embedding
    pub model: String,
    /// Object type
    pub object: String,
    /// Token usage statistics
    pub usage: EmbeddingUsage,
}

/// Embedding object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Embedding {
    /// Index of the embedding
    pub index: usize,
    /// The embedding vector
    pub embedding: Vec<f32>,
    /// Object type
    pub object: String,
}

/// Embedding usage
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingUsage {
    /// Number of tokens in the prompt
    pub prompt_tokens: u32,
    /// Total number of tokens
    pub total_tokens: u32,
}

/// Create image request
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateImageRequest {
    /// Text description of the desired image
    pub prompt: String,
    /// Model to use for image generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Number of images to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    /// Size of the generated images
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,
    /// Format of the generated images
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ImageResponseFormat>,
    /// Quality of the generated images
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<ImageQuality>,
    /// Style of the generated images
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ImageStyle>,
    /// User identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Image size
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ImageSize {
    #[serde(rename = "256x256")]
    Size256x256,
    #[serde(rename = "512x512")]
    Size512x512,
    #[serde(rename = "1024x1024")]
    Size1024x1024,
    #[serde(rename = "1792x1024")]
    Size1792x1024,
    #[serde(rename = "1024x1792")]
    Size1024x1792,
}

/// Image response format
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ImageResponseFormat {
    Url,
    B64Json,
}

/// Image quality
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ImageQuality {
    Standard,
    Hd,
}

/// Image style
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ImageStyle {
    Vivid,
    Natural,
}

/// Create image response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateImageResponse {
    /// Unix timestamp of creation
    pub created: u64,
    /// List of image data
    pub data: Vec<ImageData>,
}

/// Image data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageData {
    /// URL of the generated image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Base64-encoded image data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b64_json: Option<String>,
    /// Revised prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revised_prompt: Option<String>,
}

/// Create transcription request
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTranscriptionRequest {
    /// The audio file to transcribe
    pub file: String,
    /// ID of the model to use
    pub model: String,
    /// Language of the audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Prompt to guide the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Response format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<TranscriptionResponseFormat>,
    /// Temperature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Timestamp granularities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_granularities: Option<Vec<TranscriptionTimestampGranularity>>,
}

/// Transcription response format
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptionResponseFormat {
    Json,
    Text,
    Srt,
    Vtt,
    VerboseJson,
}

/// Transcription timestamp granularity
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptionTimestampGranularity {
    Word,
    Segment,
}

/// Create transcription response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTranscriptionResponse {
    /// The transcribed text
    pub text: String,
}

/// List models response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListModelsResponse {
    /// Object type
    pub object: String,
    /// List of model objects
    pub data: Vec<Model>,
}

/// Model object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Model {
    /// Model identifier
    pub id: String,
    /// Object type
    pub object: String,
    /// Unix timestamp of creation
    pub created: u64,
    /// Model owner
    pub owned_by: String,
}

/// Create moderation request
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateModerationRequest {
    /// Input text to moderate
    pub input: ModerationInput,
    /// Model to use for moderation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// Moderation input
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ModerationInput {
    Text(String),
    Array(Vec<String>),
}

/// Create moderation response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateModerationResponse {
    /// Unique identifier for the moderation request
    pub id: String,
    /// Model used for moderation
    pub model: String,
    /// Moderation results
    pub results: Vec<ModerationResult>,
}

/// Moderation result
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModerationResult {
    /// Whether the content was flagged
    pub flagged: bool,
    /// Category scores
    pub category_scores: ModerationCategoryScores,
    /// Categories
    pub categories: ModerationCategories,
}

/// Moderation category scores
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModerationCategoryScores {
    /// Hate score
    pub hate: f32,
    /// Hate/threatening score
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: f32,
    /// Harassment score
    pub harassment: f32,
    /// Harassment/threatening score
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: f32,
    /// Self-harm score
    #[serde(rename = "self-harm")]
    pub self_harm: f32,
    /// Self-harm/intent score
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: f32,
    /// Self-harm/instructions score
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: f32,
    /// Sexual score
    pub sexual: f32,
    /// Sexual/minors score
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: f32,
    /// Violence score
    pub violence: f32,
    /// Violence/graphic score
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: f32,
}

/// Moderation categories
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModerationCategories {
    /// Hate category
    pub hate: bool,
    /// Hate/threatening category
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: bool,
    /// Harassment category
    pub harassment: bool,
    /// Harassment/threatening category
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: bool,
    /// Self-harm category
    #[serde(rename = "self-harm")]
    pub self_harm: bool,
    /// Self-harm/intent category
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: bool,
    /// Self-harm/instructions category
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: bool,
    /// Sexual category
    pub sexual: bool,
    /// Sexual/minors category
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: bool,
    /// Violence category
    pub violence: bool,
    /// Violence/graphic category
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: bool,
}

// ==========================================
// From trait implementations for conversion
// ==========================================



impl From<crate::provider::types::EmbeddingRequest> for CreateEmbeddingRequest {
    fn from(req: crate::provider::types::EmbeddingRequest) -> Self {
        // Convert input to OpenAI format
        let input = if req.input.len() == 1 {
            EmbeddingInput::Text(req.input[0].clone())
        } else {
            EmbeddingInput::Array(req.input)
        };
        
        // Convert encoding format
        let encoding_format = req.encoding_format.map(|fmt| match fmt {
            crate::provider::types::EmbeddingEncodingFormat::Float => EmbeddingEncodingFormat::Float,
            crate::provider::types::EmbeddingEncodingFormat::Base64 => EmbeddingEncodingFormat::Base64,
        });
        
        CreateEmbeddingRequest {
            input,
            model: req.model,
            encoding_format,
            dimensions: req.dimensions,
            user: req.user,
        }
    }
}

impl From<CreateChatCompletionResponse> for crate::provider::types::ChatResponse {
    fn from(resp: CreateChatCompletionResponse) -> Self {
        // Convert choices
        let choices = resp.choices.into_iter().map(|choice| {
            // Convert message
            let message = {
                // Convert content to unified Content type
                let content = choice.message.content.map(|text| crate::provider::types::Content::Text(text));
                
                // Convert tool calls
                let tool_calls = choice.message.tool_calls.map(|calls| {
                    calls.into_iter().map(|call| {
                        crate::provider::types::ToolCall {
                            id: call.id,
                            tool_type: call.tool_type,
                            function: crate::provider::types::ToolCallFunction {
                                name: call.function.name,
                                arguments: call.function.arguments,
                            },
                        }
                    }).collect()
                });
                
                crate::provider::types::ChatMessage {
                    role: crate::provider::types::Role::Assistant,
                    content,
                    name: None,
                    tool_calls,
                    tool_call_id: None,
                }
            };
            
            // Convert finish reason (currently unused, but kept for future use)
            let _finish_reason = match choice.finish_reason.as_str() {
                "stop" => crate::provider::types::FinishReason::Stop,
                "length" => crate::provider::types::FinishReason::Length,
                "tool_calls" => crate::provider::types::FinishReason::ToolCalls,
                "content_filter" => crate::provider::types::FinishReason::ContentFilter,
                "function_call" => crate::provider::types::FinishReason::FunctionCall,
                _ => crate::provider::types::FinishReason::Stop, // Default
            };
            
            // Convert logprobs
            let logprobs = choice.logprobs.map(|logprobs| {
                crate::provider::types::Logprobs {
                    content: logprobs.content.map(|content| {
                        content.into_iter().map(|c| {
                            crate::provider::types::LogprobContent {
                                token: c.token,
                                logprob: c.logprob as f64,
                                bytes: None, // OpenAI doesn't provide bytes
                                top_logprobs: Some(c.top_logprobs.into_iter().map(|t| {
                                    crate::provider::types::TopLogprob {
                                        token: t.token,
                                        logprob: t.logprob as f64,
                                        bytes: None, // OpenAI doesn't provide bytes
                                    }
                                }).collect()),
                            }
                        }).collect()
                    }),
                }
            });
            
            crate::provider::types::ChatChoice {
                index: choice.index as i32,
                message,
                finish_reason: crate::provider::types::FinishReason::Stop,
                logprobs,
            }
        }).collect();
        
        // Convert usage
        let usage = resp.usage.map(|u| {
            crate::provider::types::Usage {
                prompt_tokens: u.prompt_tokens as i32,
                completion_tokens: u.completion_tokens as i32,
                total_tokens: u.total_tokens as i32,
            }
        });
        
        crate::provider::types::ChatResponse {
            id: resp.id,
            object: resp.object,
            created: resp.created as i64,
            model: resp.model,
            choices,
            usage,
            system_fingerprint: resp.system_fingerprint,
        }
    }
}

impl From<CreateEmbeddingResponse> for crate::provider::types::EmbeddingResponse {
    fn from(resp: CreateEmbeddingResponse) -> Self {
        // Convert data
        let data = resp.data.into_iter().map(|embedding| {
            crate::provider::types::EmbeddingObject {
                index: embedding.index as usize,
                embedding: embedding.embedding,
                object: embedding.object,
            }
        }).collect();
        
        // Convert usage
        let usage = crate::provider::types::EmbeddingUsage {
            prompt_tokens: resp.usage.prompt_tokens,
            total_tokens: resp.usage.total_tokens,
        };
        
        crate::provider::types::EmbeddingResponse {
            object: resp.object,
            data,
            model: resp.model,
            usage,
        }
    }
}

impl From<crate::provider::types::ChatRequest> for CreateChatCompletionRequest {
    fn from(req: crate::provider::types::ChatRequest) -> Self {
        // Convert messages
        let messages = req.messages.into_iter().map(|msg| {
            match msg.role {
                crate::provider::types::Role::System => {
                    // Extract text content from system message
                    let content = match msg.content {
                        Some(crate::provider::types::Content::Text(text)) => text,
                        Some(crate::provider::types::Content::Parts(parts)) => {
                            // Extract text from parts
                            let mut text_parts = Vec::new();
                            for part in parts {
                                match part {
                                    crate::provider::types::ContentPart::Text { text } => {
                                        text_parts.push(text);
                                    }
                                    _ => {
                                        // Skip non-text parts for system messages
                                    }
                                }
                            }
                            text_parts.join(" ")
                        }
                        None => String::new(),
                    };
                    
                    ChatCompletionRequestMessage::System {
                        content,
                        name: msg.name,
                    }
                }
                crate::provider::types::Role::User => {
                    // Convert content to OpenAI format
                    let content = match msg.content {
                        Some(crate::provider::types::Content::Text(text)) => {
                            ChatCompletionRequestMessageContent::Text(text)
                        }
                        Some(crate::provider::types::Content::Parts(parts)) => {
                            // Convert parts to OpenAI format
                            let mut openai_parts = Vec::new();
                            for part in parts {
                                match part {
                                    crate::provider::types::ContentPart::Text { text } => {
                                        openai_parts.push(
                                            ChatCompletionRequestMessageContentPart::Text { text }
                                        );
                                    }
                                    crate::provider::types::ContentPart::ImageUrl { image_url } => {
                                        openai_parts.push(
                                            ChatCompletionRequestMessageContentPart::ImageUrl {
                                                image_url: ChatCompletionRequestMessageContentPartImageUrl {
                                                    url: image_url.url,
                                                    detail: image_url.detail.map(|d| match d {
                                                        crate::provider::types::ImageDetail::Low => ImageDetail::Low,
                                                        crate::provider::types::ImageDetail::High => ImageDetail::High,
                                                        crate::provider::types::ImageDetail::Auto => ImageDetail::Auto,
                                                    }),
                                                }
                                            }
                                        );
                                    }
                                    // TODO: Support audio and video content in OpenAI format
                                    _ => {
                                        // Skip unsupported content types for now
                                    }
                                }
                            }
                            ChatCompletionRequestMessageContent::Array(openai_parts)
                        }
                        None => ChatCompletionRequestMessageContent::Text(String::new()),
                    };
                    
                    ChatCompletionRequestMessage::User {
                        content,
                        name: msg.name,
                    }
                }
                crate::provider::types::Role::Assistant => {
                    // Convert content to OpenAI format
                    let content = msg.content.and_then(|c| match c {
                        crate::provider::types::Content::Text(text) => {
                            Some(ChatCompletionRequestMessageContent::Text(text))
                        }
                        crate::provider::types::Content::Parts(parts) => {
                            // Extract text from parts for assistant messages
                            let mut text_parts = Vec::new();
                            for part in parts {
                                match part {
                                    crate::provider::types::ContentPart::Text { text } => {
                                        text_parts.push(text);
                                    }
                                    _ => {
                                        // Skip non-text parts for assistant messages
                                    }
                                }
                            }
                            if !text_parts.is_empty() {
                                Some(ChatCompletionRequestMessageContent::Text(text_parts.join(" ")))
                            } else {
                                None
                            }
                        }
                    });
                    
                    // Convert tool calls
                    let tool_calls = msg.tool_calls.map(|calls| {
                        calls.into_iter().map(|call| {
                            ChatCompletionMessageToolCall {
                                id: call.id,
                                tool_type: call.tool_type,
                                function: ChatCompletionMessageToolCallFunction {
                                    name: call.function.name,
                                    arguments: call.function.arguments,
                                },
                            }
                        }).collect()
                    });
                    
                    ChatCompletionRequestMessage::Assistant {
                        content,
                        name: msg.name,
                        tool_calls,
                        refusal: None,
                        reasoning: None,
                    }
                }
                crate::provider::types::Role::Tool => {
                    // Extract text content from tool message
                    let content = match msg.content {
                        Some(crate::provider::types::Content::Text(text)) => text,
                        Some(crate::provider::types::Content::Parts(parts)) => {
                            // Extract text from parts
                            let mut text_parts = Vec::new();
                            for part in parts {
                                match part {
                                    crate::provider::types::ContentPart::Text { text } => {
                                        text_parts.push(text);
                                    }
                                    _ => {
                                        // Skip non-text parts for tool messages
                                    }
                                }
                            }
                            text_parts.join(" ")
                        }
                        None => String::new(),
                    };
                    
                    ChatCompletionRequestMessage::Tool {
                        tool_call_id: msg.tool_call_id.unwrap_or_default(),
                        content,
                    }
                }
            }
        }).collect();
        
        // Convert tools
        let tools = req.tools.map(|tools| {
            tools.into_iter().map(|tool| {
                ChatCompletionTool {
                    tool_type: tool.tool_type,
                    function: ChatCompletionToolFunction {
                        name: tool.function.name,
                        description: tool.function.description,
                        parameters: tool.function.parameters,
                        strict: tool.function.strict,
                    },
                }
            }).collect()
        });
        
        // Convert tool choice
        let tool_choice = req.tool_choice.map(|choice| match choice {
            crate::provider::types::ToolChoice::String(s) => ChatCompletionToolChoiceOption::String(s),
            crate::provider::types::ToolChoice::Object(obj) => {
                ChatCompletionToolChoiceOption::Object(ChatCompletionNamedToolChoice {
                    tool_type: obj.tool_type,
                    function: ChatCompletionNamedToolChoiceFunction {
                        name: obj.function.name,
                    },
                })
            }
        });
        
        // Convert response format
        let response_format = req.response_format.map(|format| match format {
            crate::provider::types::ResponseFormat::Text => ResponseFormat::Text,
            crate::provider::types::ResponseFormat::JsonObject => ResponseFormat::JsonObject,
            crate::provider::types::ResponseFormat::JsonSchema { json_schema } => {
                ResponseFormat::JsonSchema {
                    json_schema: JsonSchemaFormat {
                        name: json_schema.name,
                        description: json_schema.description,
                        schema: json_schema.schema,
                        strict: json_schema.strict,
                    },
                }
            }
        });
        
        CreateChatCompletionRequest {
            messages,
            model: req.model,
            modalities: None, // TODO: Support modalities
            reasoning_effort: None, // TODO: Support reasoning effort
            max_completion_tokens: req.max_tokens.map(|t| t as i32),
            frequency_penalty: req.frequency_penalty,
            presence_penalty: req.presence_penalty,
            web_search_options: None, // TODO: Support web search
            top_logprobs: req.top_logprobs.map(|t| t as i32),
            response_format,
            audio: None, // TODO: Support audio output
            store: None, // TODO: Support store
            stream: req.stream,
            stop: req.stop.map(|stop| match stop {
                crate::provider::types::Stop::Single(s) => StopConfiguration::Single(s),
                crate::provider::types::Stop::Multiple(arr) => StopConfiguration::Multiple(arr),
            }),
            logit_bias: req.logit_bias.map(|bias| {
                bias.into_iter().map(|(k, v)| (k, v as i32)).collect()
            }),
            logprobs: req.logprobs,
            max_tokens: req.max_tokens.map(|t| t as i32),
            n: req.n.map(|n| n as i32),
            prediction: None, // TODO: Support prediction
            seed: req.seed.map(|s| s as i64),
            stream_options: req.stream_options.map(|options| {
                ChatCompletionStreamOptions {
                    include_usage: options.include_usage,
                }
            }),
            tools,
            tool_choice,
            parallel_tool_calls: req.parallel_tool_calls,
            function_call: None, // Deprecated
            functions: None, // Deprecated
            temperature: req.temperature,
            top_p: req.top_p,
            user: req.user,
            session_id: None, // TODO: Support session ID
        }
    }
}

