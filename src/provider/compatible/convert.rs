//! TryFrom trait implementations for type conversion
//!
//! This module provides conversions between OpenAI-compatible API types and provider-agnostic types.
//! Conversions that may fail use TryFrom and return anyhow::Error.

use super::types::*;
use crate::provider::types::{
    AssistantMessage as TypesAssistantMessage, ChatChoice as TypesChatChoice,
    ChatMessage as TypesChatMessage, ChatRequest as TypesChatRequest,
    ChatResponse as TypesChatResponse, ContentPart as TypesContentPart,
    FinishReason as TypesFinishReason,
    FunctionCall as TypesFunctionCall, Logprobs as TypesLogprobs,
    MessageContent as TypesMessageContent, TokenLogprob as TypesTokenLogprob,
    TopLogprob as TypesTopLogprob, ToolCall as TypesToolCall,
    ToolChoice as TypesToolChoice, ToolType as TypesToolType, Usage as TypesUsage,
};
use anyhow::Result;
use std::convert::TryFrom;

// Convert from types::ChatRequest to compatible::ChatCompletionRequest
impl TryFrom<TypesChatRequest> for ChatCompletionRequest {
    type Error = anyhow::Error;

    fn try_from(req: TypesChatRequest) -> Result<Self> {
        let messages = req
            .messages
            .into_iter()
            .map(|msg| {
                let (role, content, tool_calls, tool_call_id) = match msg {
                    TypesChatMessage::System(s) => {
                        let text = match s.content {
                            TypesMessageContent::Text(t) => Some(t),
                            TypesMessageContent::Parts(parts) => {
                                let mut text_parts = Vec::new();
                                for part in parts {
                                    match part {
                                        TypesContentPart::Text { text } => {
                                            text_parts.push(text);
                                        }
                                        TypesContentPart::ImageUrl { .. } => {
                                            anyhow::bail!("OpenAI-compatible provider does not support image input")
                                        }
                                        TypesContentPart::InputAudio { .. } => {
                                            anyhow::bail!("OpenAI-compatible provider does not support audio input")
                                        }
                                        TypesContentPart::InputVideo { .. } => {
                                            anyhow::bail!("OpenAI-compatible provider does not support video input")
                                        }
                                        TypesContentPart::Document { .. } => {
                                            anyhow::bail!("OpenAI-compatible provider does not support document input")
                                        }
                                    }
                                }
                                if !text_parts.is_empty() {
                                    Some(text_parts.join("\n"))
                                } else {
                                    None
                                }
                            }
                        };
                        (Role::System, text, None, None)
                    }
                    TypesChatMessage::User(u) => {
                        let text = match u.content {
                            TypesMessageContent::Text(t) => Some(t),
                            TypesMessageContent::Parts(parts) => {
                                let mut text_parts = Vec::new();
                                for part in parts {
                                    match part {
                                        TypesContentPart::Text { text } => {
                                            text_parts.push(text);
                                        }
                                        TypesContentPart::ImageUrl { .. } => {
                                            anyhow::bail!("OpenAI-compatible provider does not support image input")
                                        }
                                        TypesContentPart::InputAudio { .. } => {
                                            anyhow::bail!("OpenAI-compatible provider does not support audio input")
                                        }
                                        TypesContentPart::InputVideo { .. } => {
                                            anyhow::bail!("OpenAI-compatible provider does not support video input")
                                        }
                                        TypesContentPart::Document { .. } => {
                                            anyhow::bail!("OpenAI-compatible provider does not support document input")
                                        }
                                    }
                                }
                                if !text_parts.is_empty() {
                                    Some(text_parts.join("\n"))
                                } else {
                                    None
                                }
                            }
                        };
                        (Role::User, text, None, None)
                    }
                    TypesChatMessage::Assistant(a) => {
                        let text = match a.content {
                            Some(TypesMessageContent::Text(t)) => Some(t),
                            Some(TypesMessageContent::Parts(parts)) => {
                                let mut text_parts = Vec::new();
                                for part in parts {
                                    match part {
                                        TypesContentPart::Text { text } => {
                                            text_parts.push(text);
                                        }
                                        TypesContentPart::ImageUrl { .. } => {
                                            anyhow::bail!("OpenAI-compatible provider does not support image input in assistant message")
                                        }
                                        TypesContentPart::InputAudio { .. } => {
                                            anyhow::bail!("OpenAI-compatible provider does not support audio input in assistant message")
                                        }
                                        TypesContentPart::InputVideo { .. } => {
                                            anyhow::bail!("OpenAI-compatible provider does not support video input in assistant message")
                                        }
                                        TypesContentPart::Document { .. } => {
                                            anyhow::bail!("OpenAI-compatible provider does not support document input in assistant message")
                                        }
                                    }
                                }
                                Some(text_parts.join("\n"))
                            }
                            None => None,
                        };
                        (Role::Assistant, text, a.tool_calls, None)
                    }
                    TypesChatMessage::Tool(t) => {
                        let text = match t.content {
                            TypesMessageContent::Text(t) => Some(t),
                            TypesMessageContent::Parts(parts) => {
                                let mut text_parts = Vec::new();
                                for part in parts {
                                    if let TypesContentPart::Text { text } = part {
                                        text_parts.push(text);
                                    }
                                }
                                if !text_parts.is_empty() {
                                    Some(text_parts.join("\n"))
                                } else {
                                    None
                                }
                            }
                        };
                        (Role::Tool, text, None, Some(t.tool_call_id))
                    }
                };

                let converted_tool_calls = tool_calls.map(|calls| {
                    calls
                        .into_iter()
                        .map(|call| ToolCall {
                            id: call.id,
                            tool_type: "function".to_string(),
                            function: ToolCallFunction {
                                name: call.function.name,
                                arguments: call.function.arguments,
                            },
                        })
                        .collect()
                });

                Ok(ChatMessage {
                    role,
                    content,
                    name: None,
                    tool_calls: converted_tool_calls,
                    tool_call_id,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let tools = req.tools.map(|tools| {
            tools
                .into_iter()
                .map(|tool| Tool {
                    tool_type: "function".to_string(),
                    function: FunctionDefinition {
                        name: tool.function.name,
                        description: tool.function.description,
                        parameters: tool.function.parameters,
                        strict: tool.function.strict,
                    },
                })
                .collect()
        });

        let tool_choice = req.tool_choice.map(|choice| match choice {
            TypesToolChoice::Auto(s) => ToolChoice::String(s),
            TypesToolChoice::Named(obj) => ToolChoice::Object(NamedToolChoice {
                tool_type: "function".to_string(),
                function: NamedToolChoiceFunction {
                    name: obj.function.name,
                },
            }),
        });

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

        let stop = req.stop.map(|stop| match stop {
            crate::provider::types::Stop::Single(s) => Stop::Single(s),
            crate::provider::types::Stop::Multiple(v) => Stop::Multiple(v),
        });

        Ok(ChatCompletionRequest {
            messages,
            model: req.model.unwrap_or_default(),
            temperature: req.temperature,
            top_p: req.top_p,
            stream: req.stream,
            tools,
            tool_choice,
            max_tokens: req.max_tokens.map(|v| v as i32),
            n: req.n.map(|v| v as i32),
            stop,
            frequency_penalty: req.frequency_penalty,
            presence_penalty: req.presence_penalty,
            logit_bias: req.logit_bias.map(|lb| lb.into_iter().map(|(k, v)| (k, v as i32)).collect()),
            logprobs: req.logprobs,
            top_logprobs: req.top_logprobs.map(|v| v as i32),
            seed: req.seed,
            user: req.user,
            response_format,
            parallel_tool_calls: req.parallel_tool_calls,
            stream_options: None,
        })
    }
}

// Convert from compatible::ChatCompletionResponse to types::ChatResponse
impl From<ChatCompletionResponse> for TypesChatResponse {
    fn from(resp: ChatCompletionResponse) -> Self {
        let choices = resp
            .choices
            .into_iter()
            .map(|choice| {
                let content = choice
                    .message
                    .content
                    .map(TypesMessageContent::Text);

                let tool_calls = choice.message.tool_calls.map(|calls| {
                    calls
                        .into_iter()
                        .map(|call| TypesToolCall {
                            id: call.id,
                            type_: TypesToolType::Function,
                            function: TypesFunctionCall {
                                name: call.function.name,
                                arguments: call.function.arguments,
                            },
                        })
                        .collect()
                });

                let finish_reason = match choice.finish_reason {
                    FinishReason::Stop => TypesFinishReason::Stop,
                    FinishReason::Length => TypesFinishReason::Length,
                    FinishReason::ToolCalls => TypesFinishReason::ToolCalls,
                    FinishReason::ContentFilter => TypesFinishReason::ContentFilter,
                    FinishReason::FunctionCall => TypesFinishReason::FunctionCall,
                };

                let logprobs = choice.logprobs.map(|logprobs| TypesLogprobs {
                    content: logprobs.content.map(|content| {
                        content
                            .into_iter()
                            .map(|item| TypesTokenLogprob {
                                token: item.token,
                                logprob: item.logprob,
                                bytes: item.bytes.map(|b| b.into_iter().map(|v| v as u8).collect()),
                                top_logprobs: item.top_logprobs.map(|top_logprobs| {
                                    top_logprobs
                                        .into_iter()
                                        .map(|top| TypesTopLogprob {
                                            token: top.token,
                                            logprob: top.logprob,
                                            bytes: top.bytes.map(|b| b.into_iter().map(|v| v as u8).collect()),
                                        })
                                        .collect()
                                }),
                            })
                            .collect()
                    }),
                });

                TypesChatChoice {
                    index: choice.index as u32,
                    message: TypesAssistantMessage {
                        content,
                        name: choice.message.name,
                        tool_calls,
                        refusal: None,
                    },
                    finish_reason,
                    logprobs,
                }
            })
            .collect();

        let usage = resp.usage.map(|usage| TypesUsage {
            prompt_tokens: usage.prompt_tokens as u32,
            completion_tokens: usage.completion_tokens as u32,
            total_tokens: usage.total_tokens as u32,
            prompt_tokens_details: None,
            completion_tokens_details: None,
        });

        TypesChatResponse {
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
