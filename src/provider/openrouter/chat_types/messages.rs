//! OpenRouter API message types
//!
//! This module contains message type definitions for OpenRouter chat completion.

use serde::{Deserialize, Serialize};

use super::tools::ToolCall;

/// Chat message
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum ChatMessage {
    System(SystemMessage),
    User(UserMessage),
    Assistant(AssistantMessage),
    Tool(ToolMessage),
    Developer(DeveloperMessage),
}

/// System message
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemMessage {
    pub content: MessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// User message
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserMessage {
    pub content: MessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Assistant message
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

/// Tool message
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolMessage {
    pub content: MessageContent,
    pub tool_call_id: String,
}

/// Developer message
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeveloperMessage {
    pub content: MessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Message content
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    Parts(Vec<ContentPart>),
}

/// Content part
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    },
    ImageUrl {
        image_url: ImageUrl,
    },
    InputAudio {
        input_audio: InputAudio,
    },
    InputVideo {
        input_video: InputVideo,
    },
    Document {
        document: Document,
    },
}

/// Cache control for content
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CacheControl {
    #[serde(rename = "type")]
    pub cache_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
}

/// Image URL
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<ImageDetail>,
}

/// Image detail level
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ImageDetail {
    Low,
    High,
    Auto,
}

/// Audio input information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputAudio {
    /// Base64 encoded audio data or URL
    pub data: String,
    /// Audio format
    pub format: AudioFormat,
}

/// Audio format
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AudioFormat {
    Wav,
    Mp3,
    Flac,
    Opus,
    Pcm16,
}

/// Video input information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputVideo {
    /// Video URL (supports data: URIs and HTTP URLs)
    pub url: String,
}

/// Document information (for PDF, etc.)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Document {
    /// File content as base64 data URL or HTTP URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<String>,
    /// File ID for previously uploaded files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// Original filename
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}
