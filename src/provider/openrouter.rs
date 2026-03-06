#![allow(unused)]
use crate::provider::{call_chat_completions, call_embedding};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
// ==========================================
// /chat/completions
// ==========================================

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ChatRequest {
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Stop>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repetition_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transforms: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<Provider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum Message {
    System(SystemMessage),
    User(UserMessage),
    Assistant(AssistantMessage),
    Tool(ToolResponseMessage),
    Developer(DeveloperMessage),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemMessage {
    pub content: MessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserMessage {
    pub content: MessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolResponseMessage {
    pub content: MessageContent,
    pub tool_call_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeveloperMessage {
    pub content: MessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    Parts(Vec<ContentPart>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {
    Text { text: String },
    ImageUrl { image_url: ImageUrl },
    AudioUrl { audio_url: AudioUrl },
    VideoUrl { video_url: VideoUrl },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioUrl {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoUrl {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub function: FunctionCall,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ResponseFormat {
    Text,
    JsonObject,
    JsonSchema { json_schema: Value },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Stop {
    String(String),
    Array(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tool {
    #[serde(rename = "type")]
    pub type_: String,
    pub function: FunctionDefinition,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ToolChoice {
    String(String),
    Object(ToolChoiceObject),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolChoiceObject {
    #[serde(rename = "type")]
    pub type_: String,
    pub function: ToolChoiceFunction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolChoiceFunction {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Provider {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<Box<Provider>>,
}

// ==========================================
// /chat/completions (Response)
// ==========================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatResponse {
    pub id: String,
    pub choices: Vec<ChatResponseChoice>,
    pub created: u64,
    pub model: String,
    pub object: String, // "chat.completion"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatResponseChoice {
    pub index: u32,
    pub message: AssistantMessage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

// ==========================================
// /responses
// ==========================================

#[derive(Serialize, Deserialize, Debug, Clone)]
struct OpenResponsesRequest {
    pub request: ChatRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<ChatResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct OpenResponsesNonStreamingResponse {
    pub request: ChatRequest,
    pub response: ChatResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnthropicMessagesResponse {
    pub id: String,
    pub model: String,
    pub usage: Usage,
    pub content: Vec<ContentPart>,
    pub stop_reason: Option<String>,
}

// ==========================================
// /embeddings
// ==========================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingsRequest {
    pub input: Value,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<Provider>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub object: String, // "list"
    pub data: Vec<Embedding>,
    pub model: String,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Embedding {
    pub object: String, // "embedding"
    pub index: u32,
    pub embedding: Vec<f32>,
}

// ==========================================
// /generation
// ==========================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenerationResponse {
    pub data: GenerationData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenerationData {
    pub id: String,
    pub total_cost: f64,
    pub model: String,
}

// ==========================================
// /credits
// ==========================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreditsResponse {
    pub credits: f64,
}

// ==========================================
// /charges
// ==========================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateChargeResponse {
    pub id: String,
    pub amount: f64,
    pub currency: String,
    pub status: String,
}

// ==========================================
// /models
// ==========================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelListResponse {
    pub data: Vec<Model>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub pricing: Pricing,
    pub context_length: u32,
    pub architecture: Architecture,
    pub top_provider: TopProvider,
    pub per_request_limits: Option<PerRequestLimits>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pricing {
    pub prompt: f64,
    pub completion: f64,
    pub image: f64,
    pub request: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Architecture {
    pub modality: String,
    pub tokenizer: String,
    pub instruct_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopProvider {
    pub id: String,
    pub provider: Provider,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PerRequestLimits {
    pub prompt_tokens: String,
    pub completion_tokens: String,
}

// ==========================================
// /keys
// ==========================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyListResponse {
    pub data: Vec<Key>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Key {
    pub id: String,
    pub name: String,
    pub created: u64,
    pub last_used: Option<u64>,
    pub scopes: Vec<String>,
}

// ==========================================
// /activity
// ==========================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityResponse {
    pub data: Vec<Activity>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Activity {
    pub id: String,
    pub created: u64,
    pub model: String,
    pub provider: String,
    pub cost: f64,
    pub request_tokens: u32,
    pub response_tokens: u32,
}

// ==========================================
// From trait implementations for conversion
// ==========================================

impl From<crate::provider::types::EmbeddingRequest> for EmbeddingsRequest {
    fn from(req: crate::provider::types::EmbeddingRequest) -> Self {
        // Convert input from EmbeddingInput enum to OpenRouter format
        let input = match req.input {
            crate::provider::types::EmbeddingInput::String(s) => {
                serde_json::Value::String(s)
            }
            crate::provider::types::EmbeddingInput::StringArray(arr) => {
                serde_json::Value::Array(arr.into_iter().map(serde_json::Value::String).collect())
            }
        };

        // Convert encoding format
        let encoding_format = req.encoding_format.map(|fmt| match fmt {
            crate::provider::types::EmbeddingEncodingFormat::Float => "float".to_string(),
            crate::provider::types::EmbeddingEncodingFormat::Base64 => "base64".to_string(),
        });

        EmbeddingsRequest {
            input,
            model: req.model,
            encoding_format,
            dimensions: req.dimensions.map(|d| d as u32),
            user: req.user,
            provider: None,
        }
    }
}

// ==========================================
// OpenRouter Provider Implementation
// ==========================================

/// OpenRouter provider struct
pub struct OpenrouterProvider {
    /// API key for OpenRouter
    api_key: String,
    /// Model name (e.g., "openai/gpt-4", "anthropic/claude-3-opus")
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
        call_chat_completions::<ChatRequest, ChatResponse>(&url, &self.api_key, request).await
    }

    async fn embedding(
        &self,
        request: crate::provider::types::EmbeddingRequest,
    ) -> anyhow::Result<crate::provider::types::EmbeddingResponse> {
        let url = format!("{}/embeddings", self.base_url);
        call_embedding::<EmbeddingsRequest, EmbeddingsResponse>(&url, &self.api_key, request).await
    }

    fn capabilities(&self) -> crate::provider::types::ProviderCapabilities {
        // OpenRouter supports many models with different capabilities
        let model_lower = self.model.to_lowercase();
        let mut capabilities = crate::provider::types::ProviderCapabilities::CHAT;

        // Most models on OpenRouter support function calling
        if model_lower.contains("gpt")
            || model_lower.contains("claude")
            || model_lower.contains("gemini")
        {
            capabilities |= crate::provider::types::ProviderCapabilities::FUNCTION_CALLING;
        }

        // Check for vision capabilities
        if model_lower.contains("vision")
            || model_lower.contains("gpt-4-vision")
            || model_lower.contains("claude-3")
        {
            capabilities |= crate::provider::types::ProviderCapabilities::VISION;
        }

        // Check for audio capabilities
        if model_lower.contains("whisper") || model_lower.contains("audio") {
            capabilities |= crate::provider::types::ProviderCapabilities::AUDIO_INPUT;
        }

        // Check for JSON/object generation
        if model_lower.contains("json") {
            capabilities |= crate::provider::types::ProviderCapabilities::JSON_MODE;
        }

        // OpenRouter supports streaming
        capabilities |= crate::provider::types::ProviderCapabilities::STREAMING;

        capabilities
    }

    fn max_context_length(&self) -> usize {
        // Return context length based on model
        let model_lower = self.model.to_lowercase();

        if model_lower.contains("gpt-4") {
            if model_lower.contains("32k") {
                32768
            } else if model_lower.contains("128k") {
                131072
            } else {
                8192
            }
        } else if model_lower.contains("claude-3") {
            if model_lower.contains("200k") {
                200000
            } else {
                100000
            }
        } else if model_lower.contains("gemini") {
            if model_lower.contains("1.5") {
                1000000 // Gemini 1.5 has 1M context
            } else {
                32768
            }
        } else if model_lower.contains("gpt-3.5") {
            if model_lower.contains("16k") {
                16384
            } else {
                4096
            }
        } else {
            // Default context length
            4096
        }
    }
}

impl From<crate::provider::types::ChatRequest> for ChatRequest {
    fn from(req: crate::provider::types::ChatRequest) -> Self {
        // Convert messages from new enum-based ChatMessage
        let messages = req.messages.into_iter().map(|msg| {
            match msg {
                crate::provider::types::ChatMessage::System(s) => {
                    let content = match s.content {
                        crate::provider::types::MessageContent::Text(text) => {
                            MessageContent::Text(text)
                        }
                        crate::provider::types::MessageContent::Parts(parts) => {
                            let mut openrouter_parts = Vec::new();
                            for part in parts {
                                match part {
                                    crate::provider::types::ContentPart::Text { text } => {
                                        openrouter_parts.push(ContentPart::Text { text });
                                    }
                                    crate::provider::types::ContentPart::ImageUrl { image_url } => {
                                        openrouter_parts.push(ContentPart::ImageUrl {
                                            image_url: ImageUrl {
                                                url: image_url.url,
                                                detail: image_url.detail.map(|d| match d {
                                                    crate::provider::types::ImageDetail::Low => "low".to_string(),
                                                    crate::provider::types::ImageDetail::High => "high".to_string(),
                                                    crate::provider::types::ImageDetail::Auto => "auto".to_string(),
                                                }),
                                            }
                                        });
                                    }
                                    _ => {}
                                }
                            }
                            MessageContent::Parts(openrouter_parts)
                        }
                    };

                    Message::System(SystemMessage {
                        content,
                        name: s.name,
                    })
                }
                crate::provider::types::ChatMessage::User(u) => {
                    let content = match u.content {
                        crate::provider::types::MessageContent::Text(text) => {
                            MessageContent::Text(text)
                        }
                        crate::provider::types::MessageContent::Parts(parts) => {
                            let mut openrouter_parts = Vec::new();
                            for part in parts {
                                match part {
                                    crate::provider::types::ContentPart::Text { text } => {
                                        openrouter_parts.push(ContentPart::Text { text });
                                    }
                                    crate::provider::types::ContentPart::ImageUrl { image_url } => {
                                        openrouter_parts.push(ContentPart::ImageUrl {
                                            image_url: ImageUrl {
                                                url: image_url.url,
                                                detail: image_url.detail.map(|d| match d {
                                                    crate::provider::types::ImageDetail::Low => "low".to_string(),
                                                    crate::provider::types::ImageDetail::High => "high".to_string(),
                                                    crate::provider::types::ImageDetail::Auto => "auto".to_string(),
                                                }),
                                            }
                                        });
                                    }
                                    _ => {}
                                }
                            }
                            MessageContent::Parts(openrouter_parts)
                        }
                    };

                    Message::User(UserMessage {
                        content,
                        name: u.name,
                    })
                }
                crate::provider::types::ChatMessage::Assistant(a) => {
                    let content = a.content.map(|c| match c {
                        crate::provider::types::MessageContent::Text(text) => {
                            MessageContent::Text(text)
                        }
                        crate::provider::types::MessageContent::Parts(parts) => {
                            let mut text_parts = Vec::new();
                            for part in parts {
                                match part {
                                    crate::provider::types::ContentPart::Text { text } => {
                                        text_parts.push(text);
                                    }
                                    _ => {}
                                }
                            }
                            MessageContent::Text(text_parts.join(" "))
                        }
                    });

                    let tool_calls = a.tool_calls.map(|calls| {
                        calls.into_iter().map(|call| {
                            ToolCall {
                                id: call.id,
                                type_: "function".to_string(),
                                function: FunctionCall {
                                    name: call.function.name,
                                    arguments: call.function.arguments,
                                },
                            }
                        }).collect()
                    });

                    Message::Assistant(AssistantMessage {
                        content,
                        name: a.name,
                        tool_calls,
                        refusal: None,
                    })
                }
                crate::provider::types::ChatMessage::Tool(t) => {
                    let content = match t.content {
                        crate::provider::types::MessageContent::Text(text) => {
                            MessageContent::Text(text)
                        }
                        crate::provider::types::MessageContent::Parts(parts) => {
                            let mut text_parts = Vec::new();
                            for part in parts {
                                match part {
                                    crate::provider::types::ContentPart::Text { text } => {
                                        text_parts.push(text);
                                    }
                                    _ => {}
                                }
                            }
                            MessageContent::Text(text_parts.join(" "))
                        }
                    };

                    Message::Tool(ToolResponseMessage {
                        content,
                        tool_call_id: t.tool_call_id,
                    })
                }
            }
        }).collect();

        // Convert tools
        let tools = req.tools.map(|tools| {
            tools
                .into_iter()
                .map(|tool| Tool {
                    type_: "function".to_string(),
                    function: FunctionDefinition {
                        name: tool.function.name,
                        description: tool.function.description,
                        parameters: Some(tool.function.parameters),
                        strict: tool.function.strict,
                    },
                })
                .collect()
        });

        // Convert tool choice
        let tool_choice = req.tool_choice.map(|choice| match choice {
            crate::provider::types::ToolChoice::Auto(s) => ToolChoice::String(s),
            crate::provider::types::ToolChoice::Named(obj) => {
                ToolChoice::Object(ToolChoiceObject {
                    type_: "function".to_string(),
                    function: ToolChoiceFunction {
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
                    json_schema: serde_json::json!({
                        "name": json_schema.name,
                        "description": json_schema.description,
                        "schema": json_schema.schema,
                        "strict": json_schema.strict,
                    }),
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
            max_tokens: req.max_tokens.map(|t| t as u32),
            max_completion_tokens: req.max_tokens.map(|t| t as u32),
            temperature: req.temperature,
            top_p: req.top_p,
            top_k: None,
            frequency_penalty: req.frequency_penalty,
            presence_penalty: req.presence_penalty,
            repetition_penalty: None,
            seed: req.seed,
            tools,
            tool_choice,
            logit_bias: req
                .logit_bias
                .map(|bias| bias.into_iter().map(|(k, v)| (k, v as f32)).collect()),
            logprobs: req.logprobs,
            top_logprobs: req.top_logprobs.map(|t| t as u32),
            user: req.user,
            transforms: None,
            route: None,
            provider: None,
            plugins: None,
            session_id: None,
            metadata: None,
        }
    }
}

impl From<ChatResponse> for crate::provider::types::ChatResponse {
    fn from(resp: ChatResponse) -> Self {
        // Convert choices
        let choices = resp
            .choices
            .into_iter()
            .map(|choice| {
                // Convert message - use new AssistantMessage type
                let message = {
                    // Convert content to new MessageContent type
                    let content = choice.message.content.map(|c| match c {
                        MessageContent::Text(text) => crate::provider::types::MessageContent::Text(text),
                        MessageContent::Parts(parts) => {
                            let mut content_parts = Vec::new();
                            for part in parts {
                                match part {
                                    ContentPart::Text { text } => {
                                        content_parts.push(crate::provider::types::ContentPart::Text { text });
                                    }
                                    ContentPart::ImageUrl { image_url } => {
                                        content_parts.push(crate::provider::types::ContentPart::ImageUrl {
                                            image_url: crate::provider::types::ImageUrl {
                                                url: image_url.url,
                                                detail: image_url.detail.map(|d| {
                                                    match d.as_str() {
                                                        "low" => crate::provider::types::ImageDetail::Low,
                                                        "high" => crate::provider::types::ImageDetail::High,
                                                        "auto" => crate::provider::types::ImageDetail::Auto,
                                                        _ => crate::provider::types::ImageDetail::Auto,
                                                    }
                                                }),
                                            },
                                        });
                                    }
                                    _ => {}
                                }
                            }
                            crate::provider::types::MessageContent::Parts(content_parts)
                        }
                    });

                    // Convert tool calls
                    let tool_calls = choice.message.tool_calls.map(|calls| {
                        calls
                            .into_iter()
                            .map(|call| crate::provider::types::ToolCall {
                                id: call.id,
                                type_: crate::provider::types::ToolType::Function,
                                function: crate::provider::types::FunctionCall {
                                    name: call.function.name,
                                    arguments: call.function.arguments,
                                },
                            })
                            .collect()
                    });

                    crate::provider::types::AssistantMessage {
                        content,
                        name: choice.message.name,
                        tool_calls,
                        refusal: choice.message.refusal,
                    }
                };

                // Convert finish reason
                let finish_reason = choice.finish_reason.map(|r| match r.as_str() {
                    "stop" => crate::provider::types::FinishReason::Stop,
                    "length" => crate::provider::types::FinishReason::Length,
                    "tool_calls" => crate::provider::types::FinishReason::ToolCalls,
                    "content_filter" => crate::provider::types::FinishReason::ContentFilter,
                    "function_call" => crate::provider::types::FinishReason::FunctionCall,
                    _ => crate::provider::types::FinishReason::Stop,
                }).unwrap_or(crate::provider::types::FinishReason::Stop);

                crate::provider::types::ChatChoice {
                    index: choice.index as u32,
                    message,
                    finish_reason,
                    logprobs: None,
                }
            })
            .collect();

        // Convert usage
        let usage = resp.usage.map(|u| crate::provider::types::Usage {
            prompt_tokens: u.prompt_tokens as u32,
            completion_tokens: u.completion_tokens as u32,
            total_tokens: u.total_tokens as u32,
            prompt_tokens_details: None,
            completion_tokens_details: None,
        });

        crate::provider::types::ChatResponse {
            id: resp.id,
            object: resp.object,
            created: resp.created as u64,
            model: resp.model,
            choices,
            usage,
            system_fingerprint: resp.system_fingerprint,
        }
    }
}

impl From<EmbeddingsResponse> for crate::provider::types::EmbeddingResponse {
    fn from(resp: EmbeddingsResponse) -> Self {
        // Convert data
        let data = resp
            .data
            .into_iter()
            .map(|embedding| crate::provider::types::EmbeddingObject {
                index: embedding.index as u32,
                embedding: embedding.embedding,
                object: embedding.object,
            })
            .collect();

        // Convert usage
        let usage = crate::provider::types::EmbeddingUsage {
            prompt_tokens: resp.usage.prompt_tokens as u32,
            total_tokens: resp.usage.total_tokens as u32,
        };

        crate::provider::types::EmbeddingResponse {
            object: resp.object,
            data,
            model: resp.model,
            usage,
        }
    }
}
