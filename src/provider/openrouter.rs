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
#[serde(untagged)]
pub enum ResponseFormat {
    Object {
        #[serde(rename = "type")]
        type_: String,
    }, // "text", "json_object"
    JsonSchema {
        #[serde(rename = "type")]
        type_: String,
        json_schema: Value,
    },
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
