//! OpenAI-compatible API implementation
//!
//! This module provides types and provider implementation for OpenAI-compatible APIs
//! based on the OpenAPI specification in docs/provider/compatible.yaml.

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

/// Create chat completion request
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionRequest {
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

/// Create chat completion response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionResponse {
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

/// OpenAI-compatible provider
pub struct OpenaiCompatibleProvider {
    name: String,
    model: String,
    endpoint: String,
    api_key: String,
}

impl OpenaiCompatibleProvider {
    /// Create a new OpenAI-compatible provider
    pub fn new(
        name: impl Into<String>,
        model: impl Into<String>,
        endpoint: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            model: model.into(),
            endpoint: endpoint.into(),
            api_key: api_key.into(),
        }
    }

    /// Helper method to convert unified ChatRequest to OpenAI-compatible request
    fn convert_chat_request(&self, request: crate::provider::types::ChatRequest) -> ChatCompletionRequest {
        // Convert messages
        let messages = request.messages.into_iter().map(|msg| {
            ChatMessage {
                role: match msg.role {
                    crate::provider::types::Role::System => Role::System,
                    crate::provider::types::Role::User => Role::User,
                    crate::provider::types::Role::Assistant => Role::Assistant,
                    crate::provider::types::Role::Tool => Role::Tool,
                },
                content: msg.content,
                name: msg.name,
                tool_calls: msg.tool_calls.map(|calls| {
                    calls.into_iter().map(|call| {
                        ToolCall {
                            id: call.id,
                            tool_type: call.tool_type,
                            function: ToolCallFunction {
                                name: call.function.name,
                                arguments: call.function.arguments,
                            },
                        }
                    }).collect()
                }),
                tool_call_id: msg.tool_call_id,
            }
        }).collect();

        // Convert tools
        let tools = request.tools.map(|tools| {
            tools.into_iter().map(|tool| {
                Tool {
                    tool_type: tool.tool_type,
                    function: FunctionDefinition {
                        name: tool.function.name,
                        description: tool.function.description,
                        parameters: tool.function.parameters,
                        strict: tool.function.strict,
                    },
                }
            }).collect()
        });

        // Convert tool choice
        let tool_choice = request.tool_choice.map(|choice| match choice {
            crate::provider::types::ToolChoice::String(s) => ToolChoice::String(s),
            crate::provider::types::ToolChoice::Object(obj) => ToolChoice::Object(NamedToolChoice {
                tool_type: obj.tool_type,
                function: NamedToolChoiceFunction {
                    name: obj.function.name,
                },
            }),
        });

        // Convert response format
        let response_format = request.response_format.map(|format| match format {
            crate::provider::types::ResponseFormat::Text => ResponseFormat::Text,
            crate::provider::types::ResponseFormat::JsonObject => ResponseFormat::JsonObject,
            crate::provider::types::ResponseFormat::JsonSchema { json_schema } => ResponseFormat::JsonSchema {
                json_schema: JsonSchemaFormat {
                    name: json_schema.name,
                    description: json_schema.description,
                    schema: json_schema.schema,
                    strict: json_schema.strict,
                },
            },
        });

        // Convert stop
        let stop = request.stop.map(|stop| match stop {
            crate::provider::types::Stop::Single(s) => Stop::Single(s),
            crate::provider::types::Stop::Multiple(v) => Stop::Multiple(v),
        });

        // Convert stream options
        let stream_options = request.stream_options.map(|opts| StreamOptions {
            include_usage: opts.include_usage,
        });

        ChatCompletionRequest {
            messages,
            model: request.model,
            temperature: request.temperature,
            top_p: request.top_p,
            stream: request.stream,
            tools,
            tool_choice,
            max_tokens: request.max_tokens,
            n: request.n,
            stop,
            frequency_penalty: request.frequency_penalty,
            presence_penalty: request.presence_penalty,
            logit_bias: request.logit_bias,
            logprobs: request.logprobs,
            top_logprobs: request.top_logprobs,
            seed: request.seed,
            user: request.user,
            response_format,
            parallel_tool_calls: request.parallel_tool_calls,
            stream_options,
        }
    }

    /// Helper method to convert OpenAI-compatible response to unified ChatResponse
    fn convert_chat_response(&self, response: ChatCompletionResponse) -> crate::provider::types::ChatResponse {
        // Convert choices
        let choices = response.choices.into_iter().map(|choice| {
            crate::provider::types::ChatChoice {
                index: choice.index,
                message: crate::provider::types::ChatMessage {
                    role: match choice.message.role {
                        Role::System => crate::provider::types::Role::System,
                        Role::User => crate::provider::types::Role::User,
                        Role::Assistant => crate::provider::types::Role::Assistant,
                        Role::Tool => crate::provider::types::Role::Tool,
                    },
                    content: choice.message.content,
                    name: choice.message.name,
                    tool_calls: choice.message.tool_calls.map(|calls| {
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
                    }),
                    tool_call_id: choice.message.tool_call_id,
                },
                finish_reason: match choice.finish_reason {
                    FinishReason::Stop => crate::provider::types::FinishReason::Stop,
                    FinishReason::Length => crate::provider::types::FinishReason::Length,
                    FinishReason::ToolCalls => crate::provider::types::FinishReason::ToolCalls,
                    FinishReason::ContentFilter => crate::provider::types::FinishReason::ContentFilter,
                    FinishReason::FunctionCall => crate::provider::types::FinishReason::FunctionCall,
                },
                logprobs: choice.logprobs.map(|logprobs| {
                    crate::provider::types::Logprobs {
                        content: logprobs.content.map(|content| {
                            content.into_iter().map(|item| {
                                crate::provider::types::LogprobContent {
                                    token: item.token,
                                    logprob: item.logprob,
                                    bytes: item.bytes,
                                    top_logprobs: item.top_logprobs.map(|top_logprobs| {
                                        top_logprobs.into_iter().map(|top| {
                                            crate::provider::types::TopLogprob {
                                                token: top.token,
                                                logprob: top.logprob,
                                                bytes: top.bytes,
                                            }
                                        }).collect()
                                    }),
                                }
                            }).collect()
                        }),
                    }
                }),
            }
        }).collect();

        // Convert usage
        let usage = response.usage.map(|usage| crate::provider::types::Usage {
            prompt_tokens: usage.prompt_tokens,
            completion_tokens: usage.completion_tokens,
            total_tokens: usage.total_tokens,
        });

        crate::provider::types::ChatResponse {
            id: response.id,
            object: response.object,
            created: response.created,
            model: response.model,
            choices,
            usage,
            system_fingerprint: response.system_fingerprint,
        }
    }
}

#[async_trait::async_trait]
impl crate::provider::Provider for OpenaiCompatibleProvider {
    fn name(&self) -> &str {
        &self.name
    }

    async fn chat(&self, request: crate::provider::types::ChatRequest) -> anyhow::Result<crate::provider::types::ChatResponse> {
        // Convert unified request to OpenAI-compatible request
        let _openai_request = self.convert_chat_request(request);
        
        // TODO: Implement actual API call
        // For now, return a mock response
        Ok(crate::provider::types::ChatResponse {
            id: "mock-id".to_string(),
            object: "chat.completion".to_string(),
            created: 1234567890,
            model: self.model.clone(),
            choices: vec![crate::provider::types::ChatChoice {
                index: 0,
                message: crate::provider::types::ChatMessage {
                    role: crate::provider::types::Role::Assistant,
                    content: Some("This is a mock response. Actual API call not implemented yet.".to_string()),
                    name: None,
                    tool_calls: None,
                    tool_call_id: None,
                },
                finish_reason: crate::provider::types::FinishReason::Stop,
                logprobs: None,
            }],
            usage: Some(crate::provider::types::Usage {
                prompt_tokens: 10,
                completion_tokens: 20,
                total_tokens: 30,
            }),
            system_fingerprint: None,
        })
    }

    async fn embedding(&self, request: crate::provider::types::EmbeddingRequest) -> anyhow::Result<crate::provider::types::EmbeddingResponse> {
        // TODO: Implement actual API call
        // For now, return a mock response
        Ok(crate::provider::types::EmbeddingResponse {
            data: vec![crate::provider::types::EmbeddingObject {
                index: 0,
                embedding: vec![0.1; 1536], // Mock embedding vector
                object: "embedding".to_string(),
            }],
            model: self.model.clone(),
            object: "list".to_string(),
            usage: crate::provider::types::EmbeddingUsage {
                prompt_tokens: request.input.len() as u32 * 10,
                total_tokens: request.input.len() as u32 * 10,
            },
        })
    }

    fn capabilities(&self) -> crate::provider::types::ModelCapabilities {
        // Determine capabilities based on model name
        let model_lower = self.model.to_lowercase();
        let mut capabilities = crate::provider::types::ModelCapabilities::TEXT_GENERATION;
        
        if model_lower.contains("gpt-4") || model_lower.contains("gpt-3.5") {
            capabilities |= crate::provider::types::ModelCapabilities::FUNCTION_CALLING;
        }
        
        if model_lower.contains("vision") || model_lower.contains("gpt-4-vision") {
            capabilities |= crate::provider::types::ModelCapabilities::VISION;
        }
        
        if model_lower.contains("whisper") || model_lower.contains("audio") {
            capabilities |= crate::provider::types::ModelCapabilities::AUDIO_INPUT;
        }
        
        if model_lower.contains("json") {
            capabilities |= crate::provider::types::ModelCapabilities::OBJECT_GENERATION;
        }
        
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
        } else if model_lower.contains("gpt-3.5") {
            if model_lower.contains("16k") {
                16384
            } else {
                4096
            }
        } else if model_lower.contains("claude") {
            if model_lower.contains("100k") {
                100000
            } else if model_lower.contains("200k") {
                200000
            } else {
                100000
            }
        } else {
            // Default context length
            4096
        }
    }
}
