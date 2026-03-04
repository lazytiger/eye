use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeepSeekRequest {
    /// A list of messages comprising the conversation so far.
    pub messages: Vec<Message>,

    /// ID of the model to use.
    pub model: DeepSeekModel,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,

    /// The maximum number of tokens to generate in the completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    /// An object specifying the format that the model must output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,

    /// Up to 4 sequences where the API will stop generating further tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Stop>,

    /// If set, partial message deltas will be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Options for streaming response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,

    /// What sampling temperature to use, between 0 and 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// A list of tools the model may call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,

    /// Controls which (if any) function is called by the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,

    /// Whether to return log probabilities of the output tokens or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,

    /// An integer between 0 and 20 specifying the number of most likely tokens to return at each position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u8>,

    /// Configuration for the "thinking" feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<Thinking>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DeepSeekModel {
    #[serde(rename = "deepseek-chat")]
    DeepSeekChat,
    #[serde(rename = "deepseek-reasoner")]
    DeepSeekReasoner,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Stop {
    String(String),
    Array(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub type_: ResponseFormatType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormatType {
    Text,
    JsonObject,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_usage: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tool {
    #[serde(rename = "type")]
    pub type_: ToolType,
    pub function: Function,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ToolType {
    Function,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Function {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ToolChoice {
    Mode(ToolChoiceMode),
    Specific(ToolChoiceSpecific),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ToolChoiceMode {
    None,
    Auto,
    Required,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolChoiceSpecific {
    #[serde(rename = "type")]
    pub type_: ToolType,
    pub function: FunctionName,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionName {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Thinking {
    #[serde(rename = "type")]
    pub type_: ThinkingType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ThinkingType {
    Disabled,
    Enabled,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum Message {
    System {
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    User {
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    Assistant {
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<ToolCall>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        prefix: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reasoning_content: Option<String>,
    },
    Tool {
        content: String,
        tool_call_id: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: ToolType,
    pub function: FunctionCall,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeepSeekResponse {
    pub id: String,
    pub choices: Vec<Choice>,
    pub created: u64,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
    pub object: String, // chat.completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Choice {
    pub finish_reason: Option<String>,
    pub index: u32,
    pub message: Message,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<LogProbs>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogProbs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<LogProbContent>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogProbContent {
    pub token: String,
    pub logprob: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
    pub top_logprobs: Vec<TopLogProb>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopLogProb {
    pub token: String,
    pub logprob: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    pub completion_tokens: u32,
    pub prompt_tokens: u32,
    pub total_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens_details: Option<CompletionTokensDetails>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionTokensDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_tokens: Option<u32>,
}
