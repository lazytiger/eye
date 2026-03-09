//! TryFrom trait implementations for type conversion
//!
//! This module provides conversions between OpenAI API types and provider-agnostic types.
//! Conversions that may fail use TryFrom and return anyhow::Error.

use super::types::*;
use crate::provider::types::{
    AssistantMessage, ChatChoice, ChatMessage, ChatRequest, ChatResponse, ContentPart,
    EmbeddingEncodingFormat as TypesEmbeddingEncodingFormat, EmbeddingInput as TypesEmbeddingInput,
    EmbeddingRequest, EmbeddingResponse, FinishReason, FunctionCall, ImageDetail as TypesImageDetail,
    Logprobs, MessageContent, ResponseFormat as TypesResponseFormat, Stop as TypesStop,
    TokenLogprob, TopLogprob, ToolCall, ToolChoice as TypesToolChoice, ToolType, Usage,
};
use anyhow::Result;
use std::convert::TryFrom;

impl TryFrom<EmbeddingRequest> for CreateEmbeddingRequest {
    type Error = anyhow::Error;

    fn try_from(req: EmbeddingRequest) -> Result<Self> {
        let input = match req.input {
            TypesEmbeddingInput::String(s) => EmbeddingInput::Text(s),
            TypesEmbeddingInput::StringArray(arr) => EmbeddingInput::Array(arr),
        };

        let encoding_format = req.encoding_format.map(|fmt| match fmt {
            TypesEmbeddingEncodingFormat::Float => EmbeddingEncodingFormat::Float,
            TypesEmbeddingEncodingFormat::Base64 => EmbeddingEncodingFormat::Base64,
        });

        Ok(CreateEmbeddingRequest {
            input,
            model: req.model,
            encoding_format,
            dimensions: req.dimensions.map(|d| d as i32),
            user: req.user,
        })
    }
}

impl From<CreateEmbeddingResponse> for EmbeddingResponse {
    fn from(resp: CreateEmbeddingResponse) -> Self {
        let data = resp
            .data
            .into_iter()
            .map(|embedding| crate::provider::types::EmbeddingObject {
                index: embedding.index as u32,
                embedding: embedding.embedding,
                object: embedding.object,
            })
            .collect();

        let usage = crate::provider::types::EmbeddingUsage {
            prompt_tokens: resp.usage.prompt_tokens,
            total_tokens: resp.usage.total_tokens,
        };

        EmbeddingResponse {
            object: resp.object,
            data,
            model: resp.model,
            usage,
        }
    }
}

impl From<CreateChatCompletionResponse> for ChatResponse {
    fn from(resp: CreateChatCompletionResponse) -> Self {
        let choices = resp
            .choices
            .into_iter()
            .map(|choice| {
                let message = {
                    let content = choice
                        .message
                        .content
                        .map(|text| MessageContent::Text(text));

                    let tool_calls = choice.message.tool_calls.map(|calls| {
                        calls.into_iter()
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

                    AssistantMessage {
                        content,
                        name: None,
                        tool_calls,
                        refusal: choice.message.refusal,
                    }
                };

                let finish_reason = match choice.finish_reason.as_str() {
                    "stop" => FinishReason::Stop,
                    "length" => FinishReason::Length,
                    "tool_calls" => FinishReason::ToolCalls,
                    "content_filter" => FinishReason::ContentFilter,
                    "function_call" => FinishReason::FunctionCall,
                    _ => FinishReason::Stop,
                };

                let logprobs = choice.logprobs.map(|logprobs| Logprobs {
                    content: logprobs.content.map(|content| {
                        content
                            .into_iter()
                            .map(|c| TokenLogprob {
                                token: c.token,
                                logprob: c.logprob as f64,
                                bytes: None,
                                top_logprobs: Some(
                                    c.top_logprobs
                                        .into_iter()
                                        .map(|t| TopLogprob {
                                            token: t.token,
                                            logprob: t.logprob as f64,
                                            bytes: None,
                                        })
                                        .collect(),
                                ),
                            })
                            .collect()
                    }),
                });

                ChatChoice {
                    index: choice.index as u32,
                    message,
                    finish_reason,
                    logprobs,
                }
            })
            .collect();

        let usage = resp.usage.map(|u| Usage {
            prompt_tokens: u.prompt_tokens as u32,
            completion_tokens: u.completion_tokens as u32,
            total_tokens: u.total_tokens as u32,
            prompt_tokens_details: None,
            completion_tokens_details: None,
        });

        ChatResponse {
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

impl TryFrom<ChatRequest> for CreateChatCompletionRequest {
    type Error = anyhow::Error;

    fn try_from(req: ChatRequest) -> Result<Self> {
        let messages = req.messages.into_iter().map(|msg| {
            match msg {
                ChatMessage::System(s) => {
                    let content = match s.content {
                        MessageContent::Text(text) => text,
                        MessageContent::Parts(parts) => parts
                            .into_iter()
                            .filter_map(|part| match part {
                                ContentPart::Text { text } => Some(text),
                                _ => None,
                            })
                            .collect::<Vec<_>>()
                            .join(" "),
                    };
                    Ok(ChatCompletionRequestMessage::System {
                        content,
                        name: s.name,
                    })
                }
                ChatMessage::User(u) => {
                    let content = match u.content {
                        MessageContent::Text(text) => {
                            ChatCompletionRequestMessageContent::Text(text)
                        }
                        MessageContent::Parts(parts) => {
                            let mut openai_parts = Vec::new();
                            for part in parts {
                                match part {
                                    ContentPart::Text { text } => {
                                        openai_parts.push(ChatCompletionRequestMessageContentPart::Text { text });
                                    }
                                    ContentPart::ImageUrl { image_url } => {
                                        openai_parts.push(
                                            ChatCompletionRequestMessageContentPart::ImageUrl {
                                                image_url: ChatCompletionRequestMessageContentPartImageUrl {
                                                    url: image_url.url,
                                                    detail: image_url.detail.map(|d| match d {
                                                        TypesImageDetail::Low => ImageDetail::Low,
                                                        TypesImageDetail::High => ImageDetail::High,
                                                        TypesImageDetail::Auto => ImageDetail::Auto,
                                                    }),
                                                },
                                            },
                                        );
                                    }
                                    ContentPart::InputAudio { input_audio } => {
                                        openai_parts.push(ChatCompletionRequestMessageContentPart::InputAudio {
                                            data: input_audio.data,
                                            format: format!("{:?}", input_audio.format),
                                        });
                                    }
                                    ContentPart::InputVideo { input_video } => {
                                        anyhow::bail!("OpenAI provider does not support video input: {}", input_video.data);
                                    }
                                    ContentPart::Document { document } => {
                                        anyhow::bail!("OpenAI provider does not support document input: {:?}", document.file_type);
                                    }
                                }
                            }
                            ChatCompletionRequestMessageContent::Array(openai_parts)
                        }
                    };
                    Ok(ChatCompletionRequestMessage::User {
                        content,
                        name: u.name,
                    })
                }
                ChatMessage::Assistant(a) => {
                    let content = a.content.map(|c| match c {
                        MessageContent::Text(text) => ChatCompletionRequestMessageContent::Text(text),
                        MessageContent::Parts(parts) => {
                            let text = parts
                                .into_iter()
                                .filter_map(|part| match part {
                                    ContentPart::Text { text } => Some(text),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                                .join(" ");
                            ChatCompletionRequestMessageContent::Text(text)
                        }
                    });

                    let tool_calls = a.tool_calls.map(|calls| {
                        calls.into_iter()
                            .map(|call| ChatCompletionMessageToolCall {
                                id: call.id,
                                tool_type: "function".to_string(),
                                function: ChatCompletionMessageToolCallFunction {
                                    name: call.function.name,
                                    arguments: call.function.arguments,
                                },
                            })
                            .collect()
                    });

                    Ok(ChatCompletionRequestMessage::Assistant {
                        content,
                        name: a.name,
                        tool_calls,
                        refusal: a.refusal,
                        reasoning: None,
                    })
                }
                ChatMessage::Tool(t) => {
                    let content = match t.content {
                        MessageContent::Text(text) => text,
                        MessageContent::Parts(parts) => parts
                            .into_iter()
                            .filter_map(|part| match part {
                                ContentPart::Text { text } => Some(text),
                                _ => None,
                            })
                            .collect::<Vec<_>>()
                            .join(" "),
                    };
                    Ok(ChatCompletionRequestMessage::Tool {
                        tool_call_id: t.tool_call_id,
                        content,
                    })
                }
            }
        }).collect::<Result<Vec<_>>>()?;

        let tools = req.tools.map(|tools| {
            tools
                .into_iter()
                .map(|tool| ChatCompletionTool {
                    tool_type: "function".to_string(),
                    function: ChatCompletionToolFunction {
                        name: tool.function.name,
                        description: tool.function.description,
                        parameters: tool.function.parameters,
                        strict: tool.function.strict,
                    },
                })
                .collect()
        });

        let tool_choice = req.tool_choice.map(|choice| match choice {
            TypesToolChoice::Auto(s) => ChatCompletionToolChoiceOption::String(s),
            TypesToolChoice::Named(obj) => ChatCompletionToolChoiceOption::Object(ChatCompletionNamedToolChoice {
                tool_type: "function".to_string(),
                function: ChatCompletionNamedToolChoiceFunction {
                    name: obj.function.name,
                },
            }),
        });

        let response_format = req.response_format.map(|format| match format {
            TypesResponseFormat::Text => ResponseFormat::Text,
            TypesResponseFormat::JsonObject => ResponseFormat::JsonObject,
            TypesResponseFormat::JsonSchema { json_schema } => ResponseFormat::JsonSchema {
                json_schema: JsonSchemaFormat {
                    name: json_schema.name,
                    description: json_schema.description,
                    schema: json_schema.schema,
                    strict: json_schema.strict,
                },
            },
        });

        let stop = req.stop.map(|stop| match stop {
            TypesStop::Single(s) => StopConfiguration::Single(s),
            TypesStop::Multiple(arr) => StopConfiguration::Multiple(arr),
        });

        Ok(CreateChatCompletionRequest {
            messages,
            model: req.model.unwrap_or_default(),
            modalities: None,
            reasoning_effort: None,
            max_completion_tokens: req.max_tokens.map(|t| t as i32),
            frequency_penalty: req.frequency_penalty,
            presence_penalty: req.presence_penalty,
            web_search_options: None,
            top_logprobs: req.top_logprobs.map(|t| t as i32),
            response_format,
            audio: None,
            store: None,
            stream: req.stream,
            stop,
            logit_bias: req.logit_bias.map(|bias| bias.into_iter().map(|(k, v)| (k, v as i32)).collect()),
            logprobs: req.logprobs,
            max_tokens: req.max_tokens.map(|t| t as i32),
            n: req.n.map(|n| n as i32),
            prediction: None,
            seed: req.seed.map(|s| s as i64),
            stream_options: None,
            tools,
            tool_choice,
            parallel_tool_calls: req.parallel_tool_calls,
            function_call: None,
            functions: None,
            temperature: req.temperature,
            top_p: req.top_p,
            user: req.user,
            session_id: None,
        })
    }
}
