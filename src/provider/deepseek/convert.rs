//! TryFrom trait implementations for type conversion
//!
//! This module provides conversions between DeepSeek API types and provider-agnostic types.
//! Conversions that may fail use TryFrom and return anyhow::Error.

use super::types::*;
use crate::provider::types::{
    AssistantMessage as TypesAssistantMessage, ChatChoice as TypesChatChoice,
    ChatMessage as TypesChatMessage, ChatRequest as TypesChatRequest,
    ChatResponse as TypesChatResponse, ContentPart as TypesContentPart,
    FinishReason, FunctionCall as TypesFunctionCall,
    Logprobs as TypesLogprobs, MessageContent as TypesMessageContent,
    TokenLogprob as TypesTokenLogprob, TopLogprob as TypesTopLogprob,
    ToolCall as TypesToolCall, ToolChoice as TypesToolChoice, ToolType as TypesToolType,
    Usage as TypesUsage,
};
use anyhow::Result;
use std::convert::TryFrom;

// Convert from types::ChatRequest to deepseek::ChatRequest
impl TryFrom<TypesChatRequest> for ChatRequest {
    type Error = anyhow::Error;

    fn try_from(req: TypesChatRequest) -> Result<Self> {
        let messages = req
            .messages
            .into_iter()
            .map(|msg| match msg {
                TypesChatMessage::System(s) => {
                    let content = match s.content {
                        TypesMessageContent::Text(text) => text,
                        TypesMessageContent::Parts(parts) => parts
                            .into_iter()
                            .filter_map(|part| match part {
                                TypesContentPart::Text { text } => Some(text),
                                _ => None,
                            })
                            .collect::<Vec<_>>()
                            .join(" "),
                    };
                    Ok(ChatMessage::System {
                        content,
                        name: s.name,
                    })
                }
                TypesChatMessage::User(u) => {
                    let content = match u.content {
                        TypesMessageContent::Text(text) => text,
                        TypesMessageContent::Parts(parts) => {
                            let mut texts = Vec::new();
                            for part in parts {
                                match part {
                                    TypesContentPart::Text { text } => texts.push(text),
                                    TypesContentPart::ImageUrl { .. } => {
                                        anyhow::bail!("DeepSeek provider does not support image input")
                                    }
                                    TypesContentPart::InputAudio { .. } => {
                                        anyhow::bail!("DeepSeek provider does not support audio input")
                                    }
                                    TypesContentPart::InputVideo { .. } => {
                                        anyhow::bail!("DeepSeek provider does not support video input")
                                    }
                                    TypesContentPart::Document { .. } => {
                                        anyhow::bail!("DeepSeek provider does not support document input")
                                    }
                                }
                            }
                            texts.join(" ")
                        }
                    };
                    Ok(ChatMessage::User {
                        content,
                        name: u.name,
                    })
                }
                TypesChatMessage::Assistant(a) => {
                    let content = a.content.map(|c| match c {
                        TypesMessageContent::Text(text) => text,
                        TypesMessageContent::Parts(parts) => parts
                            .into_iter()
                            .filter_map(|part| match part {
                                TypesContentPart::Text { text } => Some(text),
                                _ => None,
                            })
                            .collect::<Vec<_>>()
                            .join(" "),
                    });

                    let tool_calls = a.tool_calls.map(|calls| {
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

                    Ok(ChatMessage::Assistant {
                        content,
                        name: a.name,
                        tool_calls,
                        prefix: None,
                        reasoning_content: None,
                    })
                }
                TypesChatMessage::Tool(t) => {
                    let content = match t.content {
                        TypesMessageContent::Text(text) => text,
                        TypesMessageContent::Parts(parts) => parts
                            .into_iter()
                            .filter_map(|part| match part {
                                TypesContentPart::Text { text } => Some(text),
                                _ => None,
                            })
                            .collect::<Vec<_>>()
                            .join(" "),
                    };
                    Ok(ChatMessage::Tool {
                        tool_call_id: t.tool_call_id,
                        content,
                    })
                }
            })
            .collect::<Result<Vec<_>>>()?;

        let tools = req.tools.map(|tools| {
            tools
                .into_iter()
                .map(|tool| Tool {
                    type_: ToolType::Function,
                    function: Function {
                        name: tool.function.name,
                        description: tool.function.description,
                        parameters: Some(tool.function.parameters),
                    },
                })
                .collect()
        });

        let tool_choice = req.tool_choice.map(|choice| match choice {
            TypesToolChoice::Auto(s) => match s.as_str() {
                "none" => ToolChoice::Mode(ToolChoiceMode::None),
                "auto" => ToolChoice::Mode(ToolChoiceMode::Auto),
                "required" => ToolChoice::Mode(ToolChoiceMode::Required),
                _ => ToolChoice::Mode(ToolChoiceMode::Auto),
            },
            TypesToolChoice::Named(obj) => ToolChoice::Specific(ToolChoiceSpecific {
                type_: ToolType::Function,
                function: FunctionName {
                    name: obj.function.name,
                },
            }),
        });

        let response_format = req.response_format.map(|format| match format {
            crate::provider::types::ResponseFormat::Text => ResponseFormat {
                type_: ResponseFormatType::Text,
            },
            crate::provider::types::ResponseFormat::JsonObject => ResponseFormat {
                type_: ResponseFormatType::JsonObject,
            },
            crate::provider::types::ResponseFormat::JsonSchema { .. } => ResponseFormat {
                type_: ResponseFormatType::JsonObject,
            },
        });

        let stop = req.stop.map(|stop| match stop {
            crate::provider::types::Stop::Single(s) => Stop::String(s),
            crate::provider::types::Stop::Multiple(arr) => Stop::Array(arr),
        });

        let model = match req.model.as_deref().unwrap_or("deepseek-chat") {
            "deepseek-chat" => Model::DeepSeekChat,
            "deepseek-reasoner" => Model::DeepSeekReasoner,
            _ => Model::DeepSeekChat,
        };

        Ok(ChatRequest {
            messages,
            model,
            frequency_penalty: req.frequency_penalty,
            max_tokens: req.max_tokens.map(|t| t as u64),
            presence_penalty: req.presence_penalty,
            response_format,
            stop,
            stream: req.stream,
            stream_options: None,
            temperature: req.temperature,
            top_p: req.top_p,
            tools,
            tool_choice,
            logprobs: req.logprobs,
            top_logprobs: req.top_logprobs.map(|t| t as u8),
            thinking: None,
        })
    }
}

// Convert from deepseek::ChatResponse to types::ChatResponse
impl From<ChatResponse> for TypesChatResponse {
    fn from(resp: ChatResponse) -> Self {
        let choices = resp
            .choices
            .into_iter()
            .map(|choice| {
                let message = match choice.message {
                    ChatMessage::Assistant {
                        content,
                        name,
                        tool_calls,
                        ..
                    } => {
                        let content = content.map(TypesMessageContent::Text);

                        let tool_calls = tool_calls.map(|calls| {
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

                        TypesAssistantMessage {
                            content,
                            name,
                            tool_calls,
                            refusal: None,
                        }
                    }
                    _ => TypesAssistantMessage {
                        content: None,
                        name: None,
                        tool_calls: None,
                        refusal: None,
                    },
                };

                let finish_reason = choice
                    .finish_reason
                    .map(|r| match r.as_str() {
                        "stop" => FinishReason::Stop,
                        "length" => FinishReason::Length,
                        "tool_calls" => FinishReason::ToolCalls,
                        "content_filter" => FinishReason::ContentFilter,
                        "function_call" => FinishReason::FunctionCall,
                        _ => FinishReason::Stop,
                    })
                    .unwrap_or(FinishReason::Stop);

                let logprobs = choice.logprobs.map(|logprobs| TypesLogprobs {
                    content: logprobs.content.map(|content| {
                        content
                            .into_iter()
                            .map(|c| TypesTokenLogprob {
                                token: c.token,
                                logprob: c.logprob,
                                bytes: c.bytes
                                    .map(|bytes| bytes.into_iter().map(|b| b as u8).collect()),
                                top_logprobs: Some(
                                    c.top_logprobs
                                        .into_iter()
                                        .map(|t| TypesTopLogprob {
                                            token: t.token,
                                            logprob: t.logprob,
                                            bytes: t.bytes.map(|bytes| {
                                                bytes.into_iter().map(|b| b as u8).collect()
                                            }),
                                        })
                                        .collect(),
                                ),
                            })
                            .collect()
                    }),
                });

                TypesChatChoice {
                    index: choice.index as u32,
                    message,
                    finish_reason,
                    logprobs,
                }
            })
            .collect();

        let usage = resp.usage.map(|u| TypesUsage {
            prompt_tokens: u.prompt_tokens as u32,
            completion_tokens: u.completion_tokens as u32,
            total_tokens: u.total_tokens as u32,
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
