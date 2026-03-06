use crate::provider::call_chat_completions;
use serde::{Deserialize, Serialize};
use serde_json::Value;
// ==========================================
// /chat/completions
// ==========================================

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DeepSeekRequest {
    pub messages: Vec<Message>,
    pub model: DeepSeekModel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Stop>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<Thinking>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum DeepSeekModel {
    #[serde(rename = "deepseek-chat")]
    DeepSeekChat,
    #[serde(rename = "deepseek-reasoner")]
    DeepSeekReasoner,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "role", rename_all = "snake_case")]
enum Message {
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
        prefix: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reasoning_content: Option<String>,
    },
    Tool {
        tool_call_id: String,
        content: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ResponseFormat {
    #[serde(rename = "type")]
    pub type_: ResponseFormatType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
enum ResponseFormatType {
    Text,
    JsonObject,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
enum Stop {
    String(String),
    Array(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct StreamOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_usage: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Tool {
    #[serde(rename = "type")]
    pub type_: ToolType,
    pub function: Function,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
enum ToolType {
    Function,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Function {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Value>,
    // strict field not supported by DeepSeek
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: ToolType,
    pub function: FunctionCall,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
enum ToolChoice {
    Mode(ToolChoiceMode),
    Specific(ToolChoiceSpecific),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
enum ToolChoiceMode {
    None,
    Auto,
    Required,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ToolChoiceSpecific {
    #[serde(rename = "type")]
    pub type_: ToolType,
    pub function: FunctionName,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FunctionName {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Thinking {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_tokens: Option<u32>,
}

// ==========================================
// /chat/completions (Response)
// ==========================================

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DeepSeekResponse {
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
struct Choice {
    pub finish_reason: Option<String>,
    pub index: u32,
    pub message: Message,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<LogProbs>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LogProbs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<LogProbContent>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LogProbContent {
    pub token: String,
    pub logprob: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
    pub top_logprobs: Vec<TopLogProb>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TopLogProb {
    pub token: String,
    pub logprob: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Usage {
    pub completion_tokens: u32,
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}

// ==========================================
// From trait implementations for conversion
// ==========================================

impl From<crate::provider::types::ChatRequest> for DeepSeekRequest {
    fn from(req: crate::provider::types::ChatRequest) -> Self {
        // Convert messages
        let messages = req
            .messages
            .into_iter()
            .map(|msg| {
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
                                            // DeepSeek doesn't support multimodal input
                                        }
                                    }
                                }
                                text_parts.join(" ")
                            }
                            None => String::new(),
                        };

                        Message::System {
                            content,
                            name: msg.name,
                        }
                    }
                    crate::provider::types::Role::User => {
                        // Extract text content from user message
                        // DeepSeek doesn't support multimodal input, so we extract text only
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
                                            // Skip non-text parts
                                            // DeepSeek doesn't support multimodal input
                                        }
                                    }
                                }
                                text_parts.join(" ")
                            }
                            None => String::new(),
                        };

                        Message::User {
                            content,
                            name: msg.name,
                        }
                    }
                    crate::provider::types::Role::Assistant => {
                        // Extract text content from assistant message
                        let content = msg.content.and_then(|c| match c {
                            crate::provider::types::Content::Text(text) => Some(text),
                            crate::provider::types::Content::Parts(parts) => {
                                // Extract text from parts
                                let mut text_parts = Vec::new();
                                for part in parts {
                                    match part {
                                        crate::provider::types::ContentPart::Text { text } => {
                                            text_parts.push(text);
                                        }
                                        _ => {
                                            // Skip non-text parts
                                        }
                                    }
                                }
                                if !text_parts.is_empty() {
                                    Some(text_parts.join(" "))
                                } else {
                                    None
                                }
                            }
                        });

                        // Convert tool calls
                        let tool_calls = msg.tool_calls.map(|calls| {
                            calls
                                .into_iter()
                                .map(|call| ToolCall {
                                    id: call.id,
                                    type_: ToolType::Function,
                                    function: FunctionCall {
                                        name: call.function.name,
                                        arguments: call.function.arguments,
                                    },
                                })
                                .collect()
                        });

                        Message::Assistant {
                            content,
                            name: msg.name,
                            tool_calls,
                            prefix: None,
                            reasoning_content: None,
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
                                            // Skip non-text parts
                                        }
                                    }
                                }
                                text_parts.join(" ")
                            }
                            None => String::new(),
                        };

                        Message::Tool {
                            tool_call_id: msg.tool_call_id.unwrap_or_default(),
                            content,
                        }
                    }
                }
            })
            .collect();

        // Convert tools
        let tools = req.tools.map(|tools| {
            tools
                .into_iter()
                .map(|tool| {
                    Tool {
                        type_: ToolType::Function,
                        function: Function {
                            name: tool.function.name,
                            description: tool.function.description,
                            parameters: Some(tool.function.parameters),
                            // strict field not supported by DeepSeek
                        },
                    }
                })
                .collect()
        });

        // Convert tool choice
        let tool_choice = req.tool_choice.map(|choice| match choice {
            crate::provider::types::ToolChoice::String(s) => match s.as_str() {
                "none" => ToolChoice::Mode(ToolChoiceMode::None),
                "auto" => ToolChoice::Mode(ToolChoiceMode::Auto),
                "required" => ToolChoice::Mode(ToolChoiceMode::Required),
                _ => ToolChoice::Mode(ToolChoiceMode::Auto), // Default to auto
            },
            crate::provider::types::ToolChoice::Object(obj) => {
                ToolChoice::Specific(ToolChoiceSpecific {
                    type_: ToolType::Function,
                    function: FunctionName {
                        name: obj.function.name,
                    },
                })
            }
        });

        // Convert response format
        let response_format = req.response_format.map(|format| match format {
            crate::provider::types::ResponseFormat::Text => ResponseFormat {
                type_: ResponseFormatType::Text,
            },
            crate::provider::types::ResponseFormat::JsonObject => ResponseFormat {
                type_: ResponseFormatType::JsonObject,
            },
            crate::provider::types::ResponseFormat::JsonSchema { json_schema: _ } => {
                // DeepSeek doesn't support JsonSchema, so we default to JsonObject
                ResponseFormat {
                    type_: ResponseFormatType::JsonObject,
                }
            }
        });

        // Convert stop
        let stop = req.stop.map(|stop| match stop {
            crate::provider::types::Stop::Single(s) => Stop::String(s),
            crate::provider::types::Stop::Multiple(arr) => Stop::Array(arr),
        });

        // Convert model (assuming DeepSeek model)
        let model = match req.model.as_str() {
            "deepseek-chat" => DeepSeekModel::DeepSeekChat,
            "deepseek-reasoner" => DeepSeekModel::DeepSeekReasoner,
            _ => DeepSeekModel::DeepSeekChat, // Default
        };

        DeepSeekRequest {
            messages,
            model,
            frequency_penalty: req.frequency_penalty,
            max_tokens: req.max_tokens.map(|t| t as u64),
            presence_penalty: req.presence_penalty,
            response_format,
            stop,
            stream: req.stream,
            stream_options: req.stream_options.map(|options| StreamOptions {
                include_usage: options.include_usage,
            }),
            temperature: req.temperature,
            top_p: req.top_p,
            tools,
            tool_choice,
            // logit_bias not supported by DeepSeek
            logprobs: req.logprobs,
            top_logprobs: req.top_logprobs.map(|t| t as u8),
            // n, seed, user, parallel_tool_calls not supported by DeepSeek
            thinking: None,
        }
    }
}

// ==========================================
// DeepSeek Provider Implementation
// ==========================================

/// DeepSeek provider struct
pub struct DeepseekProvider {
    /// API key for DeepSeek
    api_key: String,
    /// Model name (e.g., "deepseek-chat", "deepseek-reasoner")
    model: String,
    /// Base URL for DeepSeek API (default: "https://api.deepseek.com")
    base_url: String,
}

impl DeepseekProvider {
    /// Create a new DeepSeek provider
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            base_url: "https://api.deepseek.com".to_string(),
        }
    }

    /// Create a new DeepSeek provider with custom base URL
    pub fn new_with_base_url(api_key: String, model: String, base_url: String) -> Self {
        Self {
            api_key,
            model,
            base_url,
        }
    }
}

#[async_trait::async_trait]
impl crate::provider::Provider for DeepseekProvider {
    fn name(&self) -> &str {
        "deepseek"
    }

    async fn chat(
        &self,
        mut request: crate::provider::types::ChatRequest,
    ) -> anyhow::Result<crate::provider::types::ChatResponse> {
        request.model = self.model.clone();
        let url = format!("{}/chat/completions", self.base_url);
        call_chat_completions::<DeepSeekRequest, DeepSeekResponse>(&url, &self.api_key, request)
            .await
    }

    async fn embedding(
        &self,
        _request: crate::provider::types::EmbeddingRequest,
    ) -> anyhow::Result<crate::provider::types::EmbeddingResponse> {
        // DeepSeek doesn't support embeddings API
        // Return an error or mock response
        Err(anyhow::anyhow!("DeepSeek does not support embeddings API"))
    }

    fn capabilities(&self) -> crate::provider::types::ModelCapabilities {
        // DeepSeek models have specific capabilities
        let _model_lower = self.model.to_lowercase();
        let mut capabilities = crate::provider::types::ModelCapabilities::TEXT_GENERATION;

        // DeepSeek models support function calling
        capabilities |= crate::provider::types::ModelCapabilities::FUNCTION_CALLING;

        // DeepSeek Reasoner has reasoning capabilities
        // Note: REASONING capability is not defined in ModelCapabilities
        // if model_lower.contains("reasoner") {
        //     capabilities |= crate::provider::types::ModelCapabilities::REASONING;
        // }

        // DeepSeek doesn't support vision, audio, or JSON object generation
        // (as of the knowledge cutoff date)

        capabilities
    }

    fn max_context_length(&self) -> usize {
        // DeepSeek models have 128K context length
        131072 // 128K tokens
    }
}

impl From<DeepSeekResponse> for crate::provider::types::ChatResponse {
    fn from(resp: DeepSeekResponse) -> Self {
        // Convert choices
        let choices = resp
            .choices
            .into_iter()
            .map(|choice| {
                // Convert message
                let message = match choice.message {
                    Message::Assistant {
                        content,
                        name,
                        tool_calls,
                        prefix: _,
                        reasoning_content: _,
                    } => {
                        // Convert content to unified Content type
                        let content =
                            content.map(|text| crate::provider::types::Content::Text(text));

                        // Convert tool calls
                        let tool_calls = tool_calls.map(|calls| {
                            calls
                                .into_iter()
                                .map(|call| crate::provider::types::ToolCall {
                                    id: call.id,
                                    tool_type: match call.type_ {
                                        ToolType::Function => "function".to_string(),
                                    },
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
                            name,
                            tool_calls,
                            tool_call_id: None,
                        }
                    }
                    _ => {
                        // Should not happen for assistant messages
                        crate::provider::types::ChatMessage {
                            role: crate::provider::types::Role::Assistant,
                            content: None,
                            name: None,
                            tool_calls: None,
                            tool_call_id: None,
                        }
                    }
                };

                // Convert finish reason (currently unused, but kept for future use)
                let _finish_reason = choice.finish_reason.map(|r| match r.as_str() {
                    "stop" => crate::provider::types::FinishReason::Stop,
                    "length" => crate::provider::types::FinishReason::Length,
                    "tool_calls" => crate::provider::types::FinishReason::ToolCalls,
                    "content_filter" => crate::provider::types::FinishReason::ContentFilter,
                    "function_call" => crate::provider::types::FinishReason::FunctionCall,
                    _ => crate::provider::types::FinishReason::Stop, // Default
                });

                // Convert logprobs
                let logprobs = choice
                    .logprobs
                    .map(|logprobs| crate::provider::types::Logprobs {
                        content: logprobs.content.map(|content| {
                            content
                                .into_iter()
                                .map(|c| crate::provider::types::LogprobContent {
                                    token: c.token,
                                    logprob: c.logprob,
                                    bytes: c
                                        .bytes
                                        .map(|bytes| bytes.into_iter().map(|b| b as i32).collect()),
                                    top_logprobs: Some(
                                        c.top_logprobs
                                            .into_iter()
                                            .map(|t| crate::provider::types::TopLogprob {
                                                token: t.token,
                                                logprob: t.logprob,
                                                bytes: t.bytes.map(|bytes| {
                                                    bytes.into_iter().map(|b| b as i32).collect()
                                                }),
                                            })
                                            .collect(),
                                    ),
                                })
                                .collect()
                        }),
                    });

                crate::provider::types::ChatChoice {
                    index: choice.index as i32,
                    message,
                    finish_reason: crate::provider::types::FinishReason::Stop,
                    logprobs,
                }
            })
            .collect();

        // Convert usage
        let usage = resp.usage.map(|u| crate::provider::types::Usage {
            prompt_tokens: u.prompt_tokens as i32,
            completion_tokens: u.completion_tokens as i32,
            total_tokens: u.total_tokens as i32,
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
