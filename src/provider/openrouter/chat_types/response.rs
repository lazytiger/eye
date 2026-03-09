//! OpenRouter API response types
//!
//! This module contains response type definitions for OpenRouter chat completion.

use serde::{Deserialize, Serialize};

use super::messages::AssistantMessage;

/// Chat completion response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatResponse {
    pub id: String,
    pub choices: Vec<ChatChoice>,
    pub created: u64,
    pub model: String,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
    pub usage: ChatGenerationTokenUsage,
}

/// Chat choice
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatChoice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<ChatCompletionFinishReason>,
    pub index: u32,
    pub message: AssistantMessage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<ChatMessageTokenLogprobs>,
}

/// Chat completion finish reason
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ChatCompletionFinishReason {
    ToolCalls,
    Stop,
    Length,
    ContentFilter,
    Error,
}

/// Chat generation token usage
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatGenerationTokenUsage {
    pub completion_tokens: u32,
    pub prompt_tokens: u32,
    pub total_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens_details: Option<CompletionTokensDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens_details: Option<PromptTokensDetails>,
}

/// Completion tokens details
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionTokensDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_prediction_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_prediction_tokens: Option<u32>,
}

/// Prompt tokens details
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromptTokensDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_write_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_tokens: Option<u32>,
}

/// Chat message token logprobs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessageTokenLogprobs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TokenLogprob>>,
}

/// Token logprob
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenLogprob {
    pub token: String,
    pub logprob: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<Vec<TopLogprob>>,
}

/// Top logprob
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopLogprob {
    pub token: String,
    pub logprob: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}
