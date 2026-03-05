use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// ==========================================
// /chat/completions
// ==========================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatRequest {
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

    // OpenRouter specific fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<ProviderPreferences>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Vec<Plugin>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transforms: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>, // Deprecated but present in spec
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
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
pub struct DeveloperMessage {
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
    InputAudio { input_audio: InputAudio },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputAudio {
    pub data: String,
    pub format: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String, // usually "function"
    pub function: FunctionCall,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
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
    String(String), // "none", "auto", "required"
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
#[serde(untagged)]
pub enum Stop {
    String(String),
    Array(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseFormat {
    Text,
    JsonObject,
    JsonSchema { json_schema: Value },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProviderPreferences {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_fallbacks: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_parameters: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_collection: Option<String>, // "deny", "allow"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantizations: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zdr: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_distillable_text: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Plugin {
    Value(Value),
}

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
pub struct OpenResponsesRequest {
    pub input: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tool_calls: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_config: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenResponsesNonStreamingResponse {
    pub id: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// ==========================================
// /messages
// ==========================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnthropicMessagesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    pub messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<Value>, // string or array
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnthropicMessage {
    pub role: String,
    pub content: Value, // string or array
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnthropicMessagesResponse {
    pub id: String,
    pub r#type: String,
    pub role: String,
    pub content: Vec<Value>,
    pub model: String,
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
    pub usage: AnthropicUsage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnthropicUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

// ==========================================
// /embeddings
// ==========================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingsRequest {
    pub input: Value, // string or array
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<ProviderPreferences>,
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
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// ==========================================
// /credits
// ==========================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreditsResponse {
    pub data: CreditsData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreditsData {
    pub total_credits: f64,
    pub total_usage: f64,
}

// ==========================================
// /credits/coinbase
// ==========================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateChargeRequest {
    pub amount: f64,
    pub sender: String,
    pub chain_id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateChargeResponse {
    #[serde(flatten)]
    pub data: Value,
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
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// ==========================================
// /auth/keys
// ==========================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyListResponse {
    pub data: Vec<Key>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Key {
    pub label: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// ==========================================
// /activity
// ==========================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityResponse {
    pub data: Vec<ActivityItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityItem {
    pub date: String,
    pub model: String,
    pub model_permaslug: String,
    pub endpoint_id: String,
    pub provider_name: String,
    pub usage: f64,
    pub byok_usage_inference: f64,
    pub requests: u32,
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub reasoning_tokens: u32,
}

// ==========================================
// From trait implementations for conversion
// ==========================================

impl From<ChatRequest> for crate::provider::types::ChatRequest {
    fn from(req: ChatRequest) -> Self {
        // Convert messages
        let messages = req
            .messages
            .into_iter()
            .map(|msg| {
                match msg {
                    Message::System(system_msg) => {
                        // Extract text from MessageContent
                        let content = match system_msg.content {
                            MessageContent::Text(text) => {
                                Some(crate::provider::types::Content::Text(text))
                            }
                            MessageContent::Parts(parts) => {
                                // Extract text from parts
                                let mut text_parts = Vec::new();
                                for part in parts {
                                    match part {
                                        ContentPart::Text { text } => {
                                            text_parts.push(text);
                                        }
                                        _ => {
                                            // Skip non-text parts
                                        }
                                    }
                                }
                                if !text_parts.is_empty() {
                                    Some(crate::provider::types::Content::Text(
                                        text_parts.join(" "),
                                    ))
                                } else {
                                    None
                                }
                            }
                        };

                        crate::provider::types::ChatMessage {
                            role: crate::provider::types::Role::System,
                            content,
                            name: system_msg.name,
                            tool_calls: None,
                            tool_call_id: None,
                        }
                    }
                    Message::User(user_msg) => {
                        // Convert content to string
                        let content = match user_msg.content {
                            MessageContent::Text(text) => {
                                Some(crate::provider::types::Content::Text(text))
                            }
                            MessageContent::Parts(parts) => {
                                // Convert parts to unified ContentPart format
                                let mut content_parts = Vec::new();
                                for part in parts {
                                    match part {
                                        ContentPart::Text { text } => {
                                            content_parts.push(
                                                crate::provider::types::ContentPart::Text { text },
                                            );
                                        }
                                        // TODO: Support other content types (images, audio, video)
                                        _ => {
                                            // Skip unsupported content types for now
                                        }
                                    }
                                }
                                if !content_parts.is_empty() {
                                    Some(crate::provider::types::Content::Parts(content_parts))
                                } else {
                                    None
                                }
                            }
                        };

                        crate::provider::types::ChatMessage {
                            role: crate::provider::types::Role::User,
                            content,
                            name: user_msg.name,
                            tool_calls: None,
                            tool_call_id: None,
                        }
                    }
                    Message::Assistant(assistant_msg) => {
                        // Convert content to unified Content type
                        let content = assistant_msg.content.and_then(|c| match c {
                            MessageContent::Text(text) => {
                                Some(crate::provider::types::Content::Text(text))
                            }
                            MessageContent::Parts(parts) => {
                                // Convert parts to unified ContentPart format
                                let mut content_parts = Vec::new();
                                for part in parts {
                                    match part {
                                        ContentPart::Text { text } => {
                                            content_parts.push(
                                                crate::provider::types::ContentPart::Text { text },
                                            );
                                        }
                                        // TODO: Support other content types (images, audio, video)
                                        _ => {
                                            // Skip unsupported content types for now
                                        }
                                    }
                                }
                                if !content_parts.is_empty() {
                                    Some(crate::provider::types::Content::Parts(content_parts))
                                } else {
                                    None
                                }
                            }
                        });

                        // Convert tool calls
                        let converted_tool_calls = assistant_msg.tool_calls.map(|calls| {
                            calls
                                .into_iter()
                                .map(|call| crate::provider::types::ToolCall {
                                    id: call.id,
                                    tool_type: call.type_,
                                    function: crate::provider::types::ToolCallFunction {
                                        name: call.function.name,
                                        arguments: call.function.arguments,
                                    },
                                })
                                .collect()
                        });

                        crate::provider::types::ChatMessage {
                            role: crate::provider::types::Role::Assistant,
                            content,
                            name: assistant_msg.name,
                            tool_calls: converted_tool_calls,
                            tool_call_id: None,
                        }
                    }
                    Message::Tool(tool_msg) => {
                        // Extract text from MessageContent
                        let content = match tool_msg.content {
                            MessageContent::Text(text) => {
                                Some(crate::provider::types::Content::Text(text))
                            }
                            MessageContent::Parts(parts) => {
                                // Extract text from parts
                                let mut text_parts = Vec::new();
                                for part in parts {
                                    match part {
                                        ContentPart::Text { text } => {
                                            text_parts.push(text);
                                        }
                                        _ => {
                                            // Skip non-text parts
                                        }
                                    }
                                }
                                if !text_parts.is_empty() {
                                    Some(crate::provider::types::Content::Text(
                                        text_parts.join(" "),
                                    ))
                                } else {
                                    None
                                }
                            }
                        };

                        crate::provider::types::ChatMessage {
                            role: crate::provider::types::Role::Tool,
                            content,
                            name: None,
                            tool_calls: None,
                            tool_call_id: Some(tool_msg.tool_call_id),
                        }
                    }
                    Message::Developer(dev_msg) => {
                        // Extract text from MessageContent
                        let content = match dev_msg.content {
                            MessageContent::Text(text) => {
                                Some(crate::provider::types::Content::Text(text))
                            }
                            MessageContent::Parts(parts) => {
                                // Extract text from parts
                                let mut text_parts = Vec::new();
                                for part in parts {
                                    match part {
                                        ContentPart::Text { text } => {
                                            text_parts.push(text);
                                        }
                                        _ => {
                                            // Skip non-text parts
                                        }
                                    }
                                }
                                if !text_parts.is_empty() {
                                    Some(crate::provider::types::Content::Text(
                                        text_parts.join(" "),
                                    ))
                                } else {
                                    None
                                }
                            }
                        };

                        crate::provider::types::ChatMessage {
                            role: crate::provider::types::Role::System, // Treat developer as system
                            content,
                            name: dev_msg.name,
                            tool_calls: None,
                            tool_call_id: None,
                        }
                    }
                }
            })
            .collect();

        // Convert tools
        let tools = req.tools.map(|tools| {
            tools
                .into_iter()
                .map(|tool| crate::provider::types::Tool {
                    tool_type: tool.type_,
                    function: crate::provider::types::FunctionDefinition {
                        name: tool.function.name,
                        description: tool.function.description,
                        parameters: tool.function.parameters.unwrap_or_default(),
                        strict: tool.function.strict,
                    },
                })
                .collect()
        });

        // Convert tool choice
        let tool_choice = req.tool_choice.map(|choice| match choice {
            ToolChoice::String(s) => crate::provider::types::ToolChoice::String(s),
            ToolChoice::Object(obj) => crate::provider::types::ToolChoice::Object(
                crate::provider::types::NamedToolChoice {
                    tool_type: obj.type_,
                    function: crate::provider::types::NamedToolChoiceFunction {
                        name: obj.function.name,
                    },
                },
            ),
        });

        // Convert response format
        let response_format = req.response_format.map(|format| match format {
            ResponseFormat::Text => crate::provider::types::ResponseFormat::Text,
            ResponseFormat::JsonObject => crate::provider::types::ResponseFormat::JsonObject,
            ResponseFormat::JsonSchema { json_schema } => {
                // Extract fields from JSON value
                let name = json_schema
                    .get("name")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                let description = json_schema
                    .get("description")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let schema = json_schema.get("schema").cloned().unwrap_or_default();
                let strict = json_schema.get("strict").and_then(|v| v.as_bool());

                crate::provider::types::ResponseFormat::JsonSchema {
                    json_schema: crate::provider::types::JsonSchemaFormat {
                        name,
                        description,
                        schema,
                        strict,
                    },
                }
            }
        });

        // Convert stop
        let stop = req.stop.map(|stop| match stop {
            Stop::String(s) => crate::provider::types::Stop::Single(s),
            Stop::Array(arr) => crate::provider::types::Stop::Multiple(arr),
        });

        // Use model from request or default to first model from models list
        let model = req.model.unwrap_or_else(|| {
            req.models
                .and_then(|models| models.into_iter().next())
                .unwrap_or_else(|| "unknown".to_string())
        });

        crate::provider::types::ChatRequest {
            messages,
            model,
            temperature: req.temperature,
            top_p: req.top_p,
            stream: req.stream,
            tools,
            tool_choice,
            max_tokens: req
                .max_tokens
                .or(req.max_completion_tokens)
                .map(|v| v as i32),
            n: None, // OpenRouter doesn't have n parameter
            stop,
            frequency_penalty: req.frequency_penalty,
            presence_penalty: req.presence_penalty,
            logit_bias: req
                .logit_bias
                .map(|map| map.into_iter().map(|(k, v)| (k, v as i32)).collect()),
            logprobs: req.logprobs,
            top_logprobs: req.top_logprobs.map(|v| v as i32),
            seed: req.seed,
            user: req.user,
            response_format,
            parallel_tool_calls: None, // OpenRouter doesn't have this parameter
            stream_options: None,      // OpenRouter doesn't have stream options
        }
    }
}

impl From<EmbeddingsRequest> for crate::provider::types::EmbeddingRequest {
    fn from(req: EmbeddingsRequest) -> Self {
        // Convert input to Vec<String>
        let input = match req.input {
            serde_json::Value::String(text) => vec![text],
            serde_json::Value::Array(arr) => {
                arr.into_iter()
                    .filter_map(|val| match val {
                        serde_json::Value::String(s) => Some(s),
                        _ => None, // Skip non-string values
                    })
                    .collect()
            }
            _ => vec![], // Empty for other types
        };

        // Convert encoding format
        let encoding_format = req.encoding_format.map(|fmt| {
            if fmt.to_lowercase() == "base64" {
                crate::provider::types::EmbeddingEncodingFormat::Base64
            } else {
                crate::provider::types::EmbeddingEncodingFormat::Float
            }
        });

        crate::provider::types::EmbeddingRequest {
            input,
            model: req.model,
            encoding_format,
            dimensions: req.dimensions.map(|v| v as i32),
            user: req.user,
        }
    }
}
