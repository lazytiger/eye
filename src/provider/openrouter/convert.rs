//! TryFrom trait implementations for type conversion
//!
//! This module provides conversions between OpenRouter API types and provider-agnostic types.
//! Conversions that may fail use TryFrom and return anyhow::Error.

use super::chat_types::*;
use super::embedding_types::*;
use crate::provider::types::{
    AssistantMessage as TypesAssistantMessage, AudioFormat as TypesAudioFormat,
    ChatChoice as TypesChatChoice, ChatMessage as TypesChatMessage,
    ChatRequest as TypesChatRequest, ChatResponse as TypesChatResponse,
    CompletionTokensDetails as TypesCompletionTokensDetails, ContentPart as TypesContentPart,
    EmbeddingEncodingFormat as TypesEmbeddingEncodingFormat,
    EmbeddingInput as TypesEmbeddingInput, EmbeddingRequest as TypesEmbeddingRequest,
    EmbeddingResponse as TypesEmbeddingResponse, EmbeddingUsage as TypesEmbeddingUsage,
    FinishReason, FunctionCall as TypesFunctionCall, ImageDetail as TypesImageDetail,
    Logprobs as TypesLogprobs, MessageContent as TypesMessageContent,
    PromptTokensDetails as TypesPromptTokensDetails, Stop as TypesStop,
    TokenLogprob as TypesTokenLogprob, TopLogprob as TypesTopLogprob,
    ToolCall as TypesToolCall, ToolChoice as TypesToolChoice, ToolType as TypesToolType,
    Usage as TypesUsage,
};
use anyhow::Result;
use std::convert::TryFrom;

// Convert from types::ChatRequest to openrouter::ChatRequest
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
                    Ok(ChatMessage::System(SystemMessage {
                        content: MessageContent::Text(content),
                        name: s.name,
                    }))
                }
                TypesChatMessage::User(u) => {
                    let content = match u.content {
                        TypesMessageContent::Text(text) => MessageContent::Text(text),
                        TypesMessageContent::Parts(parts) => {
                            let content_parts: Result<Vec<_>> = parts
                                .into_iter()
                                .map(|part| match part {
                                    TypesContentPart::Text { text } => {
                                        Ok(ContentPart::Text { text, cache_control: None })
                                    }
                                    TypesContentPart::ImageUrl { image_url } => {
                                        Ok(ContentPart::ImageUrl {
                                            image_url: ImageUrl {
                                                url: image_url.url,
                                                detail: image_url.detail.map(|d| match d {
                                                    TypesImageDetail::Low => ImageDetail::Low,
                                                    TypesImageDetail::High => ImageDetail::High,
                                                    TypesImageDetail::Auto => ImageDetail::Auto,
                                                }),
                                            },
                                        })
                                    }
                                    TypesContentPart::InputAudio { input_audio } => {
                                        Ok(ContentPart::InputAudio {
                                            input_audio: InputAudio {
                                                data: input_audio.data,
                                                format: match input_audio.format {
                                                    TypesAudioFormat::Wav => AudioFormat::Wav,
                                                    TypesAudioFormat::Mp3 => AudioFormat::Mp3,
                                                    TypesAudioFormat::Flac => AudioFormat::Flac,
                                                    TypesAudioFormat::Opus => AudioFormat::Opus,
                                                    TypesAudioFormat::Pcm16 => AudioFormat::Pcm16,
                                                },
                                            },
                                        })
                                    }
                                    TypesContentPart::InputVideo { input_video } => {
                                        Ok(ContentPart::InputVideo {
                                            input_video: InputVideo {
                                                url: input_video.data,
                                            },
                                        })
                                    }
                                    TypesContentPart::Document { document } => {
                                        Ok(ContentPart::Document {
                                            document: Document {
                                                file_data: Some(document.data),
                                                file_id: None,
                                                filename: None,
                                            },
                                        })
                                    }
                                })
                                .collect();
                            MessageContent::Parts(content_parts?)
                        }
                    };
                    Ok(ChatMessage::User(UserMessage { content, name: u.name }))
                }
                TypesChatMessage::Assistant(a) => {
                    let content = a.content.map(|c| match c {
                        TypesMessageContent::Text(text) => MessageContent::Text(text),
                        TypesMessageContent::Parts(parts) => {
                            let text = parts
                                .into_iter()
                                .filter_map(|part| match part {
                                    TypesContentPart::Text { text } => Some(text),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                                .join(" ");
                            MessageContent::Text(text)
                        }
                    });
                    let tool_calls = a.tool_calls.map(|calls| {
                        calls.into_iter()
                            .map(|call| ToolCall {
                                id: call.id,
                                type_: "function".to_string(),
                                function: FunctionCall {
                                    name: call.function.name,
                                    arguments: call.function.arguments,
                                },
                            })
                            .collect()
                    });
                    Ok(ChatMessage::Assistant(AssistantMessage {
                        content,
                        name: a.name,
                        tool_calls,
                        refusal: a.refusal,
                    }))
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
                    Ok(ChatMessage::Tool(ToolMessage {
                        content: MessageContent::Text(content),
                        tool_call_id: t.tool_call_id,
                    }))
                }
            })
            .collect::<Result<Vec<_>>>()?;

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

        let tool_choice = req.tool_choice.map(|choice| match choice {
            TypesToolChoice::Auto(s) => ToolChoice::String(s),
            TypesToolChoice::Named(obj) => ToolChoice::Object(ToolChoiceObject {
                type_: "function".to_string(),
                function: ToolChoiceFunction { name: obj.function.name },
            }),
        });

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

        let stop = req.stop.map(|stop| match stop {
            TypesStop::Single(s) => Stop::String(s),
            TypesStop::Multiple(arr) => Stop::Array(arr),
        });

        Ok(ChatRequest {
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
        })
    }
}

// Convert from openrouter::ChatResponse to types::ChatResponse
impl From<ChatResponse> for TypesChatResponse {
    fn from(resp: ChatResponse) -> Self {
        let choices = resp.choices.into_iter().map(|choice| {
            let message = {
                let content = choice.message.content.map(|c| match c {
                    MessageContent::Text(text) => TypesMessageContent::Text(text),
                    MessageContent::Parts(parts) => {
                        let text = parts
                            .into_iter()
                            .filter_map(|part| match part {
                                ContentPart::Text { text, .. } => Some(text),
                                _ => None,
                            })
                            .collect::<Vec<_>>()
                            .join(" ");
                        TypesMessageContent::Text(text)
                    }
                });
                let tool_calls = choice.message.tool_calls.map(|calls| {
                    calls.into_iter()
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
                    name: choice.message.name,
                    tool_calls,
                    refusal: choice.message.refusal,
                }
            };

            let finish_reason = match choice.finish_reason {
                Some(ChatCompletionFinishReason::ToolCalls) => FinishReason::ToolCalls,
                Some(ChatCompletionFinishReason::Stop) => FinishReason::Stop,
                Some(ChatCompletionFinishReason::Length) => FinishReason::Length,
                Some(ChatCompletionFinishReason::ContentFilter) => FinishReason::ContentFilter,
                Some(ChatCompletionFinishReason::Error) => FinishReason::Error,
                None => FinishReason::Stop,
            };

            let logprobs = choice.logprobs.map(|lp| TypesLogprobs {
                content: lp.content.map(|content| {
                    content
                        .into_iter()
                        .map(|c| TypesTokenLogprob {
                            token: c.token,
                            logprob: c.logprob,
                            bytes: c.bytes,
                            top_logprobs: c.top_logprobs.map(|top| {
                                top.into_iter()
                                    .map(|t| TypesTopLogprob {
                                        token: t.token,
                                        logprob: t.logprob,
                                        bytes: t.bytes,
                                    })
                                    .collect()
                            }),
                        })
                        .collect()
                }),
            });

            TypesChatChoice {
                index: choice.index,
                message,
                finish_reason,
                logprobs,
            }
        }).collect();

        let usage = TypesUsage {
            prompt_tokens: resp.usage.prompt_tokens,
            completion_tokens: resp.usage.completion_tokens,
            total_tokens: resp.usage.total_tokens,
            prompt_tokens_details: resp.usage.prompt_tokens_details.map(|d| TypesPromptTokensDetails {
                cached_tokens: d.cached_tokens,
                audio_tokens: d.audio_tokens,
            }),
            completion_tokens_details: resp.usage.completion_tokens_details.map(|d| TypesCompletionTokensDetails {
                reasoning_tokens: d.reasoning_tokens,
                audio_tokens: d.audio_tokens,
            }),
        };

        TypesChatResponse {
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
impl TryFrom<TypesEmbeddingRequest> for EmbeddingsRequest {
    type Error = anyhow::Error;

    fn try_from(req: TypesEmbeddingRequest) -> Result<Self> {
        let input = match req.input {
            TypesEmbeddingInput::String(s) => EmbeddingInput::String(s),
            TypesEmbeddingInput::StringArray(arr) => EmbeddingInput::StringArray(arr),
        };

        let encoding_format = req.encoding_format.map(|fmt| match fmt {
            TypesEmbeddingEncodingFormat::Float => EmbeddingEncodingFormat::Float,
            TypesEmbeddingEncodingFormat::Base64 => EmbeddingEncodingFormat::Base64,
        });

        Ok(EmbeddingsRequest {
            input,
            model: req.model,
            encoding_format,
            dimensions: req.dimensions,
            user: req.user,
            provider: None,
            input_type: None,
        })
    }
}

// Convert from openrouter::EmbeddingsResponse to types::EmbeddingResponse
impl From<EmbeddingsResponse> for TypesEmbeddingResponse {
    fn from(resp: EmbeddingsResponse) -> Self {
        let data = resp.data.into_iter().map(|embedding| {
            let embedding_data = match embedding.embedding {
                EmbeddingData::Array(vec) => vec,
                EmbeddingData::Base64(_) => Vec::new(),
            };
            crate::provider::types::EmbeddingObject {
                index: embedding.index,
                embedding: embedding_data,
                object: embedding.object,
            }
        }).collect();

        let usage = TypesEmbeddingUsage {
            prompt_tokens: resp.usage.prompt_tokens,
            total_tokens: resp.usage.total_tokens,
        };

        TypesEmbeddingResponse {
            object: resp.object,
            data,
            model: resp.model,
            usage,
        }
    }
}
