//! Model provider trait definition
//!
//! Defines the ModelProvider trait to abstract different LLM providers

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

/// Chat message
#[derive(Debug, Clone)]
pub struct ChatMessage {
    /// Message role
    pub role: MessageRole,
    /// Message content
    pub content: String,
    /// Tool calls (if any)
    pub tool_calls: Option<Vec<ToolCall>>,
}

/// Message role
#[derive(Debug, Clone, PartialEq)]
pub enum MessageRole {
    /// System message
    System,
    /// User message
    User,
    /// Assistant message
    Assistant,
    /// Tool message
    Tool,
}

/// Tool call
#[derive(Debug, Clone)]
pub struct ToolCall {
    /// Tool call ID
    pub id: String,
    /// Tool name
    pub name: String,
    /// Tool arguments
    pub arguments: Value,
}

/// Tool definition
#[derive(Debug, Clone)]
pub struct ToolDefinition {
    /// Tool name
    pub name: String,
    /// Tool description
    pub description: String,
    /// Tool parameter definition
    pub parameters: Value,
}

/// Chat completion request
#[derive(Debug, Clone)]
pub struct ChatCompletionRequest {
    /// Messages
    pub messages: Vec<ChatMessage>,
    /// Tool definitions
    pub tools: Option<Vec<ToolDefinition>>,
    /// Temperature
    pub temperature: Option<f32>,
    /// Max tokens
    pub max_tokens: Option<u32>,
    /// Stream output
    pub stream: bool,
}

/// Chat completion response
#[derive(Debug, Clone)]
pub struct ChatCompletionResponse {
    /// Response message
    pub message: ChatMessage,
    /// Token usage
    pub usage: Option<Usage>,
}

/// Token usage
#[derive(Debug, Clone)]
pub struct Usage {
    /// Prompt token count
    pub prompt_tokens: u32,
    /// Completion token count
    pub completion_tokens: u32,
    /// Total token count
    pub total_tokens: u32,
}

/// Model provider trait
#[async_trait]
pub trait ModelProvider: Send + Sync {
    /// Send chat completion request
    async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse>;

    /// Get provider name
    fn name(&self) -> &str;

    /// Get supported provider list
    fn supported_models(&self) -> Vec<String>;

    /// Validate configuration
    fn validate_config(&self) -> Result<()>;
}
