//! OpenRouter API types and client
//!
//! This module contains schema definitions and client implementations
//! for /chat/completions and /embeddings endpoints based on openrouter.trimmed.yaml

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ==========================================
// Chat Completion Types
// ==========================================

/// Chat completion request (ChatGenerationParams)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatRequest {
    /// Messages in the conversation
    pub messages: Vec<ChatMessage>,
    /// Model to use for completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Models to use for completion (OpenRouter specific)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<String>>,
    /// Response format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    /// Stop sequences
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Stop>,
    /// Whether to stream the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Stream options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<ChatStreamOptions>,
    /// Maximum number of tokens to generate (deprecated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Maximum completion tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u32>,
    /// Temperature sampling parameter (0-2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Top-p sampling parameter (0-1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Top-k sampling parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    /// Frequency penalty (-2 to 2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    /// Presence penalty (-2 to 2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    /// Repetition penalty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repetition_penalty: Option<f32>,
    /// Random seed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// List of tools the model may call
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// Tool choice option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    /// Logit bias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, f32>>,
    /// Whether to return log probabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    /// Number of top log probabilities to return (0-20)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,
    /// Unique identifier for the end-user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Session ID for grouping requests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// Metadata (max 16 pairs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// Provider preferences for routing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<ProviderPreferences>,
    /// Plugins to enable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Vec<Plugin>>,
    /// Reasoning configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningConfig>,
    /// Trace metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<TraceConfig>,
}

/// Chat stream options
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatStreamOptions {
    /// Whether to include usage in stream
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_usage: Option<bool>,
}

/// Chat message
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum ChatMessage {
    System(SystemMessage),
    User(UserMessage),
    Assistant(AssistantMessage),
    Tool(ToolMessage),
    Developer(DeveloperMessage),
}

/// System message
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemMessage {
    pub content: MessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// User message
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

/// Tool message
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolMessage {
    pub content: MessageContent,
    pub tool_call_id: String,
}

/// Developer message
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeveloperMessage {
    pub content: MessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Message content
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    Parts(Vec<ContentPart>),
}

/// Content part
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    },
    ImageUrl {
        image_url: ImageUrl,
    },
}

/// Cache control for content
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CacheControl {
    #[serde(rename = "type")]
    pub cache_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
}

/// Image URL
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageUrl {
    pub url: String,
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

/// Tool call
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub function: FunctionCall,
}

/// Function call
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

/// Tool definition
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tool {
    #[serde(rename = "type")]
    pub type_: String,
    pub function: FunctionDefinition,
}

/// Function definition
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// Tool choice
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ToolChoice {
    String(String),
    Object(ToolChoiceObject),
}

/// Tool choice object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolChoiceObject {
    #[serde(rename = "type")]
    pub type_: String,
    pub function: ToolChoiceFunction,
}

/// Tool choice function
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolChoiceFunction {
    pub name: String,
}

/// Response format
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseFormat {
    Text,
    JsonObject,
    JsonSchema {
        json_schema: JsonSchemaConfig,
    },
    Grammar {
        grammar: String,
    },
    Python,
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
    String(String),
    Array(Vec<String>),
}

/// Provider preferences for routing
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ProviderPreferences {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_fallbacks: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_parameters: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_collection: Option<DataCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zdr: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_distillable_text: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantizations: Option<Vec<Quantization>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<ProviderSort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_price: Option<MaxPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_min_throughput: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_max_latency: Option<f64>,
}

/// Data collection setting
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DataCollection {
    Allow,
    Deny,
}

/// Quantization level
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Quantization {
    Int4,
    Int8,
    Fp4,
    Fp6,
    Fp8,
    Fp16,
    Bf16,
    Fp32,
    Unknown,
}

/// Provider sort configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ProviderSort {
    String(String),
    Config(ProviderSortConfig),
}

/// Provider sort config
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProviderSortConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
}

/// Max price configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MaxPrice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<f64>,
}

/// Plugin configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "id")]
pub enum Plugin {
    #[serde(rename = "auto-router")]
    AutoRouter {
        #[serde(skip_serializing_if = "Option::is_none")]
        enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        allowed_models: Option<Vec<String>>,
    },
    #[serde(rename = "moderation")]
    Moderation,
    #[serde(rename = "web")]
    Web {
        #[serde(skip_serializing_if = "Option::is_none")]
        enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_results: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        search_prompt: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        engine: Option<WebSearchEngine>,
    },
    #[serde(rename = "file-parser")]
    FileParser {
        #[serde(skip_serializing_if = "Option::is_none")]
        enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pdf: Option<PdfParserOptions>,
    },
    #[serde(rename = "response-healing")]
    ResponseHealing {
        #[serde(skip_serializing_if = "Option::is_none")]
        enabled: Option<bool>,
    },
}

/// Web search engine
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum WebSearchEngine {
    Native,
    Exa,
}

/// PDF parser options
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PdfParserOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<PdfParserEngine>,
}

/// PDF parser engine
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PdfParserEngine {
    MistralOcr,
    PdfText,
    Native,
}

/// Reasoning configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReasoningConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<ReasoningEffort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<ReasoningSummaryVerbosity>,
}

/// Reasoning effort level
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningEffort {
    Xhigh,
    High,
    Medium,
    Low,
    Minimal,
    None,
}

/// Reasoning summary verbosity
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningSummaryVerbosity {
    Auto,
    Concise,
    Detailed,
}

/// Trace configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TraceConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_span_id: Option<String>,
}

/// Chat completion response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatResponse {
    pub id: String,
    pub choices: Vec<ChatChoice>,
    pub created: u64,
    pub model: String,
    pub object: String,
    pub system_fingerprint: Option<String>,
    pub usage: ChatGenerationTokenUsage,
}

/// Chat completion choice
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatChoice {
    pub finish_reason: Option<ChatCompletionFinishReason>,
    pub index: u32,
    pub message: AssistantMessage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<ChatMessageTokenLogprobs>,
}

/// Chat completion finish reason
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ChatCompletionFinishReason {
    ToolCalls,
    Stop,
    Length,
    ContentFilter,
    Error,
}

/// Token usage statistics
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatGenerationTokenUsage {
    pub completion_tokens: u32,
    pub prompt_tokens: u32,
    pub total_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens_details: Option<CompletionTokensDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens_details: Option<PromptTokensDetails>,
}

/// Completion tokens details
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionTokensDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_prediction_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_prediction_tokens: Option<u32>,
}

/// Prompt tokens details
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromptTokensDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_write_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_tokens: Option<u32>,
}

/// Token log probabilities
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessageTokenLogprobs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TokenLogprob>>,
}

/// Token logprob
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenLogprob {
    pub token: String,
    pub logprob: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<Vec<TopLogprob>>,
}

/// Top logprob
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopLogprob {
    pub token: String,
    pub logprob: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}

// ==========================================
// Embeddings Types
// ==========================================

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
    /// Single string input
    String(String),
    /// Array of string inputs
    StringArray(Vec<String>),
    /// Array of numeric tokens
    NumberArray(Vec<f64>),
    /// Array of numeric arrays
    NumberArray2D(Vec<Vec<f64>>),
    /// Array of content items
    ContentArray(Vec<EmbeddingContentItem>),
}

/// Embeddings request
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingsRequest {
    /// Input to embed
    pub input: EmbeddingInput,
    /// Model to use for embeddings
    pub model: String,
    /// Encoding format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<EmbeddingEncodingFormat>,
    /// Number of dimensions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<u32>,
    /// User identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Provider preferences
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<ProviderPreferences>,
    /// Input type hint
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

// ==========================================
// OpenRouter Client
// ==========================================

use anyhow::{Result, anyhow};

/// OpenRouter API client configuration
#[derive(Clone, Debug)]
pub struct OpenRouterConfig {
    /// API key for authentication
    pub api_key: String,
    /// Base URL for the API (default: https://openrouter.ai/api/v1)
    pub base_url: String,
    /// Optional HTTP client timeout
    pub timeout_secs: Option<u64>,
}

impl Default for OpenRouterConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://openrouter.ai/api/v1".to_string(),
            timeout_secs: Some(30),
        }
    }
}

impl OpenRouterConfig {
    /// Create a new configuration with the given API key
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            ..Default::default()
        }
    }

    /// Set the base URL
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Set the timeout
    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = Some(timeout_secs);
        self
    }
}

/// OpenRouter API client
#[derive(Clone)]
pub struct OpenRouterClient {
    config: OpenRouterConfig,
    http_client: reqwest::Client,
}

impl OpenRouterClient {
    /// Create a new OpenRouter client with the given configuration
    pub fn new(config: OpenRouterConfig) -> Result<Self> {
        let mut builder = reqwest::Client::builder();
        if let Some(timeout) = config.timeout_secs {
            builder = builder.timeout(std::time::Duration::from_secs(timeout));
        }
        let http_client = builder.build()?;

        Ok(Self {
            config,
            http_client,
        })
    }

    /// Create a new OpenRouter client with API key
    pub fn with_api_key(api_key: impl Into<String>) -> Result<Self> {
        let config = OpenRouterConfig::new(api_key);
        Self::new(config)
    }

    /// Create a new OpenRouter client with API key and custom base URL
    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Result<Self> {
        let config = OpenRouterConfig::new(api_key).with_base_url(base_url);
        Self::new(config)
    }

    /// Get the API key
    pub fn api_key(&self) -> &str {
        &self.config.api_key
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.config.base_url
    }

    /// Build a URL from a path
    fn build_url(&self, path: &str) -> String {
        format!("{}{}", self.config.base_url, path)
    }

    /// Create an authenticated request builder
    fn request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        let url = self.build_url(path);
        self.http_client
            .request(method, &url)
            .header("Authorization", format!("Bearer {}", self.api_key()))
            .header("Content-Type", "application/json")
            .header("HTTP-Referer", "https://github.com/eye/eye")
            .header("X-Title", "Eye Assistant")
    }

    /// Create a chat completion
    pub async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let url = self.build_url("/chat/completions");

        let response = self
            .http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key()))
            .header("Content-Type", "application/json")
            .header("HTTP-Referer", "https://github.com/eye/eye")
            .header("X-Title", "Eye Assistant")
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json::<ChatResponse>().await?)
        } else {
            let error = response.text().await?;
            Err(anyhow!("Chat completion failed: {}", error))
        }
    }

    /// Create embeddings
    pub async fn embeddings(&self, request: EmbeddingsRequest) -> Result<EmbeddingsResponse> {
        let url = self.build_url("/embeddings");

        let response = self
            .http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key()))
            .header("Content-Type", "application/json")
            .header("HTTP-Referer", "https://github.com/eye/eye")
            .header("X-Title", "Eye Assistant")
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json::<EmbeddingsResponse>().await?)
        } else {
            let error = response.text().await?;
            Err(anyhow!("Embeddings request failed: {}", error))
        }
    }
}

/// Helper function to create a new OpenRouter client
pub fn create_client(api_key: impl Into<String>) -> Result<OpenRouterClient> {
    OpenRouterClient::with_api_key(api_key)
}

/// Helper function to create a new OpenRouter client with custom base URL
pub fn create_client_with_base_url(
    api_key: impl Into<String>,
    base_url: impl Into<String>,
) -> Result<OpenRouterClient> {
    OpenRouterClient::with_base_url(api_key, base_url)
}

// ==========================================
// From Trait Implementations for Type Conversion
// ==========================================

// Convert from types::ChatRequest to openrouter::ChatRequest
impl From<crate::provider::types::ChatRequest> for ChatRequest {
    fn from(req: crate::provider::types::ChatRequest) -> Self {
        // Convert messages
        let messages = req.messages.into_iter().map(|msg| {
            match msg {
                crate::provider::types::ChatMessage::System(s) => {
                    let content = match s.content {
                        crate::provider::types::MessageContent::Text(text) => text,
                        crate::provider::types::MessageContent::Parts(parts) => {
                            parts.into_iter().filter_map(|part| {
                                match part {
                                    crate::provider::types::ContentPart::Text { text } => Some(text),
                                    _ => None,
                                }
                            }).collect::<Vec<_>>().join(" ")
                        }
                    };
                    ChatMessage::System(SystemMessage {
                        content: MessageContent::Text(content),
                        name: s.name,
                    })
                }
                crate::provider::types::ChatMessage::User(u) => {
                    let content = match u.content {
                        crate::provider::types::MessageContent::Text(text) => MessageContent::Text(text),
                        crate::provider::types::MessageContent::Parts(parts) => {
                            let content_parts = parts.into_iter().filter_map(|part| {
                                match part {
                                    crate::provider::types::ContentPart::Text { text } => {
                                        Some(ContentPart::Text { text, cache_control: None })
                                    }
                                    crate::provider::types::ContentPart::ImageUrl { image_url } => {
                                        Some(ContentPart::ImageUrl {
                                            image_url: ImageUrl {
                                                url: image_url.url,
                                                detail: image_url.detail.map(|d| match d {
                                                    crate::provider::types::ImageDetail::Low => ImageDetail::Low,
                                                    crate::provider::types::ImageDetail::High => ImageDetail::High,
                                                    crate::provider::types::ImageDetail::Auto => ImageDetail::Auto,
                                                }),
                                            }
                                        })
                                    }
                                    _ => None,
                                }
                            }).collect();
                            MessageContent::Parts(content_parts)
                        }
                    };
                    ChatMessage::User(UserMessage {
                        content,
                        name: u.name,
                    })
                }
                crate::provider::types::ChatMessage::Assistant(a) => {
                    let content = a.content.map(|c| match c {
                        crate::provider::types::MessageContent::Text(text) => MessageContent::Text(text),
                        crate::provider::types::MessageContent::Parts(parts) => {
                            let text = parts.into_iter().filter_map(|part| {
                                match part {
                                    crate::provider::types::ContentPart::Text { text } => Some(text),
                                    _ => None,
                                }
                            }).collect::<Vec<_>>().join(" ");
                            MessageContent::Text(text)
                        }
                    });
                    let tool_calls = a.tool_calls.map(|calls| {
                        calls.into_iter().map(|call| ToolCall {
                            id: call.id,
                            type_: "function".to_string(),
                            function: FunctionCall {
                                name: call.function.name,
                                arguments: call.function.arguments,
                            },
                        }).collect()
                    });
                    ChatMessage::Assistant(AssistantMessage {
                        content,
                        name: a.name,
                        tool_calls,
                        refusal: a.refusal,
                    })
                }
                crate::provider::types::ChatMessage::Tool(t) => {
                    let content = match t.content {
                        crate::provider::types::MessageContent::Text(text) => text,
                        crate::provider::types::MessageContent::Parts(parts) => {
                            parts.into_iter().filter_map(|part| {
                                match part {
                                    crate::provider::types::ContentPart::Text { text } => Some(text),
                                    _ => None,
                                }
                            }).collect::<Vec<_>>().join(" ")
                        }
                    };
                    ChatMessage::Tool(ToolMessage {
                        content: MessageContent::Text(content),
                        tool_call_id: t.tool_call_id,
                    })
                }
            }
        }).collect();

        // Convert tools
        let tools = req.tools.map(|tools| {
            tools.into_iter().map(|tool| Tool {
                type_: "function".to_string(),
                function: FunctionDefinition {
                    name: tool.function.name,
                    description: tool.function.description,
                    parameters: Some(tool.function.parameters),
                    strict: tool.function.strict,
                },
            }).collect()
        });

        // Convert tool choice
        let tool_choice = req.tool_choice.map(|choice| match choice {
            crate::provider::types::ToolChoice::Auto(s) => ToolChoice::String(s),
            crate::provider::types::ToolChoice::Named(obj) => ToolChoice::Object(ToolChoiceObject {
                type_: "function".to_string(),
                function: ToolChoiceFunction {
                    name: obj.function.name,
                },
            }),
        });

        // Convert response format
        let response_format = req.response_format.map(|format| match format {
            crate::provider::types::ResponseFormat::Text => ResponseFormat::Text,
            crate::provider::types::ResponseFormat::JsonObject => ResponseFormat::JsonObject,
            crate::provider::types::ResponseFormat::JsonSchema { json_schema } => {
                ResponseFormat::JsonSchema {
                    json_schema: JsonSchemaConfig {
                        name: json_schema.name,
                        description: json_schema.description,
                        schema: json_schema.schema,
                        strict: json_schema.strict,
                    },
                }
            }
        });

        // Convert stop
        let stop = req.stop.map(|stop| match stop {
            crate::provider::types::Stop::Single(s) => Stop::String(s),
            crate::provider::types::Stop::Multiple(arr) => Stop::Array(arr),
        });

        ChatRequest {
            messages,
            model: req.model,
            models: None,
            response_format,
            stop,
            stream: req.stream,
            stream_options: None,
            max_tokens: req.max_tokens,
            max_completion_tokens: None,
            temperature: req.temperature,
            top_p: req.top_p,
            top_k: None,
            frequency_penalty: req.frequency_penalty,
            presence_penalty: req.presence_penalty,
            repetition_penalty: None,
            seed: req.seed,
            tools,
            tool_choice,
            logit_bias: req.logit_bias,
            logprobs: req.logprobs,
            top_logprobs: req.top_logprobs,
            user: req.user,
            session_id: None,
            metadata: None,
            provider: None,
            plugins: None,
            reasoning: None,
            trace: None,
        }
    }
}

// Convert from openrouter::ChatResponse to types::ChatResponse
impl From<ChatResponse> for crate::provider::types::ChatResponse {
    fn from(resp: ChatResponse) -> Self {
        let choices = resp.choices.into_iter().map(|choice| {
            let message = {
                let content = choice.message.content.map(|c| match c {
                    MessageContent::Text(text) => crate::provider::types::MessageContent::Text(text),
                    MessageContent::Parts(parts) => {
                        let text = parts.into_iter().filter_map(|part| {
                            match part {
                                ContentPart::Text { text, .. } => Some(text),
                                _ => None,
                            }
                        }).collect::<Vec<_>>().join(" ");
                        crate::provider::types::MessageContent::Text(text)
                    }
                });
                let tool_calls = choice.message.tool_calls.map(|calls| {
                    calls.into_iter().map(|call| crate::provider::types::ToolCall {
                        id: call.id,
                        type_: crate::provider::types::ToolType::Function,
                        function: crate::provider::types::FunctionCall {
                            name: call.function.name,
                            arguments: call.function.arguments,
                        },
                    }).collect()
                });
                crate::provider::types::AssistantMessage {
                    content,
                    name: choice.message.name,
                    tool_calls,
                    refusal: choice.message.refusal,
                }
            };

            let finish_reason = match choice.finish_reason {
                Some(ChatCompletionFinishReason::ToolCalls) => crate::provider::types::FinishReason::ToolCalls,
                Some(ChatCompletionFinishReason::Stop) => crate::provider::types::FinishReason::Stop,
                Some(ChatCompletionFinishReason::Length) => crate::provider::types::FinishReason::Length,
                Some(ChatCompletionFinishReason::ContentFilter) => crate::provider::types::FinishReason::ContentFilter,
                Some(ChatCompletionFinishReason::Error) => crate::provider::types::FinishReason::Error,
                None => crate::provider::types::FinishReason::Stop,
            };

            let logprobs = choice.logprobs.map(|lp| {
                crate::provider::types::Logprobs {
                    content: lp.content.map(|content| {
                        content.into_iter().map(|c| {
                            crate::provider::types::TokenLogprob {
                                token: c.token,
                                logprob: c.logprob,
                                bytes: c.bytes,
                                top_logprobs: c.top_logprobs.map(|top| {
                                    top.into_iter().map(|t| {
                                        crate::provider::types::TopLogprob {
                                            token: t.token,
                                            logprob: t.logprob,
                                            bytes: t.bytes,
                                        }
                                    }).collect()
                                }),
                            }
                        }).collect()
                    }),
                }
            });

            crate::provider::types::ChatChoice {
                index: choice.index,
                message,
                finish_reason,
                logprobs,
            }
        }).collect();

        let usage = crate::provider::types::Usage {
            prompt_tokens: resp.usage.prompt_tokens,
            completion_tokens: resp.usage.completion_tokens,
            total_tokens: resp.usage.total_tokens,
            prompt_tokens_details: resp.usage.prompt_tokens_details.map(|d| {
                crate::provider::types::PromptTokensDetails {
                    cached_tokens: d.cached_tokens,
                    audio_tokens: d.audio_tokens,
                }
            }),
            completion_tokens_details: resp.usage.completion_tokens_details.map(|d| {
                crate::provider::types::CompletionTokensDetails {
                    reasoning_tokens: d.reasoning_tokens,
                    audio_tokens: d.audio_tokens,
                }
            }),
        };

        crate::provider::types::ChatResponse {
            id: resp.id,
            object: resp.object,
            created: resp.created,
            model: resp.model,
            choices,
            usage: Some(usage),
            system_fingerprint: resp.system_fingerprint,
        }
    }
}

// Convert from types::EmbeddingRequest to openrouter::EmbeddingsRequest
impl From<crate::provider::types::EmbeddingRequest> for EmbeddingsRequest {
    fn from(req: crate::provider::types::EmbeddingRequest) -> Self {
        // Convert input
        let input = match req.input {
            crate::provider::types::EmbeddingInput::String(s) => EmbeddingInput::String(s),
            crate::provider::types::EmbeddingInput::StringArray(arr) => EmbeddingInput::StringArray(arr),
        };

        // Convert encoding format
        let encoding_format = req.encoding_format.map(|fmt| match fmt {
            crate::provider::types::EmbeddingEncodingFormat::Float => EmbeddingEncodingFormat::Float,
            crate::provider::types::EmbeddingEncodingFormat::Base64 => EmbeddingEncodingFormat::Base64,
        });

        EmbeddingsRequest {
            input,
            model: req.model,
            encoding_format,
            dimensions: req.dimensions,
            user: req.user,
            provider: None,
            input_type: None,
        }
    }
}

// Convert from openrouter::EmbeddingsResponse to types::EmbeddingResponse
impl From<EmbeddingsResponse> for crate::provider::types::EmbeddingResponse {
    fn from(resp: EmbeddingsResponse) -> Self {
        let data = resp.data.into_iter().map(|embedding| {
            let embedding_data = match embedding.embedding {
                EmbeddingData::Array(vec) => vec,
                EmbeddingData::Base64(_) => Vec::new(), // Skip base64 encoded embeddings
            };
            crate::provider::types::EmbeddingObject {
                index: embedding.index,
                embedding: embedding_data,
                object: embedding.object,
            }
        }).collect();

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

// ==========================================
// OpenRouter Provider Implementation
// ==========================================

use crate::provider::{call_chat_completions, call_embedding};

/// OpenRouter provider struct
pub struct OpenrouterProvider {
    /// API key for OpenRouter
    api_key: String,
    /// Model name (e.g., "openai/gpt-4o-mini")
    model: String,
    /// Base URL for OpenRouter API (default: "https://openrouter.ai/api/v1")
    base_url: String,
}

impl OpenrouterProvider {
    /// Create a new OpenRouter provider
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            base_url: "https://openrouter.ai/api/v1".to_string(),
        }
    }

    /// Create a new OpenRouter provider with custom base URL
    pub fn new_with_base_url(api_key: String, model: String, base_url: String) -> Self {
        Self {
            api_key,
            model,
            base_url,
        }
    }
}

#[async_trait::async_trait]
impl crate::provider::Provider for OpenrouterProvider {
    fn name(&self) -> &str {
        "openrouter"
    }

    async fn chat(
        &self,
        mut request: crate::provider::types::ChatRequest,
    ) -> anyhow::Result<crate::provider::types::ChatResponse> {
        request.model = Some(self.model.clone());
        let url = format!("{}/chat/completions", self.base_url);
        call_chat_completions::<ChatRequest, ChatResponse>(
            &url,
            &self.api_key,
            request,
        )
        .await
    }

    async fn embedding(
        &self,
        request: crate::provider::types::EmbeddingRequest,
    ) -> anyhow::Result<crate::provider::types::EmbeddingResponse> {
        let url = format!("{}/embeddings", self.base_url);
        call_embedding::<EmbeddingsRequest, EmbeddingsResponse>(
            &url,
            &self.api_key,
            request,
        )
        .await
    }

    fn capabilities(&self) -> crate::provider::types::ProviderCapabilities {
        // OpenRouter supports a wide variety of models
        // Default to chat + streaming, specific capabilities depend on model
        let mut capabilities = crate::provider::types::ProviderCapabilities::CHAT
            | crate::provider::types::ProviderCapabilities::STREAMING;

        let model_lower = self.model.to_lowercase();

        // Most models support function calling
        capabilities |= crate::provider::types::ProviderCapabilities::FUNCTION_CALLING;

        // Vision models
        if model_lower.contains("vision") || model_lower.contains("gpt-4")
            || model_lower.contains("claude-3") || model_lower.contains("gemini") {
            capabilities |= crate::provider::types::ProviderCapabilities::VISION;
        }

        // JSON mode models
        if model_lower.contains("json") {
            capabilities |= crate::provider::types::ProviderCapabilities::JSON_MODE;
        }

        capabilities
    }

    fn max_context_length(&self) -> usize {
        let model_lower = self.model.to_lowercase();

        // Claude 3 models
        if model_lower.contains("claude-3") {
            if model_lower.contains("opus") || model_lower.contains("sonnet") {
                200000
            } else {
                100000
            }
        // GPT-4 models
        } else if model_lower.contains("gpt-4") {
            if model_lower.contains("turbo") {
                128000
            } else if model_lower.contains("32k") {
                32768
            } else {
                8192
            }
        // GPT-3.5 models
        } else if model_lower.contains("gpt-3.5") {
            if model_lower.contains("16k") {
                16384
            } else {
                4096
            }
        } else {
            // Default context length for unknown models
            8192
        }
    }
}
